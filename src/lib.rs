//! js_parse
//! A crate for parsing raw JS into a token stream
#[macro_use]
extern crate combine;
use combine::{Parser, Stream, parser::char::char as c_char, error::ParseError};
mod comments;
mod keywords;
mod numeric;
mod punct;
mod regex;
mod strings;
mod tokens;
mod unicode;
pub use comments::Comment;
pub use keywords::Keyword;
pub use numeric::Number;
pub use punct::Punct;
pub use regex::RegEx;
pub use strings::StringLit;
pub use tokens::{Token, Item, BooleanLiteral as Boolean, Span};

/// Send over the complete text and get back
/// the completely parsed result
pub fn tokenize(text: &str) -> Vec<Token> {
    Scanner::new(text).map(|i| i.token).collect()
}

/// An iterator over a token stream built
/// from raw js text
pub struct Scanner {
    stream: String,
    eof: bool,
    cursor: usize,
    spans: Vec<Span>,
    last_open_paren_idx: usize,
}

impl Scanner {
    /// Create a new Scanner with the raw JS text
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let cursor = text.len() - text.trim_left().len();
        Scanner {
            stream: text,
            eof: false,
            cursor,
            spans: vec![],
            last_open_paren_idx: 0,
        }
    }

    //TODO: Implement construction from a reader
}

impl Iterator for Scanner {
    type Item = Item;
    fn next(&mut self) -> Option<Item> {
        if self.eof {
            return None;
        };
        match tokens::token().easy_parse(&self.stream[self.cursor..]) {
            Ok(pair) => {
                let debug = pair.0.matches_punct(Punct::ForwardSlash);
                if debug {
                }
                if pair.0.matches_punct(Punct::ForwardSlash) && self.is_regex_start() {
                    match regex::regex_tail().easy_parse(pair.1) {
                        Ok(pair) => {
                            let full_len = self.stream.len();
                            let span_end = full_len - pair.1.len();
                            let span = Span::new(self.cursor, span_end);
                            self.spans.push(span);
                            let ret = Some(Item::new(pair.0, Span::new(self.cursor, span_end)));
                            self.cursor = self.stream.len() - pair.1.trim_left().len();
                            ret
                        }
                        Err(e) => panic!("Failed to parse token last successful parse ended {}\nError: {:?}", self.cursor, e,),
                    }
                } else {
                    if debug {
                    }
                    if pair.0.matches_punct(Punct::OpenParen) {
                        self.last_open_paren_idx = self.spans.len();
                    }
                    if pair.0.is_eof() {
                        self.eof = true;
                    }
                    let full_len = self.stream.len();
                    let span_end = full_len - pair.1.len();
                    let span = Span::new(self.cursor, span_end);
                    self.spans.push(span);
                    let ret = Some(Item::new(pair.0, Span::new(self.cursor, span_end)));
                    self.cursor = self.stream.len() - pair.1.trim_left().len();
                    ret
                }
            },
            Err(e) => panic!("Failed to parse token last successful parse ended {}\nError: {:?}", self.cursor, e,),
        }
    }
}

impl Scanner {
    fn is_regex_start(&self) -> bool {
        if let Some(last_token) = self.last_token() {
            if !last_token.is_keyword() && !last_token.is_punct() {
                false
            } else if last_token.matches_keyword(Keyword::This) || last_token.matches_punct(Punct::CloseBrace) {
                false
            } else if last_token.matches_punct(Punct::CloseParen) {
                self.check_for_conditional()
            } else if last_token.matches_punct(Punct::CloseBrace) {
                self.check_for_func()
            } else {
                true
            }
        } else {
            false
        }
    }

    fn last_token(&self) -> Option<Token> {
        if self.spans.len() == 0 {
            return None;
        }
        self.token_for(&self.spans[self.spans.len() - 1])
    }


    fn check_for_conditional(&self) -> bool {
        if let Some(before) = self.nth_before_last_open_paren(1) {
            before.matches_keyword(Keyword::If) ||
            before.matches_keyword(Keyword::For) ||
            before.matches_keyword(Keyword::While) ||
            before.matches_keyword(Keyword::With)
        } else {
            true
        }
    }

    fn check_for_func(&self) -> bool {
        if let Some(before) = self.nth_before_last_open_paren(1) {
            if before.is_ident() {
                if let Some(three_before) = self.nth_before_last_open_paren(3) {
                    return Self::check_for_expression(three_before)
                }
            } else if before.matches_keyword(Keyword::Function) {
                if let Some(two_before) = self.nth_before_last_open_paren(2) {
                    return Self::check_for_expression(two_before)
                } else {
                    return false;
                }
            }
        }
        true
    }

    fn check_for_expression(token: Token) -> bool {
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
            return None
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
}

pub(crate) fn escaped<I>(q: char) -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    c_char('\\')
        .and(c_char(q))
        .map(|(_slash, c): (char, char)| c)
}

pub mod error {
    #[derive(Debug)]
    pub enum Error {
        DataMismatch(String),
    }

    impl ::std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
            match self {
                &Error::DataMismatch(ref msg) => msg.fmt(f)
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
        let expectation = vec![
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
        for test in s.zip(expectation.into_iter()) {
            assert_eq!(test.0.token, test.1);
        }
    }
}