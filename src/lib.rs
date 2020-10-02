//! ress
//! A crate for parsing raw JS into a token stream
//!
//! The primary interfaces are the function [`tokenize`][tokenize] and
//! the struct [`Scanner`][scanner]. The [`Scanner`][scanner] struct impls [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
//! and the [`tokenize`][tokenize] function is just a wrapper
//! around [`Scanner::collect()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect).
//!
//! The `Scanner` will provide a stream of `Item`s, and `Item` is
//! has 3 properties a [`Token`][token], a [`Span`][span], and a [`SourceLocation`][location]. The `Span` is a
//! representation of where the `Item` exists in the original source while the `Token`
//! provides details about what JavaScript token it represents.
//!
//! [token]: ./enum.Token
//! [span]: ./struct.Span
//! [scanner]: ./struct.Scanner
//! [tokenize]: ../fn.tokenize
//! [location]: ./struct.SourceLocation

#[macro_use]
extern crate log;

pub mod error;
mod manual_scanner;
mod tokenizer;
pub mod tokens;
pub use crate::tokenizer::Tokenizer;

pub mod prelude {
    pub use super::tokenize;
    pub use super::tokens::prelude::*;
    pub use super::Item;
    pub use super::OpenCurlyKind;
    pub use super::Position;
    pub use super::Scanner;
    pub use super::ScannerState;
    pub use super::SourceLocation;
}
use crate::tokenizer::RawKeyword;
use crate::tokens::prelude::*;
use error::{Error, RawError};
pub use manual_scanner::{ManualScanner, ScannerState as ManualState};

type Res<T> = Result<T, Error>;
mod look_behind;

use look_behind::{Brace, LookBehind, MetaToken, Paren};

/// a convince function for collecting a scanner into
/// a `Vec<Token>`
pub fn tokenize(text: &str) -> Res<Vec<Token<&str>>> {
    let mut ret = Vec::new();
    for i in Scanner::new(text) {
        let inner = i?.token;
        ret.push(inner);
    }
    Ok(ret)
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// The start and end position of a token
/// including the line/column number
pub struct SourceLocation {
    pub start: Position,
    pub end: Position,
}

impl SourceLocation {
    #[inline]
    pub const fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// A single character position in the
/// file including the line/column number
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl ::std::fmt::Display for Position {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
impl ::std::cmp::PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match self.line.cmp(&other.line) {
            Less => Less,
            Greater => Greater,
            _ => self.column.cmp(&other.column),
        }
    }
}

impl Position {
    #[inline]
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// The start and end of a token as the byte
/// index in the original text
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    /// Create a new Span from its parts
    #[inline]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    pub const fn len(self) -> usize {
        self.end - self.start
    }
}

#[derive(Clone, Debug, PartialEq)]
/// A single token with additional metadata
///
pub struct Item<T> {
    pub token: T,
    pub span: Span,
    pub location: SourceLocation,
}

impl<T> Item<T>
where
    T: TokenExt,
{
    pub fn new(token: T, span: Span, location: SourceLocation) -> Self {
        Self {
            token,
            span,
            location,
        }
    }
    fn new_(
        token: T,
        span_start: usize,
        span_end: usize,
        loc_start_line: usize,
        loc_start_col: usize,
        loc_end_line: usize,
        loc_end_col: usize,
    ) -> Self {
        Self {
            token,
            span: Span::new(span_start, span_end),
            location: SourceLocation::new(
                Position::new(loc_start_line, loc_start_col),
                Position::new(loc_end_line, loc_end_col),
            ),
        }
    }
    pub fn is_string(&self) -> bool {
        self.token.is_string()
    }
    pub fn is_eof(&self) -> bool {
        self.token.is_eof()
    }
    pub fn is_template(&self) -> bool {
        self.token.is_template_head()
            || self.token.is_template_body()
            || self.token.is_template_tail()
    }
}
/// The primary interface of this crate used
/// to tokenize any JS text into a stream of
/// `Item`s.
pub struct Scanner<'a> {
    manual_scanner: ManualScanner<'a>,
    original: &'a str,
    errored: bool,
    last_three: LookBehind,
    brace_stack: Vec<Brace>,
    paren_stack: Vec<Paren>,
}

