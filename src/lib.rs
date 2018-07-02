//! js_parse
//! A crate for parsing raw JS into a token stream
extern crate combine;
use combine::{Parser};
mod regex;
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
    tokens: Vec<Token>,
    eof: bool
}

impl Scanner {
    /// Create a new Scanner with the raw JS text
    pub fn new(text: impl Into<String>) -> Self {
        Scanner {
            stream: text.into().trim().to_owned(),
            tokens: vec![],
            eof: false,
        }
    }

    //TODO: Implement construction from a reader
}

impl Iterator for Scanner {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        if self.eof {
            return None
        };
        let new_stream = match token().parse(self.stream.as_str()) {
            Ok(pair) => {
                if pair.0 == Token::EoF {
                    self.eof = true;
                }
                self.tokens.push(pair.0);
                pair.1.to_string()
            },
            Err(_) => {
                let trailer = if self.stream.len() <= 100 {
                    ""
                } else {
                    "..."
                };
                let mut next_100 = self.stream.clone();
                next_100.truncate(100);
                let count = if self.tokens.len() >= 10 {
                    10
                } else {
                    self.tokens.len()
                };
                let last = &self.tokens.clone().into_iter().rev().collect::<Vec<Token>>()[0..count];
                eprintln!("Failed to parse token, last: {:?}\nnext: \n{}{}", last, next_100, trailer);
                panic!()
            }
        };
        self.stream = new_stream.trim_left().to_string();
        self.tokens.get(self.tokens.len() - 1).cloned()
    }
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
        let s = super::Scanner::new("(function() {
this.x = 100;
this.y = 0;
})();");
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
            Token::EoF
        ];
        for test in s.zip(expectation.into_iter()) {
            assert_eq!(test.0, test.1);
        }
    }
}