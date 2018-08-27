use combine::{
    choice, eof, error::ParseError, many, not_followed_by, parser::char::{char as c_char, string},
    try, Parser, Stream,
};

use comments;
use keywords;
use numeric;
use punct;
use regex;
use strings;
use unicode;

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub token: Token,
    pub span: Span,
}

impl Item {
    pub fn new(token: Token, span: Span) -> Item {
        Item { token, span }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
}

#[derive(Debug, PartialEq, Clone)]
/// The representation of a single JS token
pub enum Token {
    /// True of false, will contain the value
    Boolean(BooleanLiteral),
    /// The end of the file
    EoF,
    /// An identifier this will be either a variable name
    /// or a function/method name
    Ident(String),
    /// A keyword, currently this is all EcmaScript Keywords
    Keyword(keywords::Keyword),
    /// A `null` literal value
    Null,
    /// A number, this includes integers (`1`), decimals (`0.1`),
    /// hex (`0x8f`), binary (`0b010011010`), and octal (`0o273`)
    Numeric(numeric::Number),
    /// A punctuation mark, this includes all mathematical operators
    /// logical operators and general syntax punctuation
    Punct(punct::Punct),
    /// A string literal, either double or single quoted, the associated
    /// value will be the unquoted string
    String(strings::StringLit),

    RegEx(regex::RegEx),
    /// The string parts of a template string
    /// ```
    /// # extern crate ress;
    /// # use ress::{Scanner, Item, Token, Number, Template};
    /// # fn main() {
    /// let js = "`Things and stuff times ${10} equals ${100000000}... i think`";
    /// let mut s = Scanner::new(js);
    /// assert_eq!(s.next().unwrap().token,
    ///             Token::template_head("Things and stuff times "));
    /// assert_eq!(s.next().unwrap().token,
    ///             Token::numeric("10"));
    /// assert_eq!(s.next().unwrap().token,
    ///             Token::template_middle(" equals "));
    /// assert_eq!(s.next().unwrap().token,
    ///             Token::numeric("100000000"));
    /// assert_eq!(s.next().unwrap().token,
    ///             Token::template_tail("... i think"));
    /// # }
    /// ```
    Template(strings::Template),
    /// A comment, the associated value will contain the raw comment
    /// This will capture both inline comments `// I am an inline comment`
    /// and multi-line comments
    /// ```js
    /// /*multi lines
    /// * comments
    /// */
    /// ```
    Comment(comments::Comment),
}
#[derive(Debug, PartialEq, Clone)]
pub enum BooleanLiteral {
    True,
    False,
}
impl BooleanLiteral {
    pub fn is_true(&self) -> bool {
        match self {
            BooleanLiteral::True => true,
            _ => false,
        }
    }
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

impl From<String> for BooleanLiteral {
    fn from(s: String) -> Self {
        BooleanLiteral::from(s.as_str())
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

impl Into<bool> for BooleanLiteral {
    fn into(self) -> bool {
        match self {
            BooleanLiteral::True => true,
            BooleanLiteral::False => false,
        }
    }
}

impl<'a> Into<bool> for &'a BooleanLiteral {
    fn into(self) -> bool {
        match self {
            &BooleanLiteral::True => true,
            &BooleanLiteral::False => false,
        }
    }
}

pub struct Ident(String);

impl<'a> From<&'a str> for Ident {
    fn from(s: &'a str) -> Self {
        Ident(s.into())
    }
}

impl From<String> for Ident {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl ToString for Ident {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Into<String> for Ident {
    fn into(self) -> String {
        self.0
    }
}
//Constructors
impl Token {
    pub fn ident(name: &str) -> Token {
        Token::Ident(name.into())
    }
    pub fn keyword(name: &str) -> Token {
        Token::Keyword(keywords::Keyword::from(name))
    }
    pub fn numeric(number: &str) -> Token {
        Token::Numeric(numeric::Number::from(number))
    }
    pub fn punct(s: &str) -> Token {
        Token::Punct(s.into())
    }
    pub fn double_quoted_string(s: &str) -> Token {
        Token::String(strings::StringLit::Double(s.into()))
    }
    pub fn single_quoted_string(s: &str) -> Token {
        Token::String(strings::StringLit::Single(s.into()))
    }
    pub fn regex(body: &str, flags: Option<String>) -> Token {
        Token::RegEx(regex::RegEx::from_parts(body, flags.map(|s| s.into())))
    }
    pub fn no_sub_template(s: &str) -> Token {
        Token::Template(strings::Template::NoSub(s.into()))
    }
    pub fn template_head(s: &str) -> Token {
        Token::Template(strings::Template::Head(s.into()))
    }
    pub fn template_middle(s: &str) -> Token {
        Token::Template(strings::Template::Middle(s.into()))
    }
    pub fn template_tail(s: &str) -> Token {
        Token::Template(strings::Template::Tail(s.into()))
    }
    pub fn comment(comment: &str, multi: bool) -> Token {
        Token::Comment(comments::Comment::from_parts(
            comment.into(),
            if multi {
                comments::Kind::Multi
            } else {
                comments::Kind::Single
            },
        ))
    }
}
//Is tests
impl Token {
    pub fn is_boolean(&self) -> bool {
        match self {
            &Token::Boolean(_) => true,
            _ => false,
        }
    }
    pub fn is_boolean_true(&self) -> bool {
        match self {
            &Token::Boolean(ref b) => b.into(),
            _ => false,
        }
    }
    pub fn is_boolean_false(&self) -> bool {
        match self {
            &Token::Boolean(ref b) => {
                let b: bool = b.into();
                !b
            }
            _ => false,
        }
    }
    pub fn is_eof(&self) -> bool {
        self == &Token::EoF
    }
    pub fn is_ident(&self) -> bool {
        match self {
            &Token::Ident(_) => true,
            _ => false,
        }
    }
    pub fn is_keyword(&self) -> bool {
        match self {
            &Token::Keyword(_) => true,
            _ => false,
        }
    }
    pub fn is_strict_reserved(&self) -> bool {
        match self {
            &Token::Keyword(ref k) => k.is_strict_reserved(),
            _ => false
        }
    }
    pub fn is_restricted(&self) -> bool {
        match self {
            &Token::Keyword(ref k) => k.is_restricted(),
            _ => false,
        }
    }
    pub fn is_null(&self) -> bool {
        self == &Token::Null
    }

    pub fn is_numeric(&self) -> bool {
        if let Token::Numeric(ref _n) = self {
            true
        } else {
            false
        }
    }
    pub fn is_hex_literal(&self) -> bool {
        match self {
            &Token::Numeric(ref n) => n.is_hex(),
            _ => false,
        }
    }
    pub fn is_bin_literal(&self) -> bool {
        match self {
            &Token::Numeric(ref n) => n.is_bin(),
            _ => false,
        }
    }
    pub fn is_oct_literal(&self) -> bool {
        match self {
            &Token::Numeric(ref n) => n.is_oct(),
            _ => false,
        }
    }
    pub fn is_punct(&self) -> bool {
        match self {
            Token::Punct(_) => true,
            _ => false,
        }
    }
    pub fn is_string(&self) -> bool {
        if let Token::String(ref _s) = self {
            true
        } else {
            false
        }
    }
    pub fn is_double_quoted_string(&self) -> bool {
        match self {
            Token::String(ref s) => match s {
                strings::StringLit::Double(_) => true,
                _ => false,
            },
            _ => false,
        }
    }
    pub fn is_single_quoted_string(&self) -> bool {
        match self {
            Token::String(ref s) => match s {
                strings::StringLit::Single(_) => true,
                _ => false,
            },
            _ => false,
        }
    }
    pub fn is_regex(&self) -> bool {
        match self {
            &Token::RegEx(_) => true,
            _ => false,
        }
    }
    pub fn is_template(&self) -> bool {
        self.is_template_head() || self.is_template_middle() || self.is_template_tail()
    }
    pub fn is_template_head(&self) -> bool {
        match self {
            Token::Template(ref s) => s.is_head(),
            _ => false,
        }
    }
    pub fn is_template_middle(&self) -> bool {
        match self {
            Token::Template(ref s) => s.is_middle(),
            _ => false,
        }
    }
    pub fn is_template_tail(&self) -> bool {
        match self {
            Token::Template(ref s) => s.is_tail(),
            _ => false,
        }
    }
    pub fn is_literal(&self) -> bool {
        match self {
            &Token::Boolean(_) => true,
            &Token::String(_) => true,
            &Token::Null => true,
            &Token::Numeric(_) => true,
            &Token::RegEx(_) => true,
            &Token::Template(_) => true,
            _ => false,
        }
    }
    pub fn is_comment(&self) -> bool {
        match self {
            &Token::Comment(_) => true,
            _ => false,
        }
    }
    pub fn is_multi_line_comment(&self) -> bool {
        match self {
            &Token::Comment(ref t) => t.kind == comments::Kind::Multi,
            _ => false,
        }
    }

    pub fn is_single_line_comment(&self) -> bool {
        match self {
            &Token::Comment(ref t) => t.kind == comments::Kind::Single,
            _ => false,
        }
    }
}
//matches tests
impl Token {
    pub fn matches_boolean(&self, b: BooleanLiteral) -> bool {
        self == &Token::Boolean(b)
    }
    pub fn matches_boolean_str(&self, b: &str) -> bool {
        match self {
            Token::Boolean(ref lit) => match (lit, b) {
                (&BooleanLiteral::True, "true")
                | (&BooleanLiteral::False, "false") => true,
                _ => false,
            }
            _ => false,
        }
    }
    pub fn matches_ident_str(&self, name: &str) -> bool {
        self == &Token::ident(name)
    }
    pub fn matches_keyword(&self, keyword: keywords::Keyword) -> bool {
        self == &Token::Keyword(keyword)
    }
    pub fn matches_keyword_str(&self, name: &str) -> bool {
        self == &Token::keyword(name)
    }
    pub fn matches_numeric(&self, number: numeric::Number) -> bool {
        self == &Token::Numeric(number)
    }
    pub fn matches_numeric_str(&self, number: &str) -> bool {
        self == &Token::numeric(number)
    }
    pub fn matches_punct(&self, p: punct::Punct) -> bool {
        self == &Token::Punct(p)
    }
    pub fn matches_punct_str(&self, s: &str) -> bool {
        match self {
            Token::Punct(ref p) => p == &s.into(),
            _ => false,
        }
    }
    pub fn matches_regex(&self, regex: regex::RegEx) -> bool {
        self == &Token::RegEx(regex)
    }
    pub fn matches_regex_str(&self, regex: &str) -> bool {
        if let Some(idx) = regex.rfind('/') {
            let parts = regex.split_at(idx);
            let flags = if parts.1.len() == 0 {
                None
            } else {
                Some(parts.1[1..].to_string())
            };
            self == &Token::regex(&parts.0[1..], flags)
        } else {
            false
        }
    }
    pub fn matches_comment(&self, comment: comments::Comment) -> bool {
        self == &Token::Comment(comment)
    }

    pub fn matches_comment_str(&self, comment: &str) -> bool {
        match self {
            &Token::Comment(ref t) => t.content == comment,
            _ => false,
        }
    }
}
parser!{
    pub fn token[I]()(I) -> Token
        where [I: Stream<Item = char>]
    {
        choice((token_not_eof(), end_of_input())).map(|t| t)
    }
}

pub(crate) fn token_not_eof<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        comments::comment(),
        boolean_literal(),
        try(keywords::literal()),
        try(ident()),
        try(null_literal()),
        try(numeric::literal()),
        try(strings::literal()),
        try(punct::punctuation()),
        try(strings::template_start()),
    )).map(|t| t)
}

pub(crate) fn boolean_literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(true_literal()), try(false_literal())))
        .map(|t: String| Token::Boolean(BooleanLiteral::from(t)))
}

fn true_literal<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("true")
        .skip(not_followed_by(ident_part()))
        .map(|s: &str| s.to_string())
}

fn false_literal<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("false")
        .skip(not_followed_by(ident_part()))
        .map(|s: &str| s.to_string())
}

pub(crate) fn end_of_input<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    eof().map(|_| Token::EoF)
}

pub(crate) fn ident<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (ident_start(), many(ident_part())).map(|(start, body): (char, String)| {
        let mut ret = String::new();
        ret.push(start);
        ret.push_str(&body);
        Token::Ident(ret)
    })
}

pub(crate) fn null_literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("null")
        .skip(not_followed_by(ident_part()))
        .map(|_| Token::Null)
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
        assert_eq!(t, (Token::Boolean(BooleanLiteral::True), ""));
        assert_eq!(f, (Token::Boolean(BooleanLiteral::False), ""));
    }

    #[test]
    fn eof() {
        let e = super::end_of_input().parse("").unwrap();
        assert_eq!(e, (Token::EoF, ""));
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
            assert_eq!(t, (Token::Ident(i.to_owned()), ""))
        }
    }

}
