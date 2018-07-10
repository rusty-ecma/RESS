use std::str::FromStr;
use combine::{
    choice, error::ParseError, many, many1, optional,
    parser::{
        char::{char as c_char, digit, hex_digit, oct_digit},
    },
    try, Parser, Stream,
};
use tokens::{TokenData};
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    sign: Sign,
    integer: Option<usize>,
    remainder: Option<usize>,
    char_case: Option<CharCase>,
    exponent: Option<usize>,
    kind: NumericKind
}

impl Token {
    pub fn from_parts(sign: Option<char>, integer: Option<&str>, remainder: Option<&str>, char_case: Option<char>, exponent: Option<&str>, kind: NumericKind) -> Result<Self, super::error::Error> {
        let sign = if let Some(s) = sign {
            Sign::from(s)
        } else {
            Sign::Positive
        };
        let integer = if let Some(i) = integer {
            let n: usize = i.parse()?;
            Some(n)
        } else {
            None
        };
        let remainder = if let Some(r) = remainder {
            let n: usize = r.parse()?;
            Some(n)
        } else {
            None
        };
        let char_case = if let Some(c) = char_case {
            Some(Self::parse_char_case(c)?)
        } else {
            None
        };
        let exponent = if let Some(e) = exponent {
            let n: usize = e.parse()?;
            Some(n)
        } else {
            None
        };
        Ok(Token {
            sign,
            integer,
            remainder,
            char_case,
            exponent,
            kind,
        })
    }

    fn parse_integer(s: &str, kind: NumericKind) -> Result<usize, super::error::Error> {
        let radix = match kind {
            NumericKind::Decimal => 10,
            NumericKind::Bin => 2,
            NumericKind::Hex => 16,
            NumericKind::Octal => 8,
        };
        Ok(usize::from_str_radix(s, radix)?)
    }

    fn parse_char_case(c: char) -> Result<CharCase, super::error::Error> {
        let small = c.to_lowercase();
        if c != 'e' && c != 'b' && c != 'x' && c != 'o' {
            return Err(super::error::Error::DataMismatch(format!("Expected e b x or o found {}", c)))
        }
        Ok(CharCase::from(c))
    }
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl<'a> From<&'a str> for Token {
    fn from(s: &'a str) -> Self {
        let (sign, s) = if s.starts_with('-') ||  s.starts_with('+') {
            (Some(
                char::from_str(&s[0..1]).unwrap()
            ), &s[1..])
        } else {
            (None, s)
        };
        let res = if s.starts_with("0x") {
            Self::from_parts(sign, Some(&s[2..]), None, Some('x'), None, NumericKind::Hex)
        } else if s.starts_with("0b") {
            Self::from_parts(sign, Some(&s[2..]), None, Some('b'), None, NumericKind::Bin)
        } else if s.starts_with("0o") {
            Self::from_parts(sign, Some(&s[2..]), None, Some('o'), None, NumericKind::Octal)
        } else  {
            let mut first_split = s.split('.');
            let integer = if let Some(i) = first_split.next() {
                if i == "" {
                    None
                } else {
                    Some(i)
                }
            } else {
                None
            };
            let (r, ec, ex) = if let Some(remaining) = first_split.next() {
                let e = if remaining.contains('e') {
                    Some('e')
                } else if remaining.contains('E') {
                    Some('E')
                } else {
                    None
                };
                if let Some(split_char) = e {
                    let mut second_split = remaining.split(split_char);
                    let remainder = if let Some(r) = second_split.next() {
                        Some(r)
                    } else {
                        None
                    };
                    let exponent = if let Some(e) = second_split.next() {
                        Some(e)
                    } else {
                        None
                    };
                    (remainder, e, exponent)
                } else {
                    (Some(remaining), None, None)
                }
            } else {
                (None, None, None)
            };
            Self::from_parts(sign, integer, r, ec, ex, NumericKind::Decimal)
        };
        match res {
            Ok(t) => t,
            Err(e) => panic!("Error parsing numeric literal {}", e)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Sign {
    Positive,
    Negative,
}
#[derive(Debug, PartialEq, Clone)]
pub enum CharCase {
    Lower,
    Upper,
}

impl From<char> for CharCase {
    fn from(c: char) -> Self {
        if c.is_lowercase() {
            CharCase::Lower
        } else {
            CharCase::Upper
        }
    }
}

impl<'a> From<&'a str> for CharCase {
    fn from(s: &'a str) -> Self {
        if let Some(c) = s.chars().next() {
            Self::from(c)
        } else {
            panic!("Cannot create CharCase from string less than 1")
        }
    }
}

impl From<char> for Sign {
    fn from(c: char) -> Self {
        if c == '+' {
            Sign::Positive
        } else if c == '-' {
            Sign::Negative
        } else {
            panic!("Attempted to create a sign from {}, must be '+' or '-'")
        }
    }
}

impl<'a> From<&'a str> for Sign {
    fn from(s: &'a str) -> Self {
        if let Some(sign) = s.chars().next() {
            Self::from(sign)
        } else {
            panic!("Cannot create Sign from an empty str")
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum NumericKind {
    Decimal,
    Hex,
    Bin,
    Octal,
}

pub(crate) fn literal<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(bin_literal()),
        try(octal_literal()),
        try(hex_literal()),
        try(decimal_literal()),
    )).map(|t: Token| super::TokenData::Numeric(t))
}

fn decimal_literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice(
        (try(full_decimal_literal()),
        try(no_leading_decimal())
    )).map(|t| t)
}

fn full_decimal_literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        optional(choice([c_char('-'), c_char('+')])),
        //any number of digits
        many1(digit()),
        //optionally followed by a . and any number of digits
        optional((c_char('.'), many(digit()))),
        //optionally followed by e|E and any number of digits
        optional((choice((c_char('e'), c_char('E'))), many1(digit()))),
    ).map(
        |(sign, integer, remainder, exponent): (
            Option<char>,
            String,
            Option<(char, String)>,
            Option<(char, String)>,
        )| {

            let res = match (remainder, exponent) {
                (Some(r), Some(e)) => Token::from_parts(sign, Some(&integer), Some(r.1.as_str()), Some(e.0), Some(e.1.as_str()), NumericKind::Decimal),
                (None, Some(e)) => Token::from_parts(sign, Some(&integer), None, Some(e.0), Some(e.1.as_str()), NumericKind::Decimal),
                (Some(r), None) => Token::from_parts(sign, Some(&integer), Some(r.1.as_str()), None, None, NumericKind::Decimal),
                (None, None) => Token::from_parts(sign, Some(&integer), None, None, None, NumericKind::Decimal),
            };

            match res {
                Ok(t) => t,
                Err(e) => panic!("error parsing decimal literal {}", e)
            }
        },
    )
}

fn no_leading_decimal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        optional(choice([c_char('-'), c_char('+')])),
        c_char('.'),
        many1(digit()),
        optional((choice([c_char('e'), c_char('E')]), many1(digit()))),
    ).map(|(sign, _dot, remainder, exponent): (Option<char>, char, String, Option<(char, String)>)| {
        let res = if let Some(e) = exponent {
            Token::from_parts(sign, None, Some(remainder.as_str()), Some(e.0), Some(e.1.as_str()), NumericKind::Decimal)
        } else {
            Token::from_parts(sign, None, Some(remainder.as_str()), None, None, NumericKind::Decimal)
        };
        match  res {
            Ok(t) => t,
            Err(e) => panic!("Error parsing decimal literal {}", e)
        }

    })
}

fn hex_literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        optional(choice([c_char('-'), c_char('+')])),
        c_char('0'),
        choice([c_char('x'), c_char('X')]),
        many1(hex_digit()),
    ).map(|(sign, _, x, integer): (Option<char>, char, char, String)| {
        match Token::from_parts(sign, Some(integer.as_str()), None, Some(x), None, NumericKind::Hex) {
            Ok(t) => t,
            Err(e) => panic!("Error parsing hex literal {}", e)
        }
    })
}

fn bin_literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        optional(choice([c_char('-'), c_char('+')])),
        c_char('0'),
        choice([c_char('b'), c_char('B')]),
        many1(choice([c_char('1'), c_char('0')])),
    ).map(|(sign, _, b, integer): (Option<char>, char, char, String)| {
        match Token::from_parts(sign, Some(integer.as_str()), None, Some(b), None, NumericKind::Bin) {
            Ok(t) => t,
            Err(e) => panic!("Error parsing binary literal {}", e)
        }
    })
}

fn octal_literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        optional(choice([c_char('-'), c_char('+')])),
        c_char('0'),
        choice([c_char('o'), c_char('O')]),
        many1(oct_digit()),
    ).map(|(sign, _, o, integer): (Option<char>, char, char, String)| {
        match Token::from_parts(sign, Some(integer.as_str()), None, Some(o), None, NumericKind::Octal) {
            Ok(t) => t,
            Err(e) => panic!("Error parsing octal literal {}", e),
        }
    })
}