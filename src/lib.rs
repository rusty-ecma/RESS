//! ress
//! A crate for parsing raw JS into a token stream
//!
//! The primary interfaces are the function [`tokenize`][tokenize] and
//! the struct [`Scanner`][scanner]. The [`Scanner`][scanner] struct impls [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
//! and the [`tokenize`][tokenize] function is just a wrapper
//! around [`Scanner::collect()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect).
//!
//! The `Scanner` will provide a stream of `Item`s, and `Item` is
//! has 2 properties a [`Token`][token] and a [`Span`][span]. The `Span` is a
//! representation of where the `Item` exists in the original source while the `Token`
//! provides details about what JavaScript token it represents.
//!
//! An example of what a token stream might look like
//!
//! [token]: ./enum.Token
//! [span]: ./struct.Span
//! [scanner]: ./struct.Scanner
//! [tokenize]: ../fn.tokenize

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

mod tokenizer;
pub mod tokens;
use crate::tokenizer::RawToken;
pub use crate::tokenizer::Tokenizer;
pub use crate::tokens::{
    owned::{
        Comment as OwnedComment, Ident as OwnedIdent, Number as OwnedNumber, RegEx as OwnedRegEx,
        StringLit as OwnedStringLit, Template as OwnedTemplate, Token as OwnedToken,
    },
    refs::{Comment, Ident, Number, RegEx, StringLit, Template, Token as RefToken},
    BooleanLiteral as Boolean, Keyword, Punct, Token,
};