impl<'a> Scanner<'a> {
    /// Create a new `Scanner` by providing the
    /// JS text
    pub fn new(text: &'a str) -> Self {
        Self {
            manual_scanner: ManualScanner::new(text),
            original: text,
            errored: false,
            last_three: LookBehind::new(),
            paren_stack: Vec::new(),
            brace_stack: Vec::new(),
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Res<Item<Token<&'a str>>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.get_next_token(true)
    }
}

impl<'b> Scanner<'b> {
    /// Attempts to look ahead 1 token
    ///
    /// Similar to how `Peekable::peek` works however the
    /// returned value will not be a borrowed `Item`. Since
    /// there isn't a borrow happening this essentially duplicates
    /// the cost of calling `next`.
    pub fn look_ahead(&mut self) -> Option<Res<Item<Token<&'b str>>>> {
        self.get_next_token(false)
    }
    /// Skip any upcoming comments to get the
    /// next valid js token
    pub fn skip_comments(&mut self) -> Res<()> {
        debug!(target: "ress", "skipping comments");
        self.manual_scanner.skip_comments()
    }
    /// Get a copy of the scanner's current state
    pub fn get_state(&self) -> ScannerState {
        ScannerState {
            manual_state: self.manual_scanner.get_state(),
            last_three: self.last_three.clone(),
            paren_stack: self.paren_stack.clone(),
        }
    }
    /// Set the scanner's current state to the state provided
    #[inline]
    pub fn set_state(&mut self, state: ScannerState) {
        let ScannerState {
            manual_state,
            last_three,
            paren_stack,
        } = state;
        self.last_three = last_three;
        self.paren_stack = paren_stack;
        self.manual_scanner.set_state(manual_state);
    }
    #[inline]
    /// The implementation of `Scanner::next` that includes
    /// the flag for advancing, meaning the `look_ahead` method
    /// can also use this implementation
    fn get_next_token(&mut self, advance_cursor: bool) -> Option<Res<Item<Token<&'b str>>>> {
        if self.errored {
            return None;
        }
        if self.manual_scanner.eof {
            debug!("end of iterator, returning None");
            return None;
        };
        let state = self.manual_scanner.get_state();
        let next = match self.manual_scanner.next_token()? {
            Ok(n) => n,
            Err(e) => {
                self.errored = true;
                return Some(Err(e));
            }
        };

