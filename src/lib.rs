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
pub use tokens::{Token, Item, BooleanLiteral as Boolean};

/// Send over the complete text and get back
/// the completely parsed result
pub fn tokenize(text: &str) -> Vec<Token> {
    tokens::tokens().easy_parse(text).expect("failed to tokenize text").0
}

/// An iterator over a token stream built
/// from raw js text
pub struct Scanner {
    stream: String,
    eof: bool,
    cursor: usize,
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
                if pair.0 == Token::EoF {
                    self.eof = true;
                }
                let new_cursor = self.stream.len() - pair.1.trim_left().len();
                let ret = Item::new(pair.0, self.cursor, new_cursor);
                self.cursor = new_cursor;
                Some(ret)
            }
            Err(e) => {
                eprintln!(
                    "Failed to parse token last successful parse ended {}\nError: {:?}",
                    self.cursor, e,
                );
                panic!()
            }
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