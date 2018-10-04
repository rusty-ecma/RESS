//! js_parse
//! A crate for parsing raw JS into a token stream
//!
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
#[cfg(test)]
#[macro_use]
extern crate proptest;
#[macro_use]
extern crate combine;
#[macro_use]
extern crate log;
extern crate unic_ucd_ident;

use combine::Parser;
mod comments;
mod keywords;
mod numeric;
mod punct;
mod regex;
mod strings;
mod tokens;
mod unicode;
pub use comments::{Comment, Kind as CommentKind};
pub use keywords::Keyword;
pub use numeric::Number;
pub use punct::Punct;
pub use regex::RegEx;
pub use strings::{StringLit, Template};
pub use tokens::{BooleanLiteral as Boolean, Ident, Item, Span, Token};

/// a convince function for collecting a scanner into
/// a `Vec<Token>`
pub fn tokenize(text: &str) -> Vec<Token> {
    Scanner::new(text).map(|i| i.token).collect()
}

/// An iterator over a js token stream
pub struct Scanner {
    pub stream: String,
    pub eof: bool,
    pub cursor: usize,
    pub spans: Vec<Span>,
    last_open_paren_idx: usize,
    template: usize,
    replacement: usize,
    pub pending_new_line: bool,
}

impl Scanner {
    /// Create a new Scanner with the raw JS text
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let cursor = text.len() - text.trim_left_matches(whitespace).len();
        Scanner {
            stream: text,
            eof: false,
            cursor,
            spans: vec![],
            last_open_paren_idx: 0,
            template: 0,
            replacement: 0,
            pending_new_line: false,
        }
    }
}

impl Iterator for Scanner {
    type Item = Item;
    fn next(&mut self) -> Option<Item> {
        self.get_next_token(true)
    }
}

impl Scanner {
    /// Attempts to look ahead 1 token
    ///
    /// Similar to how `Peekable::peek` works however the
    /// returned value will not be a borrowed `Item`. Since
    /// there isn't a borrow happening this essentially duplicates
    /// the cost of calling `next`.
    ///
    /// ```
    /// # extern crate ress;
    /// # use ress::{Scanner,Token};
    /// # fn main() {
    /// let js = "function thing() { return; }";
    /// let mut s = Scanner::new(js);
    /// assert_eq!(s.look_ahead().unwrap().token, Token::keyword("function"));
    /// assert_eq!(s.next().unwrap().token, Token::keyword("function"));
    /// # }
    /// ```
    pub fn look_ahead(&mut self) -> Option<Item> {
        self.get_next_token(false)
    }
    /// Skip any upcoming comments to get the
    /// next valid js token
    pub fn skip_comments(&mut self) {
        debug!(target: "ress", "skipping comments");
        let mut new_cursor = self.cursor;
        while let Some(ref item) = self.next() {
            if item.token.is_comment() {
                new_cursor = self.cursor;
            } else {
                break;
            }
        }
        debug!(target: "ress", "skipped {} bytes worth of comments", new_cursor - self.cursor);
        self.cursor = new_cursor;
    }
    /// Get a copy of the scanner's current state
    pub fn get_state(&self) -> ScannerState {
        ScannerState {
            cursor: self.cursor,
            spans_len: self.spans.len(),
            last_paren: self.last_open_paren_idx,
            template: self.template,
            replacement: self.replacement,
        }
    }
    /// Set the scanner's current state to the state provided
    pub fn set_state(&mut self, state: ScannerState) {
        self.cursor = state.cursor;
        self.spans.truncate(state.spans_len);
        self.last_open_paren_idx = state.last_paren;
        self.template = state.template;
        self.replacement = state.replacement;
    }

    fn get_next_token(&mut self, advance_cursor: bool) -> Option<Item> {
        if self.eof {
            debug!(target: "ress", "end of iterator, returning None");
            return None;
        };
        let prev_cursor = self.cursor;
        let result = if self.template > 0 && self.template < self.replacement {
            debug!(target: "ress", "parsing template continuation");
            strings::template_continuation().parse(&self.stream[self.cursor..])
        } else {
            tokens::token().parse(&self.stream[self.cursor..])
        };
        match result {
            Ok(pair) => {
                if pair.0.matches_punct(Punct::ForwardSlash) && self.is_regex_start() {
                    match regex::regex_tail().parse(pair.1) {
                        Ok(regex_pair) => {
                            let full_len = self.stream.len();
                            let span_end = full_len - regex_pair.1.len();
                            let span = Span::new(self.cursor, span_end);
                            if advance_cursor {
                                self.spans.push(span.clone());
                                self.cursor = self.stream.len()
                                    - regex_pair.1.trim_left_matches(whitespace).len();
                                let whitespace = &self.stream[prev_cursor..self.cursor];
                                self.pending_new_line = whitespace.chars().any(|c| {
                                    c == '\n' || c == '\r' || c == '\u{2028}' || c == '\u{2029}'
                                });
                            }
                            debug!(target: "ress", "{}: {:?}", if advance_cursor { "next regex item" } else {"look ahead"}, regex_pair.0);
                            Some(Item::new(regex_pair.0, span))
                        }
                        Err(e) => panic!(
                            "Failed to parse token last successful parse ended {}\nError: {}",
                            self.cursor, e,
                        ),
                    }
                } else if self.template > 0
                    && self.replacement == self.template
                    && pair.0.matches_punct(Punct::CloseBrace)
                {
                    match strings::template_continuation().parse(pair.1) {
                        Ok(pair) => {
                            if pair.0.is_template_tail() && advance_cursor {
                                self.replacement = self.replacement.saturating_sub(1);
                                self.template = self.template.saturating_sub(1);
                            }
                            let full_len = self.stream.len();
                            let span_end = full_len - pair.1.len();
                            let span = Span::new(self.cursor, span_end);
                            if advance_cursor {
                                self.spans.push(span.clone());
                                self.cursor =
                                    self.stream.len() - pair.1.trim_left_matches(whitespace).len();
                                let whitespace = &self.stream[prev_cursor..self.cursor];
                                self.pending_new_line = whitespace.chars().any(|c| {
                                    c == '\n' || c == '\r' || c == '\u{2028}' || c == '\u{2029}'
                                });
                            }
                            debug!(target: "ress", "{}: {:?}", if advance_cursor { "next template item" } else {"look ahead"}, pair.0);
                            Some(Item::new(pair.0, span))
                        }
                        Err(e) => panic!(
                            "Failed to parse token last successful parse ended {}\nError: {}",
                            self.cursor, e,
                        ),
                    }
                } else {
                    if pair.0.matches_punct(Punct::OpenParen) && advance_cursor {
                        self.last_open_paren_idx = self.spans.len();
                    }
                    if pair.0.is_eof() && advance_cursor {
                        self.eof = true;
                    }
                    if pair.0.is_template_head() && advance_cursor {
                        self.template += 1;
                        self.replacement += 1;
                    }
                    let full_len = self.stream.len();
                    let span_end = full_len - pair.1.len();
                    let span = Span::new(self.cursor, span_end);
                    if advance_cursor {
                        self.spans.push(span.clone());
                        self.cursor =
                            self.stream.len() - pair.1.trim_left_matches(whitespace).len();
                        let whitespace = &self.stream[prev_cursor..self.cursor];
                        self.pending_new_line = whitespace
                            .chars()
                            .any(|c| c == '\n' || c == '\r' || c == '\u{2028}' || c == '\u{2029}');
                    }
                    debug!(target: "ress", "{}: {:?}", if advance_cursor { "next item" } else {"look ahead"}, pair.0);
                    Some(Item::new(pair.0, span))
                }
            }
            Err(e) => panic!(
                "Failed to parse token last successful parse ended {}\nError: {}",
                self.cursor, e,
            ),
        }
    }

    fn is_regex_start(&self) -> bool {
        if let Some(last_token) = self.last_token() {
            if (!last_token.is_keyword() && !last_token.is_punct())
                || last_token.matches_keyword(Keyword::This)
                || last_token.matches_punct(Punct::CloseBracket)
            {
                false
            } else if last_token.matches_punct(Punct::CloseParen) {
                self.check_for_conditional()
            } else if last_token.matches_punct(Punct::CloseBrace) {
                self.check_for_func()
            } else {
                true
            }
        } else {
            true
        }
    }

    fn last_token(&self) -> Option<Token> {
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
            before.matches_keyword(Keyword::If)
                || before.matches_keyword(Keyword::For)
                || before.matches_keyword(Keyword::While)
                || before.matches_keyword(Keyword::With)
        } else {
            true
        }
    }

    fn check_for_func(&self) -> bool {
        if let Some(before) = self.nth_before_last_open_paren(1) {
            if before.is_ident() {
                if let Some(three_before) = self.nth_before_last_open_paren(3) {
                    return Self::check_for_expression(&three_before);
                }
            } else if before.matches_keyword(Keyword::Function) {
                if let Some(two_before) = self.nth_before_last_open_paren(2) {
                    return Self::check_for_expression(&two_before);
                } else {
                    return false;
                }
            }
        }
        true
    }

    fn check_for_expression(token: &Token) -> bool {
        token.matches_punct(Punct::OpenParen)
            && !token.matches_punct(Punct::OpenBrace)
            && !token.matches_punct(Punct::OpenBracket)
            && !token.matches_punct(Punct::Assign)
            && !token.matches_punct(Punct::AddAssign)
            && !token.matches_punct(Punct::SubtractAssign)
            && !token.matches_punct(Punct::MultiplyAssign)
            && !token.matches_punct(Punct::ExponentAssign)
            && !token.matches_punct(Punct::DivideAssign)
            && !token.matches_punct(Punct::ModuloAssign)
            && !token.matches_punct(Punct::LeftShiftAssign)
            && !token.matches_punct(Punct::RightShiftAssign)
            && !token.matches_punct(Punct::UnsignedRightShiftAssign)
            && !token.matches_punct(Punct::BitwiseAndAssign)
            && !token.matches_punct(Punct::BitwiseOrAssign)
            && !token.matches_punct(Punct::BitwiseXOrAssign)
            && !token.matches_punct(Punct::Comma)
            && !token.matches_punct(Punct::Plus)
            && !token.matches_punct(Punct::Minus)
            && !token.matches_punct(Punct::Asterisk)
            && !token.matches_punct(Punct::Exponent)
            && !token.matches_punct(Punct::ForwardSlash)
            && !token.matches_punct(Punct::Modulo)
            && !token.matches_punct(Punct::Increment)
            && !token.matches_punct(Punct::Decrement)
            && !token.matches_punct(Punct::LeftShift)
            && !token.matches_punct(Punct::RightShift)
            && !token.matches_punct(Punct::UnsignedRightShift)
            && !token.matches_punct(Punct::And)
            && !token.matches_punct(Punct::Pipe)
            && !token.matches_punct(Punct::Caret)
            && !token.matches_punct(Punct::Not)
            && !token.matches_punct(Punct::BitwiseNot)
            && !token.matches_punct(Punct::LogicalAnd)
            && !token.matches_punct(Punct::LogicalOr)
            && !token.matches_punct(Punct::QuestionMark)
            && !token.matches_punct(Punct::Colon)
            && !token.matches_punct(Punct::StrictEquals)
            && !token.matches_punct(Punct::Equal)
            && !token.matches_punct(Punct::GreaterThanEqual)
            && !token.matches_punct(Punct::LessThanEqual)
            && !token.matches_punct(Punct::LessThan)
            && !token.matches_punct(Punct::GreaterThan)
            && !token.matches_punct(Punct::NotEqual)
            && !token.matches_punct(Punct::StrictNotEquals)
            && !token.matches_keyword(Keyword::In)
            && !token.matches_keyword(Keyword::TypeOf)
            && !token.matches_keyword(Keyword::InstanceOf)
            && !token.matches_keyword(Keyword::New)
            && !token.matches_keyword(Keyword::Return)
            && !token.matches_keyword(Keyword::Case)
            && !token.matches_keyword(Keyword::Delete)
            && !token.matches_keyword(Keyword::Throw)
            && !token.matches_keyword(Keyword::Void)
    }

    fn nth_before_last_open_paren(&self, n: usize) -> Option<Token> {
        if self.spans.len() < n {
            return None;
        }
        self.token_for(&self.spans[self.last_open_paren_idx - n])
    }

    fn token_for(&self, span: &Span) -> Option<Token> {
        if let Ok(t) = tokens::token().parse(&self.stream[span.start..span.end]) {
            Some(t.0)
        } else {
            None
        }
    }

    pub fn string_for(&self, span: &Span) -> Option<String> {
        if self.stream.len() < span.start || self.stream.len() < span.end {
            None
        } else {
            Some(self.stream[span.start..span.end].to_string())
        }
    }
}

fn whitespace(c: char) -> bool {
    c.is_whitespace() || c as u32 == 65279
}

pub(crate) fn is_source_char(c: char) -> bool {
    c as u32 <= 0x10_FF_FF
}

pub(crate) fn is_line_term(c: char) -> bool {
    c == '\n' || c == '\r' || c == '\u{2028}' || c == '\u{2029}'
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
#[derive(Clone, Copy)]
pub struct ScannerState {
    pub cursor: usize,
    pub spans_len: usize,
    pub last_paren: usize,
    pub template: usize,
    pub replacement: usize,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tokenizer() {
        let js = "
'use strict';
function thing() {
    let x = 0;
    console.log('stuff');
}";
        let expectation = vec![
            Token::single_quoted_string("use strict"),
            Token::punct(";"),
            Token::keyword("function"),
            Token::ident("thing"),
            Token::punct("("),
            Token::punct(")"),
            Token::punct("{"),
            Token::keyword("let"),
            Token::ident("x"),
            Token::punct("="),
            Token::numeric("0"),
            Token::punct(";"),
            Token::ident("console"),
            Token::punct("."),
            Token::ident("log"),
            Token::punct("("),
            Token::single_quoted_string("stuff"),
            Token::punct(")"),
            Token::punct(";"),
            Token::punct("}"),
            Token::EoF,
        ];
        for tok in tokenize(js).into_iter().zip(expectation.into_iter()) {
            assert_eq!(tok.0, tok.1);
        }
    }

    #[test]
    fn scanner() {
        let s = super::Scanner::new(
            "(function() {
this.x = 100;
this.y = 0;
})();",
        );
        let expected = vec![
            Token::punct("("),
            Token::keyword("function"),
            Token::punct("("),
            Token::punct(")"),
            Token::punct("{"),
            Token::keyword("this"),
            Token::punct("."),
            Token::ident("x"),
            Token::punct("="),
            Token::numeric("100"),
            Token::punct(";"),
            Token::keyword("this"),
            Token::punct("."),
            Token::ident("y"),
            Token::punct("="),
            Token::numeric("0"),
            Token::punct(";"),
            Token::punct("}"),
            Token::punct(")"),
            Token::punct("("),
            Token::punct(")"),
            Token::punct(";"),
            Token::EoF,
        ];
        validate(s, expected);
    }

    #[test]
    fn template_one_sub() {
        let one_sub = "`things and stuff times ${x}`";
        let s = Scanner::new(one_sub);
        let expected = vec![
            Token::template_head("things and stuff times "),
            Token::ident("x"),
            Token::template_tail(""),
        ];
        validate(s, expected);
    }

    #[test]
    fn template_two_subs() {
        let two_subs = "`things and stuff times ${x} divided by ${y}`";
        let s = Scanner::new(two_subs);
        let expected = vec![
            Token::template_head("things and stuff times "),
            Token::ident("x"),
            Token::template_middle(" divided by "),
            Token::ident("y"),
            Token::template_tail(""),
        ];
        validate(s, expected);
    }
    #[test]
    fn multiline_template() {
        let plain = "`things and
        stuff`";
        let p_r = tokens::token().parse(plain).unwrap();
        assert_eq!(
            p_r,
            (Token::no_sub_template(&plain[1..plain.len() - 1]), "")
        );
        let subbed = "`things and
        stuff times ${x}`";
        let s = Scanner::new(subbed);
        let expected = vec![
            Token::template_head("things and\n        stuff times "),
            Token::ident("x"),
            Token::template_tail(""),
        ];
        validate(s, expected);
    }
    #[test]
    fn nested_template() {
        let test = "`outer ${`inner ${0}`}`";
        let expected = vec![
            Token::template_head("outer "),
            Token::template_head("inner "),
            Token::numeric("0"),
            Token::template_tail(""),
            Token::template_tail(""),
        ];
        let s = Scanner::new(test);
        validate(s, expected);
    }
    #[test]
    fn look_ahead() {
        let js = "function() { return; }";
        let mut s = Scanner::new(js);
        loop {
            let peek = s.look_ahead();
            let next = s.next();
            assert_eq!(peek, next);
            if peek.is_none() {
                break;
            }
        }
    }

    fn validate(s: Scanner, expected: Vec<Token>) {
        for (i, (lhs, rhs)) in s.zip(expected.into_iter()).enumerate() {
            assert_eq!((i, lhs.token), (i, rhs));
        }
    }

    #[test]
    fn get_str() {
        let js = "function ( ) { return ; }";
        let mut s = Scanner::new(js);
        let strs = js.split(' ');
        for (i, p) in strs.enumerate() {
            let item = s.next().unwrap();
            let q = s.string_for(&item.span).unwrap();
            assert_eq!((i, p.to_string()), (i, q))
        }
    }

    #[test]
    fn item_deref_to_token() {
        let js = "function ( ) { return ; }";
        let mut s = Scanner::new(js);
        let i: Item = s.next().unwrap();

        // explicit reference to token
        assert!(i.token.is_keyword());
        // implitic deref to token
        assert!(i.is_keyword());
    }

    #[test]
    fn spans() {
        let js = include_str!("../node_modules/esprima/dist/esprima.js");
        let mut s = Scanner::new(js);
        while let Some(ref item) = s.next() {
            let from_stream = &s.stream[item.span.start..item.span.end];
            let token = item.token.to_string();

            if from_stream != token {
                panic!("token mismatch {:?} \n{}\n{}\n", item, from_stream, token);
            }
        }
    }

    #[test]
    fn local_host_regex() {
        let js = r#"/^(http|https):\/\/(localhost|127\.0\.0\.1)/"#;
        let mut s = Scanner::new(js);
        let r = s.next().unwrap();
        assert_eq!(
            r.token,
            Token::regex(r#"^(http|https):\/\/(localhost|127\.0\.0\.1)"#, None)
        );
    }
}
