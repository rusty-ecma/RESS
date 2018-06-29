extern crate combine;
use combine::{Parser};
pub mod tokens;

use tokens::{Token, tokens, token};

pub fn tokenize(text: &str) -> Result<Vec<Token>, String> {
    match tokens().parse(text.trim()) {
        Ok(t) => {
            println!("tokens: {:?}", t);
            Ok(t.0)
        },
        Err(e) => {
            println!("error parsing js {:?}", e);
            Err(format!("{:?}", e))
        }
    }
}

pub struct Scanner {
    stream: String
}

impl Scanner {
    pub fn new(text: impl Into<String>) -> Self {
        Scanner {
            stream: text.into().trim().to_owned(),
        }
    }
}

impl Iterator for Scanner {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let (ret, new_stream) = if self.stream.len() == 0 {
            (None, self.stream.clone())
        } else {
            match token().parse(self.stream.as_str()) {
                Ok(pair) => {
                    (Some(pair.0), pair.1.to_string())
                },
                Err(_) => return None //FIXME: what do we do here?
            }
        };
        self.stream = new_stream.to_string();
        ret
    }
}

#[cfg(test)]
mod test {
    use super::{tokenize, Token};
    #[test]
    fn file() {
        let js = "
'use strict';
function thing() {
    let x = 0;
    console.log('stuff');
}
        ";
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
        ];
        let toks = tokenize(js).unwrap();
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
            Token::Ident("this".into()),
            Token::Punct(".".into()),
            Token::Ident("x".into()),
            Token::Punct("=".into()),
            Token::Numeric("100".into()),
            Token::Punct(";".into()),
            Token::Ident("this".into()),
            Token::Punct(".".into()),
            Token::Ident("y".into()),
            Token::Punct("=".into()),
            Token::Numeric("0".into()),
            Token::Punct(";".into()),
            Token::Punct("}".into()),
            Token::Punct(")".into()),
            Token::Punct("(".into()),
            Token::Punct(")".into()),
        ];
        for test in s.zip(expectation.into_iter()) {
            assert_eq!(test.0, test.1);
        }
    }
}