        let ret = if next.token.is_div_punct() && self.is_regex_start() {
            self.manual_scanner.next_regex(next.span.len())?
        } else {
            Ok(next)
        };
        if advance_cursor {
            if let Ok(i) = &ret {
                if let Err(e) = self.keep_books(&i) {
                    return Some(Err(e));
                }
            }
        } else {
            self.manual_scanner.set_state(state);
        }
        Some(ret)
    }
    #[inline]
    /// Evaluate the token for possible regex
    /// start and handle updating the
    /// `self.last_three`, `self.paren_stack` and `self.brace_stack`
    fn keep_books(&mut self, item: &Item<Token<&'b str>>) -> Res<()> {
        if let Token::Punct(ref p) = &item.token {
            match p {
                Punct::OpenParen => self.handle_open_paren_books(),
                Punct::OpenBrace => self.handle_open_brace_books(),
                Punct::CloseParen => self.handle_close_paren_books(item.span.start)?,
                Punct::CloseBrace => self.handle_close_brace_books(item.span.start)?,
                _ => self
                    .last_three
                    .push((&item.token, self.manual_scanner.new_line_count as u32).into()),
            }
        } else if !item.token.is_comment() {
            self.last_three
                .push((&item.token, self.manual_scanner.new_line_count as u32).into());
        }
        Ok(())
    }
    #[inline]
    /// Handle the book keeping when we find
    /// an `(`
    fn handle_open_paren_books(&mut self) {
        let func_expr = if let Some(MetaToken::Keyword(RawKeyword::Function, _)) =
            self.last_three.one()
        {
            if let Some(tok) = self.last_three.two() {
                !Self::check_for_expression(*tok)
            } else {
                false
            }
        } else if let Some(MetaToken::Keyword(RawKeyword::Function, _)) = self.last_three.two() {
            if let Some(tok) = self.last_three.three() {
                Self::check_for_expression(*tok)
            } else {
                false
            }
        } else {
            false
        };
        let conditional = if let Some(tok) = self.last_three.one() {
            Self::check_token_for_conditional(*tok)
        } else {
            false
        };
        let paren = Paren {
            func_expr,
            conditional,
        };
        let meta = MetaToken::OpenParen(paren);
        self.paren_stack.push(paren);
        self.last_three.push(meta);
    }
    #[inline]
    /// Handle the book keeping when we find
    /// and `{`
    fn handle_open_brace_books(&mut self) {
        let is_block = if let Some(last) = self.last_three.one() {
            match last {
                MetaToken::Punct(Punct::OpenParen)
                | MetaToken::Punct(Punct::OpenBracket)
                | MetaToken::OpenParen(_)
                | MetaToken::OpenBrace(_, _) => false,
                MetaToken::Punct(Punct::Colon) => {
                    if let Some(parent) = self.brace_stack.last() {
                        parent.is_block
                    } else {
                        false
                    }
                }
                MetaToken::Punct(_) => !Self::is_op(*last),
                MetaToken::Keyword(RawKeyword::Return, line)
                | MetaToken::Keyword(RawKeyword::Yield, line) => {
                    if let Some(last) = self.last_three.two() {
                        last.line_number() != *line
                    } else {
                        false
                    }
                }
                MetaToken::Keyword(RawKeyword::Case, _) => false,
                MetaToken::Keyword(_, _) => !Self::is_op(*last),
                _ => true,
            }
        } else {
            true
        };
        let paren = if let Some(MetaToken::CloseParen(open)) = self.last_three.one() {
            Some(*open)
        } else {
            None
        };
        let brace = look_behind::Brace { is_block, paren };
        self.brace_stack.push(brace);
        self.last_three.push(MetaToken::OpenBrace(
            brace,
            self.manual_scanner.new_line_count as u32,
        ));
    }
    #[inline]
    /// Handle the book keeping when we find a `(`
    fn handle_close_paren_books(&mut self, start: usize) -> Res<()> {
        let paren = if let Some(paren) = self.paren_stack.pop() {
            paren
        } else {
            self.errored = true;
            return self.error(RawError {
                idx: start,
                msg: "Unmatched open close paren".to_string(),
            });
        };
        self.last_three.push(MetaToken::CloseParen(paren));
        Ok(())
    }
    #[inline]
    /// Handle the book keeping when we find a `{`
    fn handle_close_brace_books(&mut self, start: usize) -> Res<()> {
        if let Some(open) = self.brace_stack.pop() {
            let close = MetaToken::CloseBrace(open);
            self.last_three.push(close);
            Ok(())
        } else {
            self.error(RawError {
                idx: start,
                msg: "unmatched close brace".to_string(),
            })
        }
    }
    /// Detect if the `/` is the beginning of
    /// a regex or is division
    ///
    /// [see this for more details](https://github.com/sweet-js/sweet-core/wiki/design)
    fn is_regex_start(&self) -> bool {
        if let Some(ref last_token) = self.last_three.one() {
            match last_token {
                MetaToken::Keyword(k, _) => match k {
                    RawKeyword::This => false,
                    _ => true,
                },
                MetaToken::Punct(p) => match p {
                    Punct::CloseBracket => false,
                    _ => true,
                },
                MetaToken::CloseParen(open) => open.conditional,
                MetaToken::CloseBrace(close) => {
                    if close.is_block {
                        if let Some(open) = &close.paren {
                            !open.func_expr
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                }
                MetaToken::OpenParen(_) | MetaToken::OpenBrace(_, _) => true,
                _ => false,
            }
        } else {
            true
        }
    }
    /// Check a token for the conditional keywords
    ///
    /// > used in determining if we are at a regex or not
    fn check_token_for_conditional(tok: MetaToken) -> bool {
        if let MetaToken::Keyword(k, _) = tok {
            match k {
                RawKeyword::If | RawKeyword::For | RawKeyword::While | RawKeyword::With => true,
                _ => false,
            }
        } else {
            false
        }
    }
    /// Check if a token indicates beginning of a
    /// function expression
    ///
    /// > used in determining if we are at a regex or not
    fn check_for_expression(token: MetaToken) -> bool {
        if Self::is_op(token) {
            true
        } else {
            match token {
                MetaToken::Keyword(RawKeyword::Return, _)
                | MetaToken::Keyword(RawKeyword::Case, _) => true,
                _ => false,
            }
        }
    }
    /// Determine if a token is a punctuation or keyword
    /// that indicates an operation
    ///
    /// > used in determining if we are at a regex or not
    fn is_op(tok: MetaToken) -> bool {
        match tok {
            MetaToken::Punct(ref p) => match p {
                Punct::Equal
                | Punct::PlusEqual
                | Punct::DashEqual
                | Punct::AsteriskEqual
                | Punct::ForwardSlashEqual
                | Punct::PercentEqual
                | Punct::DoubleLessThanEqual
                | Punct::DoubleGreaterThanEqual
                | Punct::TripleGreaterThanEqual
                | Punct::AmpersandEqual
                | Punct::PipeEqual
                | Punct::CaretEqual
                | Punct::Comma
                | Punct::Plus
                | Punct::Dash
                | Punct::Asterisk
                | Punct::ForwardSlash
                | Punct::Percent
                | Punct::DoubleLessThan
                | Punct::DoubleGreaterThan
                | Punct::TripleGreaterThan
                | Punct::Ampersand
                | Punct::Pipe
                | Punct::Caret
                | Punct::DoubleAmpersand
                | Punct::DoublePipe
                | Punct::QuestionMark
                | Punct::Colon
                | Punct::TripleEqual
                | Punct::DoubleEqual
                | Punct::GreaterThanEqual
                | Punct::LessThanEqual
                | Punct::LessThan
                | Punct::GreaterThan
                | Punct::BangEqual
                | Punct::BangDoubleEqual
                | Punct::DoublePlus
                | Punct::DoubleDash
                | Punct::Tilde
                | Punct::Bang => true,
                _ => false,
            },
            MetaToken::Keyword(k, _) => match k {
                RawKeyword::InstanceOf
                | RawKeyword::In
                | RawKeyword::Delete
                | RawKeyword::Void
                | RawKeyword::TypeOf
                | RawKeyword::Throw
                | RawKeyword::New => true,
                _ => false,
            },
            _ => false,
        }
    }
    /// Get a string for any given span
    pub fn string_for(&self, span: &Span) -> Option<String> {
        self.manual_scanner.string_for(span)
    }
    /// Get a &str for any given span
    pub fn str_for(&self, span: &Span) -> Option<&'b str> {
        self.manual_scanner.str_for(span)
    }
    /// Get the line/column pair for any given byte index
    pub fn position_for(&self, idx: usize) -> (usize, usize) {
        let mut line_ct = 1;
        // This is the byte position, not the character
        // position to account for multi byte chars
        let mut byte_position = 0;
        // loop over the characters
        for (i, c) in self.original.chars().enumerate() {
            if i >= idx {
                return (line_ct, byte_position);
            }
            match c {
                '\r' => {
                    // look ahead 1 char to see if it is a newline pair
                    // if so, don't include it, it will get included in the next
                    // iteration
                    if let Some(next) = self.original.get(byte_position..byte_position + 2) {
                        if next != "\r\n" {
                            line_ct += 1;
                            byte_position = 0;
                        }
                    }
                }
                '\n' | '\u{2028}' | '\u{2029}' => {
                    line_ct += 1;
                    byte_position = 0;
                }
                _ => byte_position += c.len_utf8(),
            };
        }
        (line_ct, byte_position)
    }

    /// Helper to handle the error cases
    fn error<T>(&self, raw_error: RawError) -> Res<T> {
        let RawError { idx, msg } = raw_error;
        let (line, column) = self.position_for(idx);
        Err(Error { line, column, msg })
    }
}