/// a convince function for collecting a scanner into
/// a `Vec<Token>`
pub fn tokenize(text: &str) -> Vec<RefToken> {
    Scanner::new(text).map(|i| i.token).collect()
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// A location in the original source text
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    /// Create a new Span from its parts
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item<T> {
    pub token: T,
    pub span: Span,
}

impl<T> Item<T>
where
    T: Token,
{
    pub fn new(token: T, span: Span) -> Self {
        Self { token, span }
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

#[allow(unused)]
pub struct Scanner<'a> {
    pub stream: Tokenizer<'a>,
    pub eof: bool,
    pub spans: Vec<Span>,
    last_open_paren_idx: usize,
    pub pending_new_line: bool,
    original: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(text: &'a str) -> Self {
        let mut stream = Tokenizer::new(text);
        stream.skip_whitespace();
        Self {
            stream,
            eof: false,
            spans: Vec::new(),
            last_open_paren_idx: 0,
            pending_new_line: false,
            original: text,
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Item<RefToken<'a>>;
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
    pub fn look_ahead(&mut self) -> Option<Item<RefToken<'b>>> {
        self.get_next_token(false)
    }
    /// Skip any upcoming comments to get the
    /// next valid js token
    pub fn skip_comments(&mut self) {
        debug!(target: "ress", "skipping comments");
        let mut new_cursor = self.stream.stream.idx;
        while let Some(ref item) = self.next() {
            if let RefToken::Comment(_) = item.token {
                new_cursor = self.stream.stream.idx;
            } else {
                break;
            }
        }
        debug!(target: "ress", "skipped {} bytes worth of comments", new_cursor.saturating_sub(self.stream.stream.idx));
        self.stream.stream.idx = new_cursor;
    }
    /// Get a copy of the scanner's current state
    pub fn get_state(&self) -> ScannerState {
        ScannerState {
            cursor: self.stream.stream.idx,
            spans_len: self.spans.len(),
            last_paren: self.last_open_paren_idx,
            replacement: 0,
            curly_stack: self.stream.curly_stack.clone(),
        }
    }
    /// Set the scanner's current state to the state provided
    pub fn set_state(&mut self, state: ScannerState) {
        self.stream.stream.idx = state.cursor;
        self.spans.truncate(state.spans_len);
        self.last_open_paren_idx = state.last_paren;
        self.stream.curly_stack = state.curly_stack;
    }
    #[inline]
    fn get_next_token(&mut self, advance_cursor: bool) -> Option<Item<RefToken<'b>>> {
        if self.eof {
            debug!(target: "ress", "end of iterator, returning None");
            return None;
        };
        let prev_cursor = self.stream.stream.idx;
        let mut next = self.stream.next_();
        let ret = if next.ty.is_punct()
            && &self.stream.stream.buffer[next.start..next.start.saturating_add(1)] == b"/"
            && self.is_regex_start()
        {
            next = self.stream.next_regex();
            match next.ty {
                RawToken::RegEx(body_end) => {
                    let flags = if next.end > body_end {
                        Some(&self.original[body_end..next.end])
                    } else {
                        None
                    };
                    Item::new(
                        RefToken::RegEx(RegEx {
                            body: &self.original[next.start + 1..body_end - 1],
                            flags,
                        }),
                        Span::new(next.start, next.end),
                    )
                }
                _ => unreachable!("non-regex from next_regex"),
            }
        } else {
            let s = &self.original[next.start..next.end];
            let token = match next.ty {
                RawToken::Boolean(b) => RefToken::Boolean(Boolean::from(b)),
                RawToken::Comment(kind) => match kind {
                    tokens::CommentKind::Multi => RefToken::Comment(Comment::new_multi_line(&s[2..s.len()-2])),
                    tokens::CommentKind::Single => RefToken::Comment(Comment::new_single_line(&s[2..])),
                    tokens::CommentKind::Html => {
                        let (content, tail) = if let Some(idx) = s.rfind("-->") {
                            let actual_end = idx.saturating_add(3);
                            if actual_end < next.end {
                                (&s[4..idx], Some(&s[idx..]))
                            } else {
                                (&s[4..], None)
                            }
                        } else {
                            (&s[4..], None)
                        };
                        RefToken::Comment(Comment::new_html(content, tail))
                    }
                },
                RawToken::EoF => {
                    self.eof = true;
                    RefToken::EoF
                }
                RawToken::Ident => RefToken::Ident(Ident::from(s)),
                RawToken::Keyword(k) => RefToken::Keyword(k),
                RawToken::Null => RefToken::Null,
                RawToken::Numeric(_) => RefToken::Numeric(Number::from(s)),
                RawToken::Punct(p) => RefToken::Punct(p),
                RawToken::RegEx(_) => unreachable!("Regex from next"),
                RawToken::String(k) => {
                    let s = &s[1..s.len() - 1];
                    match k {
                        tokenizer::StringKind::Double => {
                            RefToken::String(StringLit::Double(s))
                        }
                        tokenizer::StringKind::Single => {
                            RefToken::String(StringLit::Single(s))
                        }
                    }
                },
                RawToken::Template(t) => match t {
                    tokenizer::TemplateKind::Head => {
                        let s = &s[1..s.len() - 2];
                        RefToken::Template(Template::Head(s))
                    }
                    tokenizer::TemplateKind::Body => {
                        let s = &s[1..s.len() - 2];
                        RefToken::Template(Template::Middle(s))
                    }
                    tokenizer::TemplateKind::Tail => {
                        let s = &s[1..s.len() - 1];
                        RefToken::Template(Template::Tail(s))
                    }
                    tokenizer::TemplateKind::NoSub => {
                        let s = &s[1..s.len() - 1];
                        RefToken::Template(Template::NoSub(s))
                    }
                },
            };
            Item::new(token, Span::new(next.start, next.end))
        };
        if !advance_cursor {
            self.stream.stream.idx = prev_cursor;
        } else {
            if let RefToken::Punct(ref p) = &ret.token {
                if let Punct::OpenParen = p {
                    self.last_open_paren_idx = self.spans.len()
            }
            }
            self.spans.push(ret.span);
        }
        self.stream.skip_whitespace();
        Some(ret)
    }

    fn is_regex_start(&self) -> bool {
        if let Some(last_token) = self.last_token() {
            match last_token {
                RawToken::Keyword(k) => match k {
                    Keyword::This => false,
                    _ => true,
                },
                RawToken::Punct(p) => match p {
                    Punct::CloseBracket => false,
                    Punct::CloseParen => self.check_for_conditional(),
                    Punct::CloseBrace => self.check_for_func(),
                    _ => true,
                },
                _ => false,
            }
        } else {
            true
        }
    }

    fn last_token(&self) -> Option<RawToken> {
        if self.spans.is_empty() {
            return None;
        }
        let mut current_idx = self.spans.len().saturating_sub(1);
        while current_idx > 0 {
            if let Some(t) = self.token_for(&self.spans[current_idx]) {
                if t.is_comment() {
                    current_idx = current_idx.saturating_sub(1);
                } else {
                    return Some(t);
                }
            }
        }
        None
    }

    fn check_for_conditional(&self) -> bool {
        if let Some(before) = self.nth_before_last_open_paren(1) {
            match before {
                RawToken::Keyword(k) => match k {
                    Keyword::If | Keyword::For | Keyword::While | Keyword::With => true,
                    _ => false,
                },
                _ => false,
            }
        } else {
            true
        }
    }

    fn check_for_func(&self) -> bool {
        if let Some(before) = self.nth_before_last_open_paren(1) {
            if before == RawToken::Ident {
                if let Some(three_before) = self.nth_before_last_open_paren(3) {
                    return Self::check_for_expression(&three_before);
                }
            } else if before == RawToken::Keyword(Keyword::Function) {
                if let Some(two_before) = self.nth_before_last_open_paren(2) {
                    return Self::check_for_expression(&two_before);
                } else {
                    return false;
                }
            }
        }
        true
    }

    fn check_for_expression(token: &RawToken) -> bool {
        match token {
            RawToken::Punct(p) => match p {
                Punct::OpenParen => true,
                Punct::OpenBrace => true,
                Punct::OpenBracket => true,
                Punct::Equal => true,
                Punct::PlusEqual => true,
                Punct::DashEqual => true,
                Punct::AsteriskEqual => true,
                Punct::DoubleAsteriskEqual => true,
                Punct::ForwardSlashEqual => true,
                Punct::PercentEqual => true,
                Punct::DoubleLessThanEqual => true,
                Punct::DoubleGreaterThanEqual => true,
                Punct::TripleGreaterThanEqual => true,
                Punct::AmpersandEqual => true,
                Punct::PipeEqual => true,
                Punct::CaretEqual => true,
                Punct::Comma => true,
                Punct::Plus => true,
                Punct::Dash => true,
                Punct::Asterisk => true,
                Punct::DoubleAsterisk => true,
                Punct::ForwardSlash => true,
                Punct::Percent => true,
                Punct::DoublePlus => true,
                Punct::DoubleDash => true,
                Punct::DoubleLessThan => true,
                Punct::DoubleGreaterThan => true,
                Punct::TripleGreaterThan => true,
                Punct::Ampersand => true,
                Punct::Pipe => true,
                Punct::Caret => true,
                Punct::Bang => true,
                Punct::Tilde => true,
                Punct::DoubleAmpersand => true,
                Punct::DoublePipe => true,
                Punct::QuestionMark => true,
                Punct::Colon => true,
                Punct::TripleEqual => true,
                Punct::DoubleEqual => true,
                Punct::GreaterThanEqual => true,
                Punct::LessThanEqual => true,
                Punct::LessThan => true,
                Punct::GreaterThan => true,
                Punct::BangEqual => true,
                Punct::BangDoubleEqual => true,
                _ => false,
            },
            RawToken::Keyword(k) => match k {
                Keyword::In => true,
                Keyword::TypeOf => true,
                Keyword::InstanceOf => true,
                Keyword::New => true,
                Keyword::Return => true,
                Keyword::Case => true,
                Keyword::Delete => true,
                Keyword::Throw => true,
                Keyword::Void => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn nth_before_last_open_paren(&self, n: usize) -> Option<RawToken> {
        if self.spans.len() < n {
            return None;
        }
        self.token_for(&self.spans[self.last_open_paren_idx.saturating_sub(n)])
    }

    fn token_for(&self, span: &Span) -> Option<tokenizer::RawToken> {
        if self.original.len() < span.end {
            return None;
        }
        let s = &self.original[span.start..span.end];
        Some(Tokenizer::new(s).next_().ty)
    }

    pub fn string_for(&self, span: &Span) -> Option<String> {
        Some(self.str_for(span)?.to_string())
    }

    pub fn str_for(&self, span: &Span) -> Option<&'b str> {
        if self.original.len() < span.start || self.original.len() < span.end {
            None
        } else {
            Some(&self.original[span.start..span.end])
        }
    }
}
#[inline]
fn is_line_term(c: char) -> bool {
    c == '\n' || c == '\r' || c == '\u{2028}' || c == '\u{2029}'
}

#[cfg(test)]
mod test {
    use super::*;
    //     #[test]
    //     fn tokenizer() {
    //         let js = "
    // 'use strict';
    // function thing() {
    //     let x = 0;
    //     console.log('stuff');
    // }";
    //         let expectation = vec![
    //             RefToken::String(StringLit::Single),
    //             RefToken::Punct(Punct::SemiColon),
    //             RefToken::Keyword(Keyword::Function),
    //             RefToken::Ident,
    //             RefToken::Punct(Punct::OpenParen),
    //             RefToken::Punct(Punct::CloseParen),
    //             RefToken::Punct(Punct::OpenBrace),
    //             RefToken::Keyword(Keyword::Let),
    //             RefToken::Ident,
    //             RefToken::Punct(Punct::Equal),
    //             RefToken::Numeric(super::tokens::Number::Dec),
    //             RefToken::punct(Punct::SemiColon),
    //             RefToken::Ident,
    //             RefToken::Punct(Punct::Period),
    //             RefToken::Ident,
    //             RefToken::Punct(Punct::OpenParen),
    //             RefToken::String(StringLit::Single),
    //             RefToken::punct(Punct::CloseParen),
    //             RefToken::punct(Punct::SemiColon),
    //             RefToken::punct(Punct::CloseBrace),
    //             RefToken::EoF,
    //         ];
    //         for tok in tokenize(js).into_iter().zip(expectation.into_iter()) {
    //             assert_eq!(tok.0, tok.1);
    //         }
    //     }

    #[test]
    fn tok_scanner() {
        let s = super::Scanner::new(
            "(function() {
this.x = 100;
this.y = 0;
})();",
        );
        let expected = vec![
            RefToken::Punct(Punct::OpenParen), //"("
            RefToken::Keyword(Keyword::Function),
            RefToken::Punct(Punct::OpenParen),  //"("
            RefToken::Punct(Punct::CloseParen), //")"
            RefToken::Punct(Punct::OpenBrace),  //"{"
            RefToken::Keyword(Keyword::This),
            RefToken::Punct(Punct::Period), //"."
            RefToken::Ident("x".into()),
            RefToken::Punct(Punct::Equal), //"="
            RefToken::Numeric("100".into()),
            RefToken::Punct(Punct::SemiColon), //";"
            RefToken::Keyword(Keyword::This),
            RefToken::Punct(Punct::Period), //"."
            RefToken::Ident("y".into()),
            RefToken::Punct(Punct::Equal), //"="
            RefToken::Numeric("0".into()),
            RefToken::Punct(Punct::SemiColon),  //";"
            RefToken::Punct(Punct::CloseBrace), //"}"
            RefToken::Punct(Punct::CloseParen), //")"
            RefToken::Punct(Punct::OpenParen),  //"("
            RefToken::Punct(Punct::CloseParen), //")"
            RefToken::Punct(Punct::SemiColon),  //";"
            RefToken::EoF,
        ];
        validate(s, expected);
    }

    #[test]
    fn tok_scanner_jq() {
        let js = include_str!("../node_modules/jquery/dist/jquery.js");
        let t = Scanner::new(js);
        let _: Vec<_> = t.collect();
    }

    // #[test]
    // fn template_one_sub() {
    //     let one_sub = "`things and stuff times ${x}`";
    //     let s = Scanner::new(one_sub);
    //     let expected = vec![
    //         Token::template_head("things and stuff times "),
    //         Token::ident("x"),
    //         Token::template_tail(""),
    //     ];
    //     validate(s, expected);
    // }

    // #[test]
    // fn template_two_subs() {
    //     let two_subs = "`things and stuff times ${x} divided by ${y}`";
    //     let s = Scanner::new(two_subs);
    //     let expected = vec![
    //         Token::template_head("things and stuff times "),
    //         Token::ident("x"),
    //         Token::template_middle(" divided by "),
    //         Token::ident("y"),
    //         Token::template_tail(""),
    //     ];
    //     validate(s, expected);
    // }
    // #[test]
    // fn multiline_template() {
    //     let plain = "`things and
    //     stuff`";
    //     let p_r = tokens::token().parse(plain).unwrap();
    //     assert_eq!(
    //         p_r,
    //         (Token::no_sub_template(&plain[1..plain.len() - 1]), "")
    //     );
    //     let subbed = "`things and
    //     stuff times ${x}`";
    //     let s = Scanner::new(subbed);
    //     let expected = vec![
    //         Token::template_head("things and\n        stuff times "),
    //         Token::ident("x"),
    //         Token::template_tail(""),
    //     ];
    //     validate(s, expected);
    // }
    // #[test]
    // fn nested_template() {
    //     let test = "`outer ${`inner ${0}`}`";
    //     let expected = vec![
    //         Token::template_head("outer "),
    //         Token::template_head("inner "),
    //         Token::numeric("0"),
    //         Token::template_tail(""),
    //         Token::template_tail(""),
    //     ];
    //     let s = Scanner::new(test);
    //     validate(s, expected);
    // }
    // #[test]
    // fn look_ahead() {
    //     let js = "function() { return; }";
    //     let mut s = Scanner::new(js);
    //     loop {
    //         let peek = s.look_ahead();
    //         let next = s.next();
    //         assert_eq!(peek, next);
    //         if peek.is_none() {
    //             break;
    //         }
    //     }
    // }

    fn validate(s: Scanner, expected: Vec<RefToken>) {
        for (i, (lhs, rhs)) in s.zip(expected.into_iter()).enumerate() {
            println!("{:?}, {:?}", lhs.token, rhs);
            assert_eq!((i, lhs.token), (i, rhs));
        }
    }

    // #[test]
    // fn get_str() {
    //     let js = "function ( ) { return ; }";
    //     let mut s = Scanner::new(js);
    //     let strs = js.split(' ');
    //     for (i, p) in strs.enumerate() {
    //         let item = s.next().unwrap();
    //         let q = s.string_for(&item.span).unwrap();
    //         assert_eq!((i, p.to_string()), (i, q))
    //     }
    // }

    // #[test]
    // fn item_deref_to_token() {
    //     let js = "function ( ) { return ; }";
    //     let mut s = Scanner::new(js);
    //     let i: Item = s.next().unwrap();

    //     // explicit reference to token
    //     assert!(i.token.is_keyword());
    //     // implicit deref to token
    //     assert!(i.is_keyword());
    // }

    // #[test]
    // fn spans() {
    //     let js = include_str!("../node_modules/esprima/dist/esprima.js");
    //     let mut s = Scanner::new(js);
    //     while let Some(ref item) = s.next() {
    //         let from_stream = &s.stream[item.span.start..item.span.end];
    //         let token = item.token.to_string();

    //         if from_stream != token {
    //             panic!("token mismatch {:?} \n{}\n{}\n", item, from_stream, token);
    //         }
    //     }
    // }

    // #[test]
    // fn local_host_regex() {
    //     let js = r#"/^(http|https):\/\/(localhost|127\.0\.0\.1)/"#;
    //     let mut s = Scanner::new(js);
    //     let r = s.next().unwrap();
    //     assert_eq!(
    //         r.token,
    //         Token::regex(r#"^(http|https):\/\/(localhost|127\.0\.0\.1)"#, None)
    //     );
    // }
}

pub mod error {
    #[derive(Debug)]
    pub enum Error {
        DataMismatch(String),
    }

    impl ::std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
            match self {
                Error::DataMismatch(ref msg) => msg.fmt(f),
            }
        }
    }

    impl ::std::error::Error for Error {}

    impl From<::std::num::ParseIntError> for Error {
        fn from(other: ::std::num::ParseIntError) -> Self {
            Error::DataMismatch(format!("Error parsing int: {}", other))
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OpenCurlyKind {
    Template,
    Block,
}

#[derive(Clone)]
pub struct ScannerState {
    pub cursor: usize,
    pub spans_len: usize,
    pub last_paren: usize,
    pub replacement: usize,
    pub curly_stack: Vec<OpenCurlyKind>,
}
