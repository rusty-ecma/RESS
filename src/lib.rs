//! js_parse
//! A crate for parsing raw JS into a token stream
extern crate combine;
use combine::{Parser, Stream, parser::char::char as c_char, error::ParseError};
mod regex;
mod strings;
mod tokens;
mod unicode;
mod numeric;
use tokens::token;
pub use tokens::TokenData;
/// Send over the complete text and get back
/// the completely parsed result
pub fn tokenize(text: &str) -> Vec<TokenData> {
    Scanner::new(text).collect()
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
        Scanner {
            stream: text.into().trim().to_owned(),
            eof: false,
            cursor: 0
        }
    }

    //TODO: Implement construction from a reader
}

impl Iterator for Scanner {
    type Item = TokenData;
    fn next(&mut self) -> Option<TokenData> {
        if self.eof {
            return None;
        };
        match token().easy_parse(&self.stream[self.cursor..]) {
            Ok(pair) => {
                if pair.0 == TokenData::EoF {
                    self.eof = true;
                }
                self.cursor = self.stream.len() - pair.1.trim().len();
                Some(pair.0)
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
    use super::{tokenize, TokenData};
    #[test]
    fn tokenizer() {
        let js = "
'use strict';
function thing() {
    let x = 0;
    console.log('stuff');
}";
        let expectation = vec![
            TokenData::String("use strict".into()),
            TokenData::Punct(";".into()),
            TokenData::Keyword("function".into()),
            TokenData::Ident("thing".into()),
            TokenData::Punct("(".into()),
            TokenData::Punct(")".into()),
            TokenData::Punct("{".into()),
            TokenData::Keyword("let".into()),
            TokenData::Ident("x".into()),
            TokenData::Punct("=".into()),
            TokenData::Numeric("0".into()),
            TokenData::Punct(";".into()),
            TokenData::Ident("console".into()),
            TokenData::Punct(".".into()),
            TokenData::Ident("log".into()),
            TokenData::Punct("(".into()),
            TokenData::String("stuff".into()),
            TokenData::Punct(")".into()),
            TokenData::Punct(";".into()),
            TokenData::Punct("}".into()),
            TokenData::EoF,
        ];
        let toks = tokenize(js);
        assert_eq!(toks, expectation);
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
            TokenData::Punct("(".into()),
            TokenData::Keyword("function".into()),
            TokenData::Punct("(".into()),
            TokenData::Punct(")".into()),
            TokenData::Punct("{".into()),
            TokenData::Keyword("this".into()),
            TokenData::Punct(".".into()),
            TokenData::Ident("x".into()),
            TokenData::Punct("=".into()),
            TokenData::Numeric("100".into()),
            TokenData::Punct(";".into()),
            TokenData::Keyword("this".into()),
            TokenData::Punct(".".into()),
            TokenData::Ident("y".into()),
            TokenData::Punct("=".into()),
            TokenData::Numeric("0".into()),
            TokenData::Punct(";".into()),
            TokenData::Punct("}".into()),
            TokenData::Punct(")".into()),
            TokenData::Punct("(".into()),
            TokenData::Punct(")".into()),
            TokenData::Punct(";".into()),
            TokenData::EoF,
        ];
        for test in s.zip(expectation.into_iter()) {
            assert_eq!(test.0, test.1);
        }
    }
}