#[inline]
fn is_line_term(c: char) -> bool {
    c == '\n' || c == '\r' || c == '\u{2028}' || c == '\u{2029}'
}

#[derive(Clone, Copy, PartialEq, Debug)]
/// For keeping track of the nested-ness of
/// templates and blocks
pub enum OpenCurlyKind {
    Template,
    Block,
}

#[derive(Clone)]
/// All of the important state
/// for the scanner, used to
/// cache and reset a `Scanner`
pub struct ScannerState {
    pub manual_state: ManualState,
    pub last_three: LookBehind,
    pub paren_stack: Vec<Paren>,
}

#[cfg(test)]
mod test {
    use super::tokens::*;
    use super::*;
    #[test]
    fn tokenizer() {
        let js = "#!/usr/bin/env node
'use strict';
function thing() {
    let x = 0;
    console.log('stuff');
}";
        let expectation = vec![
            Token::Comment(Comment {
                kind: tokens::CommentKind::Hashbang,
                content: "/usr/bin/env node",
                tail_content: None,
            }),
            Token::String(StringLit::single("use strict", false)),
            Token::Punct(Punct::SemiColon),
            Token::Keyword(Keyword::Function("function".into())),
            Token::Ident("thing".into()),
            Token::Punct(Punct::OpenParen),
            Token::Punct(Punct::CloseParen),
            Token::Punct(Punct::OpenBrace),
            Token::Keyword(Keyword::Let("let".into())),
            Token::Ident("x".into()),
            Token::Punct(Punct::Equal),
            Token::Number("0".into()),
            Token::Punct(Punct::SemiColon),
            Token::Ident("console".into()),
            Token::Punct(Punct::Period),
            Token::Ident("log".into()),
            Token::Punct(Punct::OpenParen),
            Token::String(StringLit::single("stuff", false)),
            Token::Punct(Punct::CloseParen),
            Token::Punct(Punct::SemiColon),
            Token::Punct(Punct::CloseBrace),
            Token::EoF,
        ];
        for (lhs, rhs) in Scanner::new(js).zip(expectation.into_iter()) {
            let lhs = lhs.unwrap();
            assert_eq!(lhs.token, rhs);
        }
    }

    #[test]
    fn tok_scanner() {
        let s = super::Scanner::new(
            "(function() {
this.x = 100;
this.y = 0;
})();",
        );
        let expected = vec![
            Token::Punct(Punct::OpenParen), //"("
            Token::Keyword(Keyword::Function("function".into())),
            Token::Punct(Punct::OpenParen),  //"("
            Token::Punct(Punct::CloseParen), //")"
            Token::Punct(Punct::OpenBrace),  //"{"
            Token::Keyword(Keyword::This("this".into())),
            Token::Punct(Punct::Period), //"."
            Token::Ident("x".into()),
            Token::Punct(Punct::Equal), //"="
            Token::Number("100".into()),
            Token::Punct(Punct::SemiColon), //";"
            Token::Keyword(Keyword::This("this".into())),
            Token::Punct(Punct::Period), //"."
            Token::Ident("y".into()),
            Token::Punct(Punct::Equal), //"="
            Token::Number("0".into()),
            Token::Punct(Punct::SemiColon),  //";"
            Token::Punct(Punct::CloseBrace), //"}"
            Token::Punct(Punct::CloseParen), //")"
            Token::Punct(Punct::OpenParen),  //"("
            Token::Punct(Punct::CloseParen), //")"
            Token::Punct(Punct::SemiColon),  //";"
            Token::EoF,
        ];
        validate(s, expected);
    }

    #[test]
    fn tok_scanner_jq() {
        let js = include_str!("../node_modules/jquery/dist/jquery.js");
        let t = Scanner::new(js);
        let _: Vec<_> = t.collect();
    }

    #[test]
    fn look_ahead() {
        let js = "function() { return; }";
        let mut s = Scanner::new(js);
        while let Some(peek) = s.look_ahead() {
            let peek = peek.unwrap();
            if let Some(next) = s.next() {
                let next = next.unwrap();
                assert_eq!(peek, next);
            }
        }
    }

    fn validate(s: Scanner, expected: Vec<Token<&str>>) {
        for (i, (lhs, rhs)) in s.zip(expected.into_iter()).enumerate() {
            let lhs = lhs.unwrap();
            println!("{:?}, {:?}", lhs.token, rhs);
            assert_eq!((i, lhs.token), (i, rhs));
        }
    }

    #[test]
    fn get_str() {
        let js = "function ( ) { return ; }";
        let mut s = Scanner::new(js);
        let strs = js.split(' ');
        for (i, p) in strs.enumerate() {
            let item = s.next().unwrap().unwrap();
            let q = s.string_for(&item.span).unwrap();
            assert_eq!((i, p.to_string()), (i, q))
        }
    }

    #[test]
    fn spans() {
        let js = include_str!("../node_modules/esprima/dist/esprima.js");
        let mut s = Scanner::new(js);
        while let Some(item) = s.next() {
            let item = item.unwrap();
            let from_stream = &js[item.span.start..item.span.end];
            if item.token.is_regex() {
                println!("{:?} - {:?}", from_stream, item.token);
            }
            let token = item.token.to_string();

            assert_eq!(
                from_stream, token,
                "token mismatch {:?} \n{}\n{}\n",
                item, from_stream, token
            );
        }
    }

    #[test]
    fn local_host_regex() {
        let js = r#"/^(http|https):\/\/(localhost|127\.0\.0\.1)/"#;
        let regex = RegEx::from_parts(r"^(http|https):\/\/(localhost|127\.0\.0\.1)", None);
        let mut s = Scanner::new(js);
        let r = s.next().unwrap().unwrap();
        assert_eq!(r.token, Token::RegEx(regex));
    }
    #[test]
    fn regex_replace() {
        let expect = vec![
            Token::Ident("ident".into()),
            Token::Punct(Punct::Period),
            Token::Ident("replace".into()),
            Token::Punct(Punct::OpenParen),
            Token::RegEx(RegEx::from_parts("%(\\d)", Some("g"))),
            Token::Punct(Punct::Comma),
            Token::String(StringLit::single("", false)),
            Token::Punct(Punct::CloseParen),
        ];
        let js = r#"ident.replace(/%(\d)/g, '')"#;
        let s = Scanner::new(js);
        for (i, (exp, item)) in expect.iter().zip(s).enumerate() {
            assert_eq!((i, exp), (i, &item.unwrap().token));
        }
    }

    #[test]
    fn error() {
        let js = "
(function() {
    let x = 'asdf
    ';
})()";
        for item in Scanner::new(js) {
            match item {
                Ok(_) => (),
                Err(e) => {
                    assert_eq!(e.line, 3);
                    assert_eq!(e.column, 17);
                }
            }
        }
    }

    #[test]
    fn locations() {
        let js = r"(function() {
    let x = 'asdf\
';
    let y = `asd
f`;
    /*
    * things
    */
})();";
        let expectation = vec![
            SourceLocation::new(Position::new(1, 1), Position::new(1, 2)), // 0 (
            SourceLocation::new(Position::new(1, 2), Position::new(1, 10)), // 1 function
            SourceLocation::new(Position::new(1, 10), Position::new(1, 11)), // 2 (
            SourceLocation::new(Position::new(1, 11), Position::new(1, 12)), // 3 )
            SourceLocation::new(Position::new(1, 13), Position::new(1, 14)), // 4 {
            SourceLocation::new(Position::new(2, 5), Position::new(2, 8)), // 5 let
            SourceLocation::new(Position::new(2, 9), Position::new(2, 10)), // 6 x
            SourceLocation::new(Position::new(2, 11), Position::new(2, 12)), // 7 =
            SourceLocation::new(Position::new(2, 13), Position::new(3, 1)), // 8 'asdf'
            SourceLocation::new(Position::new(3, 1), Position::new(3, 2)), // 9 ;
            SourceLocation::new(Position::new(4, 5), Position::new(4, 8)), // 10 let
            SourceLocation::new(Position::new(4, 9), Position::new(4, 10)), // 11 y
            SourceLocation::new(Position::new(4, 11), Position::new(4, 12)), // 12 =
            SourceLocation::new(Position::new(4, 13), Position::new(5, 2)), // 13 `asdf`
            SourceLocation::new(Position::new(5, 2), Position::new(5, 3)), // 14 ;
            SourceLocation::new(Position::new(6, 5), Position::new(8, 6)), // 15 comment
            SourceLocation::new(Position::new(9, 1), Position::new(9, 2)), // 16 }
            SourceLocation::new(Position::new(9, 2), Position::new(9, 3)), // 17 )
            SourceLocation::new(Position::new(9, 3), Position::new(9, 4)), // 18 (
            SourceLocation::new(Position::new(9, 4), Position::new(9, 5)), // 19 )
            SourceLocation::new(Position::new(9, 5), Position::new(9, 6)), // 20 ;
        ];
        for (i, (lhs, rhs)) in Scanner::new(js).zip(expectation.iter()).enumerate() {
            let item = lhs.expect("error parsing item");
            assert_eq!((i, item.location), (i, *rhs))
        }
    }

    #[test]
    fn position_display() {
        assert_eq!(format!("{}", Position::new(1, 25)), "1:25".to_string(),);
        assert_eq!(format!("{}", Position::new(25, 0)), "25:0".to_string(),);
    }
    #[test]
    fn position_ord() {
        assert!(
            Position::new(1, 25) < Position::new(2, 25),
            "line 1 not less than line 2"
        );
        assert!(
            Position::new(2, 25) > Position::new(1, 25),
            "line 2 not greater than line 1"
        );
        assert!(
            Position::new(1, 1) < Position::new(1, 5),
            "same line, col 1 not less than col 5"
        );
        assert!(
            Position::new(1, 5) > Position::new(1, 1),
            "same line, col 5 not greater than col 1"
        );
    }

    #[test]
    fn skip_comments() {
        let js = "#! /bin/node;
'use strict';
// comment 1
let x = 1;
/*
Lots of information
in a multi line comment
let q = 0;
*/
let y = 1;
// more than one
/* comment type */
<!-- could be skipped -->
ley z = 9;";
        let mut s = Scanner::new(js);
        for i in 0..4 {
            s.skip_comments().unwrap();
            assert!(
                !s.next().unwrap().unwrap().token.is_comment(),
                " failed to skip comment on iter {}",
                i
            );
        }
    }

    #[test]
    fn invalid_regex_flags_for_error() {
        let mut s = Scanner::new("let x = /asdf");
        let _let = s.next();
        let _x = s.next();
        let _eq = s.next();
        let re = s.next().unwrap();
        assert!(re.is_err(), "regex was not an error");
    }

    #[test]
    fn template_with_middle() {
        let mut s = Scanner::new("`asdf${0}qwerty${1}poiuy`");
        let _head = s.next().unwrap().unwrap();
        let _zero = s.next().unwrap().unwrap();
        let middle = s.next().unwrap().unwrap();
        assert!(middle.token.is_template_body(), "middle was not a template");
        let _one = s.next().unwrap().unwrap();
        let _tail = s.next().unwrap().unwrap();
    }

    #[test]
    #[should_panic = "Unmatched open close paren"]
    fn unmatched_close_paren_error() {
        Scanner::new(")").next().unwrap().unwrap();
    }
    #[test]
    #[should_panic = "unmatched close brace"]
    fn unmatched_close_brace_error() {
        Scanner::new("}").next().unwrap().unwrap();
    }
    #[test]
    fn this_over_number() {
        let mut s = Scanner::new("this / 100");
        let _this = s.next().unwrap().unwrap();
        let div = s.next().unwrap().unwrap();
        assert!(
            div.token.matches_punct(Punct::ForwardSlash),
            "regex with leading this"
        );
        let _one_hundred = s.next().unwrap().unwrap();
    }
    #[test]
    fn keyword_regex() {
        let mut s = Scanner::new("break /a/");
        let _break = s.next().unwrap().unwrap();
        let re = s.next().unwrap().unwrap();
        assert!(re.token.is_regex(), "regex was not a regex: {:?}", re);
    }
}
