use combine::{
    choice, eof, error::ParseError, many,
    parser::{
        char::{char as c_char, string, spaces},
    },
    try, Parser, Stream,
};

use regex;
use strings;
use unicode;
use numeric;
use punct;
use keywords;
use comments;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub data: TokenData,
    pub span: Span,
}

impl Token {
    pub fn new(data: TokenData, start: usize, end: usize) -> Token {
        Token {
            data,
            span: Span {
                start,
                end,
            }
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Clone)]
/// The representation of a single JS token
pub enum TokenData {
    /// True of false, will contain the value
    Boolean(BooleanLiteral),
    /// The end of the file
    EoF,
    /// An identifier this will be either a variable name
    /// or a function/method name
    Ident(String),
    /// A keyword, currently this is all EcmaScript Keywords
    Keyword(keywords::Token),
    /// A `null` literal value
    Null,
    /// A number, this includes integers (`1`), decimals (`0.1`),
    /// hex (`0x8f`), binary (`0b010011010`), and octal (`0o273`)
    Numeric(numeric::Token),
    /// A punctuation mark, this includes all mathematical operators
    /// logical operators and general syntax punctuation
    Punct(punct::Token),
    /// A string literal, either double or single quoted, the associated
    /// value will be the unquoted string
    String(strings::Token),
    /// A regex literal (`/[a-fA-F0-9]+/g`) the first associated value
    /// will be the pattern, the second will be the optional flags
    RegEx(regex::Token),
    /// A template string literal
    /// note: This is not yet implemented
    Template(Vec<TokenData>),
    /// A comment, the associated value will contain the raw comment
    /// This will capture both inline comments `// I am an inline comment`
    /// and multi-line comments
    /// ```js
    /// /*multi lines
    /// * comments
    /// */
    /// ```
    Comment(comments::Token),
}
#[derive(Debug, PartialEq, Clone)]
pub enum BooleanLiteral {
    True,
    False,
}

impl<'a> From<&'a str> for BooleanLiteral {
    fn from(s: &'a str) -> Self {
        if s == "true" {
            BooleanLiteral::True
        } else if s == "false" {
            BooleanLiteral::False
        } else {
            panic!(r#"BooleanLiteral can only be created for "true" or "false"."#)
        }
    }
}

impl From<bool> for BooleanLiteral {
    fn from(b: bool) -> Self {
        if b {
            BooleanLiteral::True
        } else {
            BooleanLiteral::False
        }
    }
}

impl Into<String> for BooleanLiteral {
    fn into(self) -> String {
        match self {
            BooleanLiteral::True => "true".into(),
            BooleanLiteral::False => "false".into(),
        }
    }
}

impl TokenData {
    pub fn is_punct(&self) -> bool {
        if let TokenData::Punct(ref _p) = self {
            true
        } else {
            false
        }
    }

    pub fn is_punct_with(&self, p: &str) -> bool {
        self == &TokenData::punct(p)
    }

    pub fn punct(s: impl Into<String>) -> TokenData {
        TokenData::Punct(punct::Token::from(s.into()))
    }

    pub fn is_boolean(&self) -> bool {
        if let TokenData::Boolean(ref _b) = self {
            true
        } else {
            false
        }
    }

    pub fn is_boolean_with(&self, b: bool) -> bool {
        self == &TokenData::Boolean(BooleanLiteral::from(b))
    }

    pub fn is_eof(&self) -> bool {
        self == &TokenData::EoF
    }

    pub fn is_ident(&self) -> bool {
        if let TokenData::Ident(ref _i) = self {
            true
        } else {
            false
        }
    }

    pub fn is_ident_with(&self, name: &str) -> bool {
        self == &TokenData::ident(name)
    }

    pub fn ident(name: impl Into<String>) -> TokenData {
        TokenData::Ident(name.into())
    }

    pub fn is_keyword(&self) -> bool {
        if let TokenData::Keyword(ref _k) = self {
            true
        } else {
            false
        }
    }

    pub fn is_keyword_with(&self, name: &str) -> bool {
        self == &TokenData::keyword(name)
    }

    pub fn keyword(name: impl Into<String>) -> TokenData {
        TokenData::Keyword(keywords::Token::from(name.into()))
    }

    pub fn is_null(&self) -> bool {
        self == &TokenData::Null
    }

    pub fn is_numeric(&self) -> bool {
        if let TokenData::Numeric(ref _n) = self {
            true
        } else {
            false
        }
    }

    pub fn is_hex_literal(&self) -> bool {
        match self {
            &TokenData::Numeric(ref n) => n.kind == numeric::Kind::Hex,
            _ => false,
        }
    }

    pub fn is_bin_literal(&self) -> bool {
        match self {
            &TokenData::Numeric(ref n) => n.kind == numeric::Kind::Bin,
            _ => false,
        }
    }

    pub fn is_oct_literal(&self) -> bool {
        match self {
            &TokenData::Numeric(ref n) => n.kind == numeric::Kind::Octal,
            _ => false,
        }
    }

    pub fn is_numeric_with(&self, number: &str) -> bool {
        self == &TokenData::numeric(number)
    }

    pub fn numeric(number: impl Into<String>) -> TokenData {
        TokenData::Numeric(numeric::Token::from(number.into()))
    }

    pub fn is_string(&self) -> bool {
        if let TokenData::String(ref _s) = self {
            true
        } else {
            false
        }
    }

    pub fn is_string_with_content(&self, s: &str) -> bool {
        match self {
            TokenData::String(ref t) => t.content == s,
            _ => false,
        }
    }

    pub fn double_quoted_string(s: &str) -> TokenData {
        TokenData::String(strings::Token::from_parts(strings::Quote::Double, s))
    }

    pub fn single_quoted_string(s: &str) -> TokenData {
        TokenData::String(strings::Token::from_parts(strings::Quote::Single, s))
    }

    pub fn string(s: impl Into<String>, quote: strings::Quote) -> TokenData {
        TokenData::String(strings::Token::from_parts(quote, s.into().as_str()))
    }

    pub fn is_regex(&self) -> bool {
        if let TokenData::RegEx(ref _r) = self {
            true
        } else {
            false
        }
    }

    pub fn is_regex_with(&self, body: &str, flags: Option<&str>) -> bool {
        self == &TokenData::regex(body, flags)
    }

    pub fn regex(body: &str, flags: Option<impl Into<String>>) -> TokenData {
        TokenData::RegEx(regex::Token::from_parts(body, flags.map(|s| s.into())))
    }

    pub fn is_template(&self) -> bool {
        if let TokenData::Template(ref _t) = self {
            true
        } else {
            false
        }
    }

    pub fn template(value: impl Iterator<Item = TokenData>) -> TokenData {
        TokenData::Template(value.collect())
    }

    pub fn is_comment(&self) -> bool {
        if let TokenData::Comment(ref _c) = self {
            true
        } else {
            false
        }
    }

    pub fn is_comment_with(&self, comment: &str) -> bool {
        match self {
            &TokenData::Comment(ref t) => t.content == comment,
            _ => false
        }
    }

    pub fn is_multi_line_comment(&self) -> bool {
        match self {
            &TokenData::Comment(ref t) => t.kind == comments::Kind::Multi,
            _ => false
        }
    }

    pub fn is_single_line_comment(&self) -> bool {
        match self {
            &TokenData::Comment(ref t) => t.kind == comments::Kind::Single,
            _ => false,
        }
    }

    pub fn comment(comment: impl Into<String>, multi: bool) -> TokenData {
        TokenData::Comment(comments::Token::from_parts(comment.into(), if multi {
            comments::Kind::Multi
        } else {
            comments::Kind::Single
        }))
    }
}

pub fn tokens<I>() -> impl Parser<Input = I, Output = Vec<TokenData>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    many(token_not_eof().skip(spaces()))
}

pub fn token<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (choice((
        try(token_not_eof()),
        try(end_of_input()),
    ))).map(|t| t)
}

pub(crate) fn token_not_eof<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (choice((
        try(comments::comment()),
        try(boolean_literal()),
        try(keywords::literal()),
        try(ident()),
        try(null_literal()),
        try(numeric::literal()),
        try(regex::literal()),
        try(strings::literal()),
        try(punct::punctuation()),
        try(strings::template()),
    ))).map(|t| t)
}

pub(crate) fn boolean_literal<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((string("true"), string("false"))).map(|t: &str| TokenData::Boolean(BooleanLiteral::from(t)))
}

pub(crate) fn end_of_input<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    eof().map(|_| TokenData::EoF)
}

pub(crate) fn ident<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (ident_start(), many(ident_part())).map(|(start, body): (char, String)| {
        let mut ret = String::new();
        ret.push(start);
        ret.push_str(&body);
        TokenData::Ident(ret)
    })
}

pub(crate) fn null_literal<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("null").map(|_| TokenData::Null)
}

fn unicode_char<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(unicode::lu()),
        try(unicode::ll()),
        try(unicode::lt()),
        try(unicode::lm()),
        try(unicode::lo()),
        try(unicode::nl()),
    )).map(|c: char| c)
}

fn ident_start<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(unicode_char()),
        try(c_char('$')),
        try(c_char('_')),
        try(unicode::char_literal()),
    )).map(|c: char| c)
}

pub(crate) fn ident_part<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(ident_start()),
        try(unicode::mn()),
        try(unicode::mc()),
        try(unicode::nd()),
        try(unicode::pc()),
    ))
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn bool() {
        let t = super::boolean_literal().parse("true").unwrap();
        let f = super::boolean_literal().parse("false").unwrap();
        assert_eq!(t, (TokenData::Boolean(BooleanLiteral::True), ""));
        assert_eq!(f, (TokenData::Boolean(BooleanLiteral::False), ""));
    }

    #[test]
    fn eof() {
        let e = super::end_of_input().parse("").unwrap();
        assert_eq!(e, (TokenData::EoF, ""));
    }

    #[test]
    fn ident_tests() {
        let idents = vec![
            "$",
            "x",
            "thing",
            "num",
            "stuff",
            "anotherThing",
            "snake_thing",
            "junk",
            "_",
            "_private",
        ];
        for i in idents {
            let t = token().parse(i.clone()).unwrap();
            assert_eq!(t, (TokenData::Ident(i.to_owned()), ""))
        }
    }
    
}
