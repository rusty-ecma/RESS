//! js_parse
//! A crate for parsing raw JS into a token stream
extern crate combine;
use combine::{Parser, Stream, parser::char::char as c_char, error::ParseError};
mod regex;
mod strings;
mod tokens;
mod unicode;
use tokens::token;
pub use tokens::Token;
/// Send over the complete text and get back
/// the completely parsed result
pub fn tokenize(text: &str) -> Vec<Token> {
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
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        if self.eof {
            return None;
        };
        match token().easy_parse(&self.stream[self.cursor..]) {
            Ok(pair) => {
                if pair.0 == Token::EoF {
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

#[cfg(test)]
mod test {
    use super::{tokenize, Token};
    #[test]
    fn tokenizer() {
        let js = "
'use strict';
function thing() {
    let x = 0;
    console.log('stuff');
}";
        let expectation = vec![
            Token::String("use strict".into()),
            Token::Punct(";".into()),
            Token::Keyword("function".into()),
            Token::Ident("thing".into()),
            Token::Punct("(".into()),
            Token::Punct(")".into()),
            Token::Punct("{".into()),
            Token::Keyword("let".into()),
            Token::Ident("x".into()),
            Token::Punct("=".into()),
            Token::Numeric("0".into()),
            Token::Punct(";".into()),
            Token::Ident("console".into()),
            Token::Punct(".".into()),
            Token::Ident("log".into()),
            Token::Punct("(".into()),
            Token::String("stuff".into()),
            Token::Punct(")".into()),
            Token::Punct(";".into()),
            Token::Punct("}".into()),
            Token::EoF,
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
            Token::Punct("(".into()),
            Token::Keyword("function".into()),
            Token::Punct("(".into()),
            Token::Punct(")".into()),
            Token::Punct("{".into()),
            Token::Keyword("this".into()),
            Token::Punct(".".into()),
            Token::Ident("x".into()),
            Token::Punct("=".into()),
            Token::Numeric("100".into()),
            Token::Punct(";".into()),
            Token::Keyword("this".into()),
            Token::Punct(".".into()),
            Token::Ident("y".into()),
            Token::Punct("=".into()),
            Token::Numeric("0".into()),
            Token::Punct(";".into()),
            Token::Punct("}".into()),
            Token::Punct(")".into()),
            Token::Punct("(".into()),
            Token::Punct(")".into()),
            Token::Punct(";".into()),
            Token::EoF,
        ];
        for test in s.zip(expectation.into_iter()) {
            assert_eq!(test.0, test.1);
        }
    }
}