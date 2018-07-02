extern crate combine;
use combine::{Parser};
pub mod regex;
pub mod tokens;
pub mod unicode;
use tokens::{Token, token};

pub fn tokenize(text: &str) -> Vec<Token> {
    Scanner::new(text).collect()
}

pub struct Scanner {
    stream: String,
    tokens: Vec<Token>,
    eof: bool
}

impl Scanner {
    pub fn new(text: impl Into<String>) -> Self {
        Scanner {
            stream: text.into().trim().to_owned(),
            tokens: vec![],
            eof: false,
        }
    }
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
                let mut last_100 = self.stream.clone();
                last_100.truncate(100);
                eprintln!("Failed to parse token, parsed: {:?}\nstream: \n{}{}", self.tokens, last_100, trailer);
                panic!() //FIXME: what do we do here?
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