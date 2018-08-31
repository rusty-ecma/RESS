use combine::{
    choice, error::ParseError, many, many1, optional,
    parser::char::{char as c_char, digit, hex_digit, oct_digit}, try, Parser, Stream,
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
        optional((choice((c_char('e'), c_char('E'))), many1(digit()))),
    ).map(
        |(integer, remainder, exponent): (
            String,
            Option<(char, String)>,
            Option<(char, String)>,
        )| {
            let mut ret = String::new();

            ret.push_str(&integer);
            if let Some((p, r)) = remainder {
                ret.push(p);
                ret.push_str(&r);
            }
            if let Some((e, ex)) = exponent {
                ret.push(e);
                ret.push_str(&ex);
            }
            Number(ret)
        },
    )
}

fn no_leading_decimal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        optional(choice([c_char('-'), c_char('+')])),
        c_char('.'),
        many1(digit()),
        optional((choice([c_char('e'), c_char('E')]), many1(digit()))),
    ).map(
        |(sign, dot, remainder, exponent): (Option<char>, char, String, Option<(char, String)>)| {
            let mut ret = String::new();
            if let Some(sign) = sign {
                ret.push(sign);
            }
            ret.push(dot);
            ret.push_str(&remainder);
            if let Some((e, ex)) = exponent {
                ret.push(e);
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
        optional(choice([c_char('-'), c_char('+')])),
        c_char('0'),
        choice([c_char('x'), c_char('X')]),
        many1(hex_digit()),
    ).map(
        |(sign, zero, x, integer): (Option<char>, char, char, String)| {
            let mut ret = String::new();
            if let Some(sign) = sign {
                ret.push(sign);
            }
            ret.push(zero);
            ret.push(x);
            ret.push_str(&integer);
            Number(ret)
        },
    )
}

fn bin_literal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        optional(choice([c_char('-'), c_char('+')])),
        c_char('0'),
        choice([c_char('b'), c_char('B')]),
        many1(choice([c_char('1'), c_char('0')])),
    ).map(
        |(sign, zero, b, integer): (Option<char>, char, char, String)| {
            let mut ret = String::new();
            if let Some(sign) = sign {
                ret.push(sign);
            }
            ret.push(zero);
            ret.push(b);
            ret.push_str(&integer);
            Number(ret)
        },
    )
}

fn octal_literal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        optional(choice([c_char('-'), c_char('+')])),
        c_char('0'),
        choice([c_char('o'), c_char('O')]),
        many1(oct_digit()),
    ).map(
        |(sign, zero, o, integer): (Option<char>, char, char, String)| {
            let mut ret = String::new();
            if let Some(sign) = sign {
                ret.push(sign);
            }
            ret.push(zero);
            ret.push(o);
            ret.push_str(&integer);
            Number(ret)
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use tokens;
    #[test]
    fn full_decimal() {
        let vals = vec![
            "0.1",
            "1.1",
            "888888888.88888888888",
            "1.8876e2",
        ];
        for val in vals {
            let d = tokens::token().parse(val.clone()).unwrap();
            assert_eq!(d, (Token::Numeric(Number::from(val)), ""));
        }
        if let Ok(_) = full_decimal_literal().parse(".00") {
            panic!("parsed .00 as full decimal literal");
        }
    }

    #[test]
    fn no_leading() {
        let vals = vec![
            ".2", "-.2", ".2E1", "+.8", "+.2E4", ".7e34", "-.7e2", "+.4e5",
        ];
        for val in vals {
            let d = tokens::token().parse(val.clone()).unwrap();
            assert_eq!(d, (Token::Numeric(Number::from(val)), ""))
        }
        if let Ok(_) = no_leading_decimal().parse("00.0") {
            panic!("parsed 00.0 as no leading decimal")
        }
    }

    #[test]
    fn hex() {
        let vals = vec![
            "0x123", "0X456", "0xdef", "0xABC", "0xDEF",
        ];
        for val in vals {
            let h = tokens::token().parse(val.clone()).unwrap();
            assert_eq!(h, (Token::Numeric(Number::from(val)), ""))
        }

        if let Ok(_) = hex_literal().parse("555") {
            panic!("parsed 555 as hex literal")
        }
    }
    #[test]
    fn bin() {
        let vals = vec!["0b000", "0B111"];
        for val in vals {
            let h = tokens::token().parse(val.clone()).unwrap();
            assert_eq!(h, (Token::Numeric(Number::from(val)), ""))
        }

        if let Ok(_) = bin_literal().parse("0b") {
            panic!("parsed 0b as hex literal")
        }
    }

    #[test]
    fn oct() {
        let vals = vec!["0o7", "0O554"];
        for val in vals {
            let h = tokens::token().parse(val.clone()).unwrap();
            assert_eq!(h, (Token::Numeric(Number::from(val)), ""))
        }

        if let Ok(_) = octal_literal().parse("0O8") {
            panic!("parsed 0O8 as octal literal")
        }
    }

    #[test]
    fn decimal() {
        let vals = vec![
            "0.1",
            "1.1",
            "888888888.88888888888",
            "1.8876e2",
            ".2E1",
            ".7e34",
        ];
        for val in vals {
            let d = tokens::token().parse(val.clone()).unwrap();
            assert_eq!(d, (Token::Numeric(Number::from(val)), ""));
        }
        if let Ok(f) = tokens::token().parse("asdfghjk") {
            match f {
                (Token::Numeric(d), _) => panic!("parsed asdfghjk as decimal {:?}", d),
                _ => (),
            }
        }
    }

    #[test]
    fn number() {
        let vals = vec![
            "0.1",
            "1.1",
            "888888888.88888888888",
            "1.8876e2",
            ".2E1",
            ".7e34",
            "0x123",
            "0X456",
            "0xdef",
            "0xABC",
            "0xDEF",
            "0o7",
            "0O554",
            "0b000",
            "0B111",
        ];
        for val in vals {
            let d = tokens::token().parse(val.clone()).unwrap();
            assert_eq!(d, (Token::Numeric(Number::from(val)), ""));
        }
        match tokens::token().parse("asdfghjk").unwrap() {
            (Token::Numeric(f), "") => panic!("parsed asdfghjk as number {:?}", f),
            _ => (),
        }
    }

    proptest! {
        #[test]
        fn normal_decimal(s in r#"((0[oO][0-7]+)|(0[xX][0-9a-fA-F]+)|(0[bB][01]+)|(([0-9]+)(\.[0-9]+)?([eE][0-9]+)?)|((\.[0-9])([eE][0-9]+)?))"#) {
            let r = tokens::token().easy_parse(s.as_str()).unwrap();
            assert!(r.0.is_numeric() && r.0.matches_numeric_str(&s))
        }
    }
}
