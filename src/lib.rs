//! js_parse
//! A crate for parsing raw JS into a token stream
#[macro_use]
extern crate combine;
use combine::{Parser, Stream, parser::char::char as c_char, error::ParseError};
mod regex;
mod strings;
mod tokens;
mod unicode;
mod numeric;
mod punct;
mod keywords;
mod comments;
pub use tokens::{TokenData, Token, BooleanLiteral};
pub use punct::Token as Punct;
pub use keywords::Token as Keyword;
pub use numeric::Token as Number;

/// Send over the complete text and get back
/// the completely parsed result
pub fn tokenize(text: &str) -> Vec<TokenData> {
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
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        if self.eof {
            return None;
        };
        match tokens::token().easy_parse(&self.stream[self.cursor..]) {
            Ok(pair) => {
                if pair.0 == TokenData::EoF {
                    self.eof = true;
                }
                let new_cursor = self.stream.len() - pair.1.trim_left().len();
                let ret = Token::new(pair.0, self.cursor, new_cursor);
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

mod error {
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
            TokenData::single_quoted_string("use strict"),
            TokenData::punct(";"),
            TokenData::keyword("function"),
            TokenData::ident("thing"),
            TokenData::punct("("),
            TokenData::punct(")"),
            TokenData::punct("{"),
            TokenData::keyword("let"),
            TokenData::ident("x"),
            TokenData::punct("="),
            TokenData::numeric("0"),
            TokenData::punct(";"),
            TokenData::ident("console"),
            TokenData::punct("."),
            TokenData::ident("log"),
            TokenData::punct("("),
            TokenData::single_quoted_string("stuff"),
            TokenData::punct(")"),
            TokenData::punct(";"),
            TokenData::punct("}"),
            TokenData::EoF,
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
            TokenData::punct("("),
            TokenData::keyword("function"),
            TokenData::punct("("),
            TokenData::punct(")"),
            TokenData::punct("{"),
            TokenData::keyword("this"),
            TokenData::punct("."),
            TokenData::ident("x"),
            TokenData::punct("="),
            TokenData::numeric("100"),
            TokenData::punct(";"),
            TokenData::keyword("this"),
            TokenData::punct("."),
            TokenData::ident("y"),
            TokenData::punct("="),
            TokenData::numeric("0"),
            TokenData::punct(";"),
            TokenData::punct("}"),
            TokenData::punct(")"),
            TokenData::punct("("),
            TokenData::punct(")"),
            TokenData::punct(";"),
            TokenData::EoF,
        ];
        for test in s.zip(expectation.into_iter()) {
            assert_eq!(test.0.data, test.1);
        }
    }
}