use combine::Parser;
pub mod comments;
pub mod keywords;
pub mod numbers;
pub mod punct;
pub mod regex;
pub mod strings;
pub mod tokens;

use super::{is_line_term, whitespace_or_line_term, ScannerState};
use keywords::Keyword;
use punct::Punct;
pub use refs::tokens::RefToken;
use tokens::Span;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RefItem {
    pub token: RefToken,
    pub span: Span,
}

impl RefItem {
    pub fn new(token: RefToken, span: Span) -> Self {
        Self { token, span }
    }
}

#[allow(unused)]
pub struct RefScanner {
    pub stream: String,
    pub eof: bool,
    pub cursor: usize,
    pub spans: Vec<Span>,
    last_open_paren_idx: usize,
    template: usize,
    replacement: usize,
    pub pending_new_line: bool,
    curly_count: usize,
}

impl RefScanner {
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let cursor = text.len() - text.trim_start_matches(super::whitespace).len();
        Self {
            stream: text,
            eof: false,
            cursor,
            spans: vec![],
            last_open_paren_idx: 0,
            template: 0,
            replacement: 0,
            pending_new_line: false,
            curly_count: 0,
        }
    }
}

impl Iterator for RefScanner {
    type Item = RefItem;
    fn next(&mut self) -> Option<RefItem> {
        self.get_next_token(true)
    }
}

impl RefScanner {
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
    pub fn look_ahead(&mut self) -> Option<RefItem> {
        self.get_next_token(false)
    }
    /// Skip any upcoming comments to get the
    /// next valid js token
    pub fn skip_comments(&mut self) {
        debug!(target: "ress", "skipping comments");
        let mut new_cursor = self.cursor;
        while let Some(ref item) = self.next() {
            if let RefToken::Comment(_) = item.token {
                new_cursor = self.cursor;
            } else {
                break;
            }
        }
        debug!(target: "ress", "skipped {} bytes worth of comments", new_cursor.saturating_sub(self.cursor));
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
            curly_count: self.curly_count,
        }
    }
    /// Set the scanner's current state to the state provided
    pub fn set_state(&mut self, state: ScannerState) {
        self.cursor = state.cursor;
        self.spans.truncate(state.spans_len);
        self.last_open_paren_idx = state.last_paren;
        self.template = state.template;
        self.replacement = state.replacement;
        self.curly_count = state.curly_count;
    }

    fn get_next_token<'a>(&mut self, advance_cursor: bool) -> Option<RefItem> {
        if self.eof {
            debug!(target: "ress", "end of iterator, returning None");
            return None;
        };
        let prev_cursor = self.cursor;
        let result = self::tokens::token().parse(&self.stream[self.cursor..]);
        match result {
            Ok(pair) => {
                if pair.0.matches_punct(&Punct::ForwardSlash) && self.is_regex_start() {
                    match regex::regex_tail().parse(pair.1) {
                        Ok(regex_pair) => {
                            let full_len = self.stream.len();
                            let span_end = full_len - regex_pair.1.len();
                            let span = Span::new(self.cursor, span_end);
                            if advance_cursor {
                                self.spans.push(span.clone());
                                self.cursor = self.stream.len()
                                    - regex_pair
                                        .1
                                        .trim_start_matches(whitespace_or_line_term)
                                        .len();
                                let whitespace = &self.stream[prev_cursor..self.cursor];
                                self.pending_new_line = whitespace.chars().any(is_line_term);
                            }
                            debug!(target: "ress", "{}: {:?}", if advance_cursor { "next regex item" } else {"look ahead"}, regex_pair.0);
                            Some(RefItem::new(regex_pair.0, span))
                        }
                        Err(e) => panic!(
                            "Failed to parse token last successful parse ended {}\nError: {}",
                            self.cursor, e,
                        ),
                    }
                } else if self.template > 0
                    && pair.0.matches_punct(&Punct::CloseBrace)
                    && self.curly_count == 0 {
                    match strings::template_continuation().parse(pair.1) {
                        Ok(pair) => {
                            if pair.0.is_template_tail() && advance_cursor {
                                self.template = self.template.saturating_sub(1);
                            }
                            let full_len = self.stream.len();
                            let span_end = full_len - pair.1.len();
                            let span = Span::new(self.cursor, span_end);
                            if advance_cursor {
                                self.spans.push(span.clone());
                                self.cursor = self.stream.len()
                                    - pair.1.trim_start_matches(whitespace_or_line_term).len();
                                let whitespace = &self.stream[prev_cursor..self.cursor];
                                self.pending_new_line = whitespace.chars().any(is_line_term);
                            }
                            debug!(target: "ress", "{}: {:?}", if advance_cursor { "next template item" } else {"look ahead"}, pair.0);
                            Some(RefItem::new(pair.0, span))
                        }
                        Err(e) => panic!(
                            "Failed to parse token last successful parse ended {}\nError: {}",
                            self.cursor, e,
                        ),
                    }
                } else {
                    if self.template > 0 && pair.0.matches_punct(&Punct::OpenBrace) {
                        self.curly_count = self.curly_count.saturating_add(1);
                    }
                    if self.template > 0 && pair.0.matches_punct(&Punct::CloseBrace) {
                        self.curly_count = self.curly_count.saturating_sub(1);
                    }
                    if pair.0.matches_punct(&Punct::OpenParen) && advance_cursor {
                        self.last_open_paren_idx = self.spans.len();
                    }
                    if pair.0.is_eof() && advance_cursor {
                        self.eof = true;
                    }
                    if pair.0.is_template_head() && advance_cursor && !pair.0.is_template_tail() {
                        self.template += 1;
                    }
                    let full_len = self.stream.len();
                    let span_end = full_len - pair.1.len();
                    let span = Span::new(self.cursor, span_end);
                    if advance_cursor {
                        self.spans.push(span.clone());
                        self.cursor = self.stream.len()
                            - pair
                                .1
                                .trim_start_matches(super::whitespace_or_line_term)
                                .len();
                        let whitespace = &self.stream[prev_cursor..self.cursor];
                        self.pending_new_line = whitespace.chars().any(super::is_line_term);
                    }
                    info!(target: "ress", "{}: {:?}", if advance_cursor { "next item" } else {"look ahead"}, pair.0);
                    Some(RefItem::new(pair.0, span))
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
                || last_token.matches_keyword(&Keyword::This)
                || last_token.matches_punct(&Punct::CloseBracket)
            {
                false
            } else if last_token.matches_punct(&Punct::CloseParen) {
                self.check_for_conditional()
            } else if last_token.matches_punct(&Punct::CloseBrace) {
                self.check_for_func()
            } else {
                true
            }
        } else {
            true
        }
    }

    fn last_token(&self) -> Option<RefToken> {
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
            before.matches_keyword(&Keyword::If)
                || before.matches_keyword(&Keyword::For)
                || before.matches_keyword(&Keyword::While)
                || before.matches_keyword(&Keyword::With)
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
            } else if before.matches_keyword(&Keyword::Function) {
                if let Some(two_before) = self.nth_before_last_open_paren(2) {
                    return Self::check_for_expression(&two_before);
                } else {
                    return false;
                }
            }
        }
        true
    }

    fn check_for_expression(token: &RefToken) -> bool {
        token.matches_punct(&Punct::OpenParen)
            && !token.matches_punct(&Punct::OpenBrace)
            && !token.matches_punct(&Punct::OpenBracket)
            && !token.matches_punct(&Punct::Assign)
            && !token.matches_punct(&Punct::AddAssign)
            && !token.matches_punct(&Punct::SubtractAssign)
            && !token.matches_punct(&Punct::MultiplyAssign)
            && !token.matches_punct(&Punct::ExponentAssign)
            && !token.matches_punct(&Punct::DivideAssign)
            && !token.matches_punct(&Punct::ModuloAssign)
            && !token.matches_punct(&Punct::LeftShiftAssign)
            && !token.matches_punct(&Punct::RightShiftAssign)
            && !token.matches_punct(&Punct::UnsignedRightShiftAssign)
            && !token.matches_punct(&Punct::BitwiseAndAssign)
            && !token.matches_punct(&Punct::BitwiseOrAssign)
            && !token.matches_punct(&Punct::BitwiseXOrAssign)
            && !token.matches_punct(&Punct::Comma)
            && !token.matches_punct(&Punct::Plus)
            && !token.matches_punct(&Punct::Minus)
            && !token.matches_punct(&Punct::Asterisk)
            && !token.matches_punct(&Punct::Exponent)
            && !token.matches_punct(&Punct::ForwardSlash)
            && !token.matches_punct(&Punct::Modulo)
            && !token.matches_punct(&Punct::Increment)
            && !token.matches_punct(&Punct::Decrement)
            && !token.matches_punct(&Punct::LeftShift)
            && !token.matches_punct(&Punct::RightShift)
            && !token.matches_punct(&Punct::UnsignedRightShift)
            && !token.matches_punct(&Punct::And)
            && !token.matches_punct(&Punct::Pipe)
            && !token.matches_punct(&Punct::Caret)
            && !token.matches_punct(&Punct::Not)
            && !token.matches_punct(&Punct::BitwiseNot)
            && !token.matches_punct(&Punct::LogicalAnd)
            && !token.matches_punct(&Punct::LogicalOr)
            && !token.matches_punct(&Punct::QuestionMark)
            && !token.matches_punct(&Punct::Colon)
            && !token.matches_punct(&Punct::StrictEquals)
            && !token.matches_punct(&Punct::Equal)
            && !token.matches_punct(&Punct::GreaterThanEqual)
            && !token.matches_punct(&Punct::LessThanEqual)
            && !token.matches_punct(&Punct::LessThan)
            && !token.matches_punct(&Punct::GreaterThan)
            && !token.matches_punct(&Punct::NotEqual)
            && !token.matches_punct(&Punct::StrictNotEquals)
            && !token.matches_keyword(&Keyword::In)
            && !token.matches_keyword(&Keyword::TypeOf)
            && !token.matches_keyword(&Keyword::InstanceOf)
            && !token.matches_keyword(&Keyword::New)
            && !token.matches_keyword(&Keyword::Return)
            && !token.matches_keyword(&Keyword::Case)
            && !token.matches_keyword(&Keyword::Delete)
            && !token.matches_keyword(&Keyword::Throw)
            && !token.matches_keyword(&Keyword::Void)
    }

    fn nth_before_last_open_paren(&self, n: usize) -> Option<RefToken> {
        if self.spans.len() < n {
            return None;
        }
        self.token_for(&self.spans[self.last_open_paren_idx - n])
    }

    fn token_for(&self, span: &Span) -> Option<RefToken> {
        if let Ok(t) = self::tokens::token().parse(&self.stream[span.start..span.end]) {
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
    //             RefToken::Punct(Punct::Assign),
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
    fn ref_scanner() {
        let s = super::RefScanner::new(
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
            RefToken::Ident,
            RefToken::Punct(Punct::Assign), //"="
            RefToken::Numeric(tokens::Number::Dec),
            RefToken::Punct(Punct::SemiColon), //";"
            RefToken::Keyword(Keyword::This),
            RefToken::Punct(Punct::Period), //"."
            RefToken::Ident,
            RefToken::Punct(Punct::Assign), //"="
            RefToken::Numeric(tokens::Number::Dec),
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

    fn validate(s: RefScanner, expected: Vec<RefToken>) {
        for (i, (lhs, rhs)) in s.zip(expected.into_iter()).enumerate() {
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
