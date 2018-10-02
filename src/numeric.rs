use combine::{
    choice,
    error::ParseError,
    many, many1, optional,
    parser::char::{char as c_char, digit, hex_digit, oct_digit},
    try, Parser, Stream,
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

#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
    Decimal,
    Hex,
    Bin,
    Octal,
}

pub(crate) fn literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(bin_literal()),
        try(octal_literal()),
        try(hex_literal()),
        try(decimal_literal()),
    )).map(|t: Number| super::Token::Numeric(t))
}

fn decimal_literal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(full_decimal_literal()), try(no_leading_decimal()))).map(|t| t)
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
            |(integer, remainder, exponent): (
                String,
                Option<(char, String)>,
                Option<String>,
            )| {
                let mut ret = String::new();
                ret.push_str(&integer);
                if let Some((p, r)) = remainder {
                    ret.push(p);
                    ret.push_str(&r);
                }
                if let Some(ex) = exponent {
                    ret.push_str(&ex);
                }
                Number(ret)
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
        optional(
            choice([c_char('-'), c_char('+')])
        ),
        many1(digit())
    ).map(|(e, sign, value): (char, Option<char>, String)| {
        let mut ret = e.to_string();
        if let Some(sign) = sign {
            ret.push(sign)
        }
        ret.push_str(&value);
        ret
    })
}

fn no_leading_decimal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        c_char('.'),
        many1(digit()),
        optional(exponent()),
    )
        .map(
            |(dot, remainder, exponent): (char, String, Option<String>)| {
                let mut ret = String::new();
                ret.push(dot);
                ret.push_str(&remainder);
                if let Some(ex) = exponent {
                    ret.push_str(&ex);
                }
                Number(ret)
            },
        )
}

fn hex_literal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        c_char('0'),
        choice([c_char('x'), c_char('X')]),
        many1(hex_digit()),
    )
        .map(|(zero, x, integer): (char, char, String)| {
            let mut ret = format!("{}", zero);
            ret.push(x);
            ret.push_str(&integer);
            Number(ret)
        })
}

fn bin_literal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        c_char('0'),
        choice([c_char('b'), c_char('B')]),
        many1(choice([c_char('1'), c_char('0')])),
    )
        .map(|(zero, b, integer): (char, char, String)| {
            let mut ret = String::new();
            ret.push(zero);
            ret.push(b);
            ret.push_str(&integer);
            Number(ret)
        })
}

fn octal_literal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        c_char('0'),
        choice([c_char('o'), c_char('O')]),
        many1(oct_digit()),
    )
        .map(|(zero, o, integer): (char, char, String)| {
            let mut ret = String::new();
            ret.push(zero);
            ret.push(o);
            ret.push_str(&integer);
            Number(ret)
        })
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
