use combine::{
    choice,
    error::ParseError,
    many, many1, optional,
    parser::char::{char as c_char, digit, hex_digit, oct_digit},
    attempt, Parser, Stream,
};

use tokens::Token;
#[derive(Debug, PartialEq, Clone)]
pub struct Number(String);

impl Number {
    pub fn kind(&self) -> Kind {
        if self.0.starts_with("0x") {
            Kind::Hex
        } else if self.0.starts_with("0b") {
            Kind::Bin
        } else if self.0.starts_with("0o") {
            Kind::Octal
        } else {
            Kind::Decimal
        }
    }

    pub fn is_hex(&self) -> bool {
        self.kind() == Kind::Hex
    }
    pub fn is_bin(&self) -> bool {
        self.kind() == Kind::Bin
    }
    pub fn is_oct(&self) -> bool {
        self.kind() == Kind::Octal
    }
    pub fn is_dec(&self) -> bool {
        self.kind() == Kind::Decimal
    }
    pub fn has_exponent(&self) -> bool {
        match self.kind() {
            Kind::Decimal => self.0.contains(|c| c == 'e' || c == 'E'),
            _ => false,
        }
    }
}

impl From<String> for Number {
    fn from(s: String) -> Self {
        Number(s)
    }
}

impl<'a> From<&'a str> for Number {
    fn from(s: &'a str) -> Self {
        Number(s.into())
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Kind {
    Decimal,
    Hex,
    Bin,
    Octal,
}

pub fn literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(non_decimal()),
        attempt(decimal_literal()),
    )).map(super::Token::Numeric)
}

fn decimal_literal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((attempt(full_decimal_literal()), attempt(no_leading_decimal()))).map(|t| t)
}

fn full_decimal_literal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        //any number of digits
        many1(digit()),
        //optionally followed by a . and any number of digits
        optional((c_char('.'), many(digit()))),
        //optionally followed by e|E and any number of digits
        optional(exponent()),
    )
        .map(
            |(integer, remainder, exponent): (String, Option<(char, String)>, Option<String>)| {
                let remainder = if let Some((_, remainder)) = remainder {
                    format!(".{}", remainder)
                } else {
                    String::new()
                };
                let exponent = if let Some(exp) = exponent {
                    format!("{}", exp)
                } else {
                    String::new()
                };
                Number(format!("{integer}{remainder}{exponent}",
                    integer=integer,
                    remainder=remainder,
                    exponent=exponent,
                ))
            },
        )
}

fn exponent<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        choice([c_char('e'), c_char('E')]),
        optional(choice([c_char('-'), c_char('+')])),
        many1(digit()),
    )
        .map(|(e, sign, value): (char, Option<char>, String)| {
            let sign = if let Some(sign) = sign {
                sign.to_string()
            } else {
                String::new()
            };
            format!("{}{}{}", e, sign, value)
        })
}

fn no_leading_decimal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (c_char('.'), many1(digit()), optional(exponent())).map(
        |(_, remainder, exponent): (_, String, Option<String>)| {
            let ex = if let Some(ex) = exponent {
                ex
            } else {
                String::new()
            };
            Number(format!(".{}{}", remainder, ex))
        },
    )
}

pub fn non_decimal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(hex_literal()),
        attempt(octal_literal()),
        attempt(bin_literal())
    )).map(|(kind, integer): (char, String)| Number(format!("0{}{}", kind, integer)))
}

fn hex_literal<I>() -> impl Parser<Input = I, Output = (char, String)>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        c_char('0'),
        choice([c_char('x'), c_char('X')]),
        many1(hex_digit()),
    )
        .map(|(_, x, integer): (_, char, String)| (x, integer))
}

fn bin_literal<I>() -> impl Parser<Input = I, Output = (char, String)>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        c_char('0'),
        choice([c_char('b'), c_char('B')]),
        many1(choice([c_char('1'), c_char('0')])),
    )
        .map(|(_, b, integer): (_, char, String)| (b, integer))
}

fn octal_literal<I>() -> impl Parser<Input = I, Output = (char, String)>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        c_char('0'),
        choice([c_char('o'), c_char('O')]),
        many1(oct_digit()),
    )
        .map(|(_, o, integer): (_, char, String)| (o, integer))
}

#[cfg(test)]
mod test {
    use super::*;
    use tokens;

    proptest! {
        #[test]
        fn normal_decimal(s in r#"((0[oO][0-7]+)|(0[xX][0-9a-fA-F]+)|(0[bB][01]+)|(([0-9]+)(\.[0-9]+)?([eE]([-+])?[0-9]+)?)|((\.[0-9])([eE]([-+])?[0-9]+)?))"#) {
            let r = tokens::token().easy_parse(s.as_str()).unwrap();
            assert!(r.0.is_numeric() && r.0.matches_numeric_str(&s))
        }
    }

    proptest! {
        #[test]
        fn fail_non_number(s in r#"[^0-9]+"#) {
            if let Ok((t, _)) = tokens::token().parse(s.as_str()) {
                if let tokens::Token::Numeric(_) = t {
                    panic!("Parsed non-number as number")
                }
            }
        }
    }
}
