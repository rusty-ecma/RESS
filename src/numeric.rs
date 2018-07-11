use std::str::FromStr;
use combine::{
    choice, error::ParseError, many, many1, optional,
    parser::{
        char::{char as c_char, digit, hex_digit, oct_digit},
    },
    try, Parser, Stream,
};
use tokens::{Token};
#[derive(Debug, PartialEq, Clone)]
pub struct Number {
    pub sign: Option<Sign>,
    pub integer: Option<usize>,
    pub remainder: Option<usize>,
    pub char_case: Option<CharCase>,
    pub exponent: Option<usize>,
    pub kind: Kind
}

impl Number {
    pub fn from_parts(sign: Option<char>, integer: Option<&str>, remainder: Option<&str>, 
                        char_case: Option<char>, exponent: Option<&str>, kind: Kind) -> Result<Self, super::error::Error> {
        let sign = if let Some(s) = sign {
            Some(Sign::from(s))
        } else {
            None
        };
        let integer = if let Some(i) = integer {
            let n: usize = Self::parse_integer(i, &kind)?;
            Some(n)
        } else {
            None
        };
        let remainder = if let Some(r) = remainder {
            let n: usize = Self::parse_integer(r, &kind)?;
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
            let n: usize = Self::parse_integer(e, &kind)?;
            Some(n)
        } else {
            None
        };
        Ok(Number {
            sign,
            integer,
            remainder,
            char_case,
            exponent,
            kind,
        })
    }

    fn parse_integer(s: &str, kind: &Kind) -> Result<usize, super::error::Error> {
        let radix = match kind {
            &Kind::Decimal => 10,
            &Kind::Bin => 2,
            &Kind::Hex => 16,
            &Kind::Octal => 8,
        };
        Ok(usize::from_str_radix(s, radix)?)
    }

    fn parse_char_case(c: char) -> Result<CharCase, super::error::Error> {
        let small = c.to_lowercase().next().ok_or(super::error::Error::DataMismatch("numeric char to_lowercase failed".into()))?;
        if small != 'e' && small != 'b' && small != 'x' && small != 'o' {
            return Err(super::error::Error::DataMismatch(format!("Expected e b x or o found {}", c)))
        }
        Ok(CharCase::from(c))
    }
}

impl From<String> for Number {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl<'a> From<&'a str> for Number {
    fn from(s: &'a str) -> Self {
        let (sign, s) = if s.starts_with('-') ||  s.starts_with('+') {
            (Some(
                char::from_str(&s[0..1]).unwrap()
            ), &s[1..])
        } else {
            (None, s)
        };
        let res = if s.starts_with("0x") {
            Self::from_parts(sign, Some(&s[2..]), None, Some('x'), None, Kind::Hex)
        } else if s.starts_with("0X") {
            Self::from_parts(sign, Some(&s[2..]), None, Some('X'), None, Kind::Hex)
        } else if s.starts_with("0b") {
            Self::from_parts(sign, Some(&s[2..]), None, Some('b'), None, Kind::Bin)
        } else if s.starts_with("0B") {
            Self::from_parts(sign, Some(&s[2..]), None, Some('B'), None, Kind::Bin)
        } else if s.starts_with("0o") {
            Self::from_parts(sign, Some(&s[2..]), None, Some('o'), None, Kind::Octal)
        } else if s.starts_with("0O") {
            Self::from_parts(sign, Some(&s[2..]), None, Some('O'), None, Kind::Octal)
        } else {
            let (c, mut parts) = if s.contains('e') {
                (Some('e'), s.split('e'))
            } else if s.contains('E') {
                (Some('E'), s.split('E'))
            } else {
                (None, s.split('e'))
            };
            if let Some(front) = parts.next() {
                if front.starts_with('.') {
                    Self::from_parts(sign, None, Some(&front[1..]), c, parts.next(), Kind::Decimal)
                } else {
                    let mut front_parts = front.split('.');
                    let int = front_parts.next();
                    let rem = front_parts.next();
                    let exp = parts.next();
                    Self::from_parts(sign, int, rem, c, exp, Kind::Decimal)
                }
            } else {
                Self::from_parts(sign, None, None, None, None, Kind::Decimal)
            }
        };
        match res {
            Ok(t) => t,
            Err(e) => panic!("{}", e),
        }
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        match self.kind {
            Kind::Hex => self.as_hex_string(),
            Kind::Decimal => self.as_decimal_string(),
            Kind::Octal => self.as_oct_string(),
            Kind::Bin => self.as_bin_string(),
        }
    }
}

//to string implementations
impl Number {
    fn string_prefix(&self) -> String {
        let mut ret = String::new();
        if let Some(ref sign) = self.sign {
            ret.push(sign.into());
        }
        if self.kind == Kind::Decimal {
            return ret;
        }
        if let Some(ref c) = self.char_case {
            ret.push_str(&format!("0{}", self.kind.flag(c)));
        } else {
            ret.push_str(&format!("0{}", self.kind.flag(&CharCase::Lower)));
        }
        ret
    }
    fn as_hex_string(&self) -> String {
        let mut ret = self.string_prefix();
        if let Some(ref i) = self.integer {
            ret.push_str(&format!("{:x}", i));
        }
        ret
    }
    fn as_oct_string(&self) -> String {
        let mut ret = self.string_prefix();
        if let Some(ref i) = self.integer {
            ret.push_str(&format!("{:o}", i))
        }
        ret
    }
    fn as_bin_string(&self) -> String {
        let mut ret = self.string_prefix();
        if let Some(ref i) = self.integer {
            ret.push_str(&format!("{:b}", i))
        }
        ret
    }
    fn as_decimal_string(&self) -> String {
        let mut ret = self.string_prefix();
        if let Some(i) = self.integer {
            ret.push_str(&format!("{}", i));
        }
        if let Some(r) = self.remainder {
            ret.push_str(&format!(".{}", r));
        }
        if let Some(ref c) = self.char_case {
            ret.push(self.kind.flag(c));
        }
        if let Some(e) = self.exponent {
            ret.push_str(&format!("{}", e));
        }
        ret
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

impl CharCase {
    fn with_char(&self, c: char) -> Option<char> {
        match self {
            CharCase::Upper => c.to_uppercase().next(),
            CharCase::Lower => c.to_lowercase().next(),
        }
    }
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

impl Into<char> for Sign {
    fn into(self) -> char {
        match self {
            Sign::Positive => '+',
            Sign::Negative => '-',
        }
    }
}

impl<'a> Into<char> for &'a Sign {
    fn into(self) -> char {
        match self {
            &Sign::Positive => '+',
            &Sign::Negative => '-',
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
    Decimal,
    Hex,
    Bin,
    Octal,
}

impl Kind {
    fn flag(&self, case: &CharCase) -> char {
        match self {
            Kind::Decimal => case.with_char('e').unwrap(),
            Kind::Hex => case.with_char('x').unwrap(),
            Kind::Octal => case.with_char('o').unwrap(),
            Kind::Bin => case.with_char('b').unwrap(),
        }
    }
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
    choice(
        (try(full_decimal_literal()),
        try(no_leading_decimal())
    )).map(|t| t)
}

fn full_decimal_literal<I>() -> impl Parser<Input = I, Output = Number>
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
                (Some(r), Some(e)) => Number::from_parts(sign, Some(&integer), Some(r.1.as_str()), Some(e.0), Some(e.1.as_str()), Kind::Decimal),
                (None, Some(e)) => Number::from_parts(sign, Some(&integer), None, Some(e.0), Some(e.1.as_str()), Kind::Decimal),
                (Some(r), None) => Number::from_parts(sign, Some(&integer), Some(r.1.as_str()), None, None, Kind::Decimal),
                (None, None) => Number::from_parts(sign, Some(&integer), None, None, None, Kind::Decimal),
            };

            match res {
                Ok(t) => t,
                Err(e) => panic!("error parsing decimal literal {}", e)
            }
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
    ).map(|(sign, _dot, remainder, exponent): (Option<char>, char, String, Option<(char, String)>)| {
        let res = if let Some(e) = exponent {
            Number::from_parts(sign, None, Some(remainder.as_str()), Some(e.0), Some(e.1.as_str()), Kind::Decimal)
        } else {
            Number::from_parts(sign, None, Some(remainder.as_str()), None, None, Kind::Decimal)
        };
        match  res {
            Ok(t) => t,
            Err(e) => panic!("Error parsing decimal literal {}", e)
        }

    })
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
    ).map(|(sign, _, x, integer): (Option<char>, char, char, String)| {
        match Number::from_parts(sign, Some(integer.as_str()), None, Some(x), None, Kind::Hex) {
            Ok(t) => t,
            Err(e) => panic!("Error parsing hex literal {}", e)
        }
    })
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
    ).map(|(sign, _, b, integer): (Option<char>, char, char, String)| {
        match Number::from_parts(sign, Some(integer.as_str()), None, Some(b), None, Kind::Bin) {
            Ok(t) => t,
            Err(e) => panic!("Error parsing binary literal {}", e)
        }
    })
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
    ).map(|(sign, _, o, integer): (Option<char>, char, char, String)| {
        match Number::from_parts(sign, Some(integer.as_str()), None, Some(o), None, Kind::Octal) {
            Ok(t) => t,
            Err(e) => panic!("Error parsing octal literal {}", e),
        }
    })
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
            "+8",
            "-6",
            "+1E5",
            "-1E2",
            "1.8876e2",
            "-1.009987e87",
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
            "0x123", "0X456", "-0x789", "+0X0abc", "0xdef", "0xABC", "0xDEF",
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
        let vals = vec!["0b000", "0B111", "-0B0101", "+0b1010"];
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
        let vals = vec!["0o7", "0O554", "-0o12345670", "+0O12345670"];
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
            "+8",
            "-6",
            "+1E5",
            "-1E2",
            "1.8876e2",
            "-1.009987e87",
            ".2",
            "-.2",
            ".2E1",
            "+.8",
            "+.2E4",
            ".7e34",
            "-.7e2",
            "+.4e5",
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
            "+8",
            "-6",
            "+1E5",
            "-1E2",
            "1.8876e2",
            "-1.009987e87",
            ".2",
            "-.2",
            ".2E1",
            "+.8",
            "+.2E4",
            ".7e34",
            "-.7e2",
            "+.4e5",
            "0x123",
            "0X456",
            "-0x789",
            "+0X0abc",
            "0xdef",
            "0xABC",
            "0xDEF",
            "0o7",
            "0O554",
            "-0o12345670",
            "+0O12345670",
            "0b000",
            "0B111",
            "-0B0101",
            "+0b1010",
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
}