use unic_ucd_ident::{is_id_continue, is_id_start};

mod buffer;

use crate::{
    refs::{
        RefToken as Token,
        tokens::StringLit,
    },
    Punct,
};
pub struct RawToken {
    ty: Token,
    start: usize,
    end: usize,
}

pub struct Tokenizer<'a> {
    stream: buffer::JSBuffer<'a>,
    current_start: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(stream: &'a str) -> Self {
        Tokenizer {
            current_start: 0,
            stream: stream.into()
        }
    }

    pub fn next_(&mut self) -> RawToken {
        self.current_start = self.stream.idx;
        let next_char = match self.stream.next_char() {
            Some(ch) => ch,
            None => return RawToken {
                start: self.stream.idx,
                end: self.stream.idx,
                ty: Token::EoF
            }
        };
        if is_id_start(next_char) {
            return self.ident();
        }
        if next_char == '"' || next_char == '\'' {
            return self.string(next_char);
        }
        if next_char == '(' || next_char == ')' || next_char == ';' {
            return self.punct(next_char);
        }
        self.punct(next_char)
    }

    fn ident(&mut self) -> RawToken {
        unimplemented!()
    }

    fn string(&mut self, quote: char) -> RawToken {
        let mut escaped = false;
        loop {
            // TODO: add new line validation (escaped only)
            if self.look_ahead_matches(r#"\"#) {
                if escaped {
                    escaped = false;
                } else {
                    escaped = true;
                }
                self.stream.skip(1)
            } else if self.stream.look_ahead_matches(&[quote as u8]) {
                self.stream.skip(1);
                if !escaped {
                    break;
                }
                escaped = false;
            } else {
                self.stream.skip(1);
                escaped = false;
            }
        }
        let inner = if quote == '"' {
            StringLit::Double
        } else {
            StringLit::Single
        };
        self.gen_token(Token::String(inner))
    }
    fn punct(&mut self, c: char) -> RawToken {
        match c {
            '(' => self.gen_punct(Punct::OpenParen),
            '{' => self.gen_punct(Punct::OpenBrace),
            ')' => self.gen_punct(Punct::CloseParen),
            '}' => self.gen_punct(Punct::CloseBrace),
            ';' => self.gen_punct(Punct::SemiColon),
            ',' => self.gen_punct(Punct::Comma),
            '[' => self.gen_punct(Punct::OpenBracket),
            ']' => self.gen_punct(Punct::CloseBracket),
            ':' => self.gen_punct(Punct::Colon),
            '?' => self.gen_punct(Punct::QuestionMark),
            '#' => self.gen_punct(Punct::Private),
            '~' => self.gen_punct(Punct::BitwiseNot),
            '.' => {
                // ...
                if self.look_ahead_matches("..") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::Spread)
                } else {
                    self.gen_punct(Punct::Period)
                }
            },
            '>' => {
                if self.look_ahead_matches(">>=") {
                    self.stream.skip(3);
                    self.gen_punct(Punct::UnsignedRightShiftAssign)
                } else if self.look_ahead_matches(">>")  {
                    self.stream.skip(2);
                    self.gen_punct(Punct::UnsignedRightShift)
                } else if self.look_ahead_matches(">=") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::RightShiftAssign)
                } else if self.look_ahead_matches(">") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::RightShift)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::GreaterThanEqual)
                } else {
                    self.gen_punct(Punct::GreaterThan)
                }
            },
            '<' => {
                if self.look_ahead_matches("<=") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::LeftShiftAssign)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::LessThanEqual)
                } else if self.look_ahead_matches("<") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::LeftShift)
                } else {
                    self.gen_punct(Punct::LessThan)
                }
            },
            '=' => {
                if self.look_ahead_matches("==") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::StrictEquals)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::Equal)
                } else if self.look_ahead_matches(">") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::FatArrow)
                } else {
                    self.gen_punct(Punct::Assign)
                }
            },
            '!' => {
                if self.look_ahead_matches("==") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::StrictNotEquals)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::NotEqual)
                } else {
                    self.gen_punct(Punct::Not)
                }
            },
            '*' => {
                if self.look_ahead_matches("*=") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::ExponentAssign)
                } else if self.look_ahead_matches("*") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::Exponent)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::MultiplyAssign)
                } else {
                    self.gen_punct(Punct::Asterisk)
                }
            },
            '&' => {
                if self.look_ahead_matches("&") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::LogicalAnd)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::BitwiseAndAssign)
                } else {
                    self.gen_punct(Punct::And)
                }
            },
            '|' => {
                if self.look_ahead_matches("|") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::LogicalOr)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::BitwiseOrAssign)
                } else {
                    self.gen_punct(Punct::Pipe)
                }
            },
            '+' => {
                if self.look_ahead_matches("+") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::Increment)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::AddAssign)
                } else {
                    self.gen_punct(Punct::Plus)
                }
            },
            '-' => {
                if self.look_ahead_matches("-") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::Decrement)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::SubtractAssign)
                } else {
                    self.gen_punct(Punct::Minus)
                }
            },
            '/' => {
                if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::DivideAssign)
                } else {
                    self.gen_punct(Punct::ForwardSlash)
                }
            },
            '%' => {
                if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::ModuloAssign)
                } else {
                    self.gen_punct(Punct::Modulo)
                }
            },
            '^' => {
                if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::BitwiseXOrAssign)
                } else {
                    self.gen_punct(Punct::Caret)
                }
            }
            _ => unimplemented!()
        }
    }
    fn look_ahead_matches(&self, s: &str) -> bool {
        self.stream.look_ahead_matches(s.as_bytes())
    }
    fn gen_punct(&self, p: Punct) -> RawToken {
        self.gen_token(Token::Punct(p))
    }
    fn gen_token(&self, ty: Token) -> RawToken {
        RawToken {
            start: self.current_start,
            end: self.stream.idx,
            ty,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static STRINGS: &[&str] = &[
    r#""things and stuff""#,
    r#"'people and places'"#,
    r#""with and escaped \"""#,
    r#"'another escaped \''"#,
    r#""with a new \
line""#,
    r#"'another new line \
hahaha'"#,
    "\"sequence double quoted\\\r\nis hard\"",
    "'new line sequence\\\r\nmight be harder'",
    ];
    #[test]
    fn tokenizer_strings() {
        for s in STRINGS {
            let item = Tokenizer::new(s).next_();
            match &item.ty {
                Token::String(ref lit) => {
                    if &s[0..1] == "'" {
                        match lit {
                            StringLit::Single => (),
                            StringLit::Double => panic!("Expected single quote string, found double"),
                        }
                    } else {
                        match lit {
                            StringLit::Single => panic!("expected double quote string, found single"),
                            StringLit::Double => (),
                        }
                    }
                },
                _ => panic!("Expected string, found {:?}", item.ty),
            }
            assert_eq!(s.len(), item.end - item.start);
            
        }
    }
}