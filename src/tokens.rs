use std::ops::Deref;

use combine::{
    choice, eof,
    error::ParseError,
    many, not_followed_by,
    parser::char::{char as c_char, string},
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
/// A wrapper around a token that will include
/// the byte span of the text that it was found
/// at
pub struct Item {
    pub token: Token,
    pub span: Span,
}

impl Item {
    /// Create a new Item from its parts
    pub fn new(token: Token, span: Span) -> Item {
        Item { token, span }
    }
}

impl Deref for Item {
    type Target = Token;
    fn deref<'a>(&'a self) -> &'a Self::Target {
        &self.token
    }
}

#[derive(Debug, PartialEq, Clone)]
/// A location in the original source text
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    /// Create a new Span from its parts
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
}

#[derive(Debug, PartialEq, Clone)]
/// The representation of a single JS token
pub enum Token {
    /// `true` of `false`
    Boolean(BooleanLiteral),
    /// The end of the file
    EoF,
    /// An identifier this will be either a variable name
    /// or a function/method name
    Ident(Ident),
    /// A word that has been reserved to not be used as an identifier
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
    /// A regular expression literal.
    /// ```js
    /// let regex = /[a-zA-Z]+/g;
    /// ```
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
/// The tokenized representation of `true` or `false`
pub enum BooleanLiteral {
    True,
    False,
}
impl BooleanLiteral {
    /// Test if this instance represents `true`
    pub fn is_true(&self) -> bool {
        match self {
            BooleanLiteral::True => true,
            _ => false,
        }
    }
}

impl<'a> From<&'a str> for BooleanLiteral {
    /// Create a BooleanLiteral from raw text
    ///
    /// panics if argument is not `true` or `false`
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
    /// Create a BooleanLiteral from raw text
    ///
    /// panics if argument is not `true` or `false`
    fn from(s: String) -> Self {
        BooleanLiteral::from(s.as_str())
    }
}

impl From<bool> for BooleanLiteral {
    /// Creates a JS Bool for a rust bool
    fn from(b: bool) -> Self {
        if b {
            BooleanLiteral::True
        } else {
            BooleanLiteral::False
        }
    }
}

impl Into<String> for BooleanLiteral {
    /// Return this BooleanLiteral to the text
    /// that was parsed to create it
    fn into(self) -> String {
        match self {
            BooleanLiteral::True => "true".into(),
            BooleanLiteral::False => "false".into(),
        }
    }
}

impl ToString for BooleanLiteral {
    /// Return this BooleanLiteral to the text
    /// that was parsed to create it
    fn to_string(&self) -> String {
        match self {
            BooleanLiteral::True => "true".into(),
            BooleanLiteral::False => "false".into(),
        }
    }
}

impl Into<bool> for BooleanLiteral {
    /// Creates a Rust bool for a js bool
    fn into(self) -> bool {
        match self {
            BooleanLiteral::True => true,
            BooleanLiteral::False => false,
        }
    }
}

impl<'a> Into<bool> for &'a BooleanLiteral {
    /// Creates a js bool for a rust bool
    fn into(self) -> bool {
        match self {
            BooleanLiteral::True => true,
            BooleanLiteral::False => false,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
/// An identifier
/// ```
/// # extern crate ress;
/// # use ress::{Scanner, Item, Token, Ident};
/// # fn main() {
/// let js = "var x = 1;";
/// let mut s = Scanner::new(js);
/// let _var = s.next().unwrap();
/// assert_eq!(s.next().unwrap().token,
///             Token::Ident(Ident::from("x")));
/// let _assign = s.next().unwrap();
/// let _one = s.next().unwrap();
/// # }
/// ```
pub struct Ident(String);

impl<'a> PartialEq<&'a str> for Ident {
    fn eq(&self, other: &&'a str) -> bool {
        &self.0 == other
    }
}

impl PartialEq<str> for Ident {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

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
    ///Create and instance of Token::Ident from a &str
    pub fn ident(name: &str) -> Token {
        Token::Ident(name.into())
    }
    ///Create and instance of Token::Keyword from a &str
    ///
    /// panics if the argument isn't a valid js keyword
    pub fn keyword(name: &str) -> Token {
        Token::Keyword(keywords::Keyword::from(name))
    }
    ///Create and instance of Token::Numeric from a &str
    pub fn numeric(number: &str) -> Token {
        Token::Numeric(numeric::Number::from(number))
    }
    ///Create and instance of Token::Punct from a &str
    ///
    /// panics if the augment isn't valid js punctuation
    pub fn punct(s: &str) -> Token {
        Token::Punct(s.into())
    }
    ///Create and instance of Token::String from a &str wrapped in double quotes
    pub fn double_quoted_string(s: &str) -> Token {
        Token::String(strings::StringLit::Double(s.into()))
    }
    ///Create an instance of Token::String from a &str wrapped in single quotes
    pub fn single_quoted_string(s: &str) -> Token {
        Token::String(strings::StringLit::Single(s.into()))
    }
    /// Create an instance of Token::RegEx from a &str and an Option<String>
    pub fn regex(body: &str, flags: Option<String>) -> Token {
        Token::RegEx(regex::RegEx::from_parts(body, flags))
    }
    /// Creates an instance of Token::Template with a template string that has
    /// no substitutions
    ///
    /// ```js
    /// var noSub = `template string with no subs`;
    /// ```
    pub fn no_sub_template(s: &str) -> Token {
        Token::Template(strings::Template::NoSub(s.into()))
    }
    /// Creates an instance of Token::Template for a template head
    /// ```js
    /// let t = `head ${true} middle ${false} tail ${false}`;
    /// ```
    pub fn template_head(s: &str) -> Token {
        Token::Template(strings::Template::Head(s.into()))
    }
    /// Creates an instance of a Token::Template for a template middle
    /// ```js
    /// let t = `head ${false} middle ${true} tail ${false}`;
    /// ```
    pub fn template_middle(s: &str) -> Token {
        Token::Template(strings::Template::Middle(s.into()))
    }
    /// Creates an instance of a Token::Template for a template tail
    /// ```js
    /// let t = `head ${false} middle ${false} tail ${true}`;
    /// ```
    pub fn template_tail(s: &str) -> Token {
        Token::Template(strings::Template::Tail(s.into()))
    }
    /// Creates an instance of a Token::Comment for a comment string and a flag
    /// if this comment should be treated as a multi line comment
    /// note, this will not generate HTML-style comments
    /// ```
    /// # extern crate ress;
    /// # use ress::{Scanner, Item, Token, Comment};
    /// # fn main() {
    /// let single_js = "//I am a comment";
    /// let multi_js = "/*I am a multi-line comment*/";
    /// let mut s = Scanner::new(single_js);
    /// let single_scanner = s.next().expect("unable to parse single line comment");
    /// let single = Token::comment("I am a comment", false);
    /// assert_eq!(single, single_scanner.token);
    /// s = Scanner::new(multi_js);
    /// let multi_scanner = s.next().expect("Unable to parse multi-line comment");
    /// let multi = Token::comment("I am a multi-line comment", true);
    /// assert_eq!(multi, multi_scanner.token);
    /// # }
    /// ```
    pub fn comment(comment: &str, multi: bool) -> Token {
        Token::Comment(comments::Comment::from_parts(
            comment.into(),
            if multi {
                comments::Kind::Multi
            } else {
                comments::Kind::Single
            },
            None
        ))
    }
}
//Is tests
impl Token {
    pub fn is_boolean(&self) -> bool {
        match self {
            Token::Boolean(_) => true,
            _ => false,
        }
    }
    pub fn is_boolean_true(&self) -> bool {
        match self {
            Token::Boolean(ref b) => b.into(),
            _ => false,
        }
    }
    pub fn is_boolean_false(&self) -> bool {
        match self {
            Token::Boolean(ref b) => {
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
            Token::Ident(_) => true,
            _ => false,
        }
    }
    pub fn is_keyword(&self) -> bool {
        match self {
            Token::Keyword(_) => true,
            _ => false,
        }
    }
    pub fn is_strict_reserved(&self) -> bool {
        match self {
            Token::Keyword(ref k) => k.is_strict_reserved(),
            _ => false,
        }
    }
    pub fn is_restricted(&self) -> bool {
        match self {
            Token::Ident(ref i) => i == "arguments" || i == "eval",
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
            Token::Numeric(ref n) => n.is_hex(),
            _ => false,
        }
    }
    pub fn is_bin_literal(&self) -> bool {
        match self {
            Token::Numeric(ref n) => n.is_bin(),
            _ => false,
        }
    }
    pub fn is_oct_literal(&self) -> bool {
        match self {
            Token::Numeric(ref n) => n.is_oct(),
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
            Token::RegEx(_) => true,
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
            Token::Boolean(_) => true,
            Token::String(_) => true,
            Token::Null => true,
            Token::Numeric(_) => true,
            Token::RegEx(_) => true,
            Token::Template(_) => true,
            _ => false,
        }
    }
    pub fn is_comment(&self) -> bool {
        match self {
            Token::Comment(_) => true,
            _ => false,
        }
    }
    pub fn is_multi_line_comment(&self) -> bool {
        match self {
            Token::Comment(ref t) => t.kind == comments::Kind::Multi,
            _ => false,
        }
    }

    pub fn is_single_line_comment(&self) -> bool {
        match self {
            Token::Comment(ref t) => t.kind == comments::Kind::Single,
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
                (&BooleanLiteral::True, "true") | (&BooleanLiteral::False, "false") => true,
                _ => false,
            },
            _ => false,
        }
    }
    pub fn matches_ident_str(&self, name: &str) -> bool {
        match self {
            Token::Ident(ref i) => name == i.0,
            _ => false,
        }
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
            let flags = if parts.1.is_empty() {
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
            Token::Comment(ref t) => t.content == comment,
            _ => false,
        }
    }

    pub fn matches_string_content(&self, content: &str) -> bool {
        match self {
            Token::String(ref lit) => match lit {
                strings::StringLit::Single(ref s) => content == s,
                strings::StringLit::Double(ref s) => content == s,
            },
            _ => false,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Boolean(ref b) => b.to_string(),
            Token::EoF => String::new(),
            Token::Ident(ref i) => i.to_string(),
            Token::Keyword(ref k) => k.to_string(),
            Token::Null => String::from("null"),
            Token::Numeric(ref n) => n.to_string(),
            Token::Punct(ref p) => p.to_string(),
            Token::RegEx(ref r) => r.to_string(),
            Token::String(ref s) => s.to_string(),
            Token::Template(ref t) => t.to_string(),
            Token::Comment(ref c) => c.to_string(),
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
        .skip(not_followed_by(raw_ident_part()))
        .map(|s: &str| s.to_string())
}

pub(crate) fn raw_ident_part<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        unicode::id_continue(),
        c_char('$'),
        c_char('\\').skip(c_char('u')),
        c_char('\u{200C}'),
        c_char('\u{200D}')
    ))
}

fn false_literal<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("false")
        .skip(not_followed_by(raw_ident_part()))
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
    (ident_start(), many(ident_part())).map(|(start, body): (String, String)| {
        Token::Ident(Ident(start + &body))
    })
}

pub(crate) fn null_literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("null")
        .skip(not_followed_by(raw_ident_part()))
        .map(|_| Token::Null)
}

fn ident_start<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(unicode::id_start().map(|c: char| c.to_string())),
        try(c_char('$').map(|c: char| c.to_string())),
        try(c_char('_').map(|c: char| c.to_string())),
        try(unicode::char_literal()),
    )).map(|s: String| s)
}

pub(crate) fn ident_part<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(ident_start()),
        try(raw_ident_part().map(|c: char| c.to_string())),
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
            assert_eq!(t, (Token::Ident(Ident(i.to_string())), ""))
        }
    }

}
