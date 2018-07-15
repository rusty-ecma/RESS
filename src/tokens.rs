use combine::{
    choice, eof, error::ParseError, many,
    parser::{
        char::{char as c_char, string},
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
pub struct Item {
    pub token: Token,
    pub span: Span,
}

impl Item {
    pub fn new(token: Token, span: Span) -> Item {
        Item {
            token,
            span,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span {
            start,
            end,
        }
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
    /// A regex literal (`/[a-fA-F0-9]+/g`) the first associated value
    /// will be the pattern, the second will be the optional flags
    RegEx(regex::RegEx),
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
            &BooleanLiteral::False => false
        }
    }
}

impl Token {
    pub fn is_punct(&self) -> bool {
        if let Token::Punct(ref _p) = self {
            true
        } else {
            false
        }
    }

    pub fn matches_punct(&self, p: punct::Punct) -> bool {
        self == &Token::Punct(p)
    }

    pub fn matches_punct_str(&self, s: &str) -> bool {
        self == &Token::punct(s)
    }

    pub fn punct(s: &str) -> Token {
        Token::Punct(s.into())
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            &Token::Boolean(_) => true,
            _ => false
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
            },
            _ => false,
        }
    }
    pub fn matches_boolean(&self, b: BooleanLiteral) -> bool {
        self == &Token::Boolean(b)
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
    pub fn matches_ident_str(&self, name: &str) -> bool {
        self == &Token::ident(name)
    }

    pub fn ident(name: &str) -> Token {
        Token::Ident(name.into())
    }

    pub fn is_keyword(&self) -> bool {
        match self {
            &Token::Keyword(_) => true,
            _ => false,
        }
    }

    pub fn matches_keyword(&self, keyword: keywords::Keyword) -> bool {
        self == &Token::Keyword(keyword)
    }

    pub fn matches_keyword_str(&self, name: &str) -> bool {
        self == &Token::keyword(name)
    }

    pub fn keyword(name: &str) -> Token {
        Token::Keyword(keywords::Keyword::from(name))
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
            &Token::Numeric(ref n) => n.kind == numeric::Kind::Hex,
            _ => false,
        }
    }

    pub fn is_bin_literal(&self) -> bool {
        match self {
            &Token::Numeric(ref n) => n.kind == numeric::Kind::Bin,
            _ => false,
        }
    }

    pub fn is_oct_literal(&self) -> bool {
        match self {
            &Token::Numeric(ref n) => n.kind == numeric::Kind::Octal,
            _ => false,
        }
    }

    pub fn matches_numeric_str(&self, number: &str) -> bool {
        self == &Token::numeric(number)
    }

    pub fn matches_numeric(&self, number: numeric::Number) -> bool {
        self == &Token::Numeric(number)
    }

    pub fn numeric(number: &str) -> Token {
        Token::Numeric(numeric::Number::from(number))
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
                _ => false
            },
            _ => false,
        }
    }

    pub fn is_single_quoted_string(&self) -> bool {
        match self {
            Token::String(ref s) => match s {
                strings::StringLit::Single(_) => true,
                _ => false
            },
            _ => false,
        }
    }

    pub fn double_quoted_string(s: &str) -> Token {
        Token::String(strings::StringLit::Double(s.into()))
    }

    pub fn single_quoted_string(s: &str) -> Token {
        Token::String(strings::StringLit::Single(s.into()))
    }

    pub fn no_sub_template(s: &str) -> Token {
        Token::String(strings::StringLit::NoSubTemplate(s.into()))
    }

    pub fn template_head(s: &str) -> Token {
        Token::String(strings::StringLit::TemplateHead(s.into()))
    }

    pub fn template_middle(s: &str) -> Token {
        Token::String(strings::StringLit::TemplateMiddle(s.into()))
    }

    pub fn template_tail(s: &str) -> Token {
        Token::String(strings::StringLit::TemplateTail(s.into()))
    }

    pub fn is_regex(&self) -> bool {
        match self {
            &Token::RegEx(_) => true,
            _ => false,
        }
    }

    pub fn matches_regex(&self, regex: regex::RegEx) -> bool {
        self == &Token::RegEx(regex)
    }

    pub fn matches_regex_str(&self, body: &str, flags: Option<&str>) -> bool {
        self == &Token::regex(body, flags)
    }

    pub fn regex(body: &str, flags: Option<impl Into<String>>) -> Token {
        Token::RegEx(regex::RegEx::from_parts(body, flags.map(|s| s.into())))
    }

    pub fn is_template(&self) -> bool {
        self.is_template_head() ||
        self.is_template_middle() ||
        self.is_template_tail()
    }

    pub fn is_template_head(&self) -> bool {
        match self {
            &Token::String(ref s) => match s {
                &strings::StringLit::TemplateHead(_) => true,
                _ => false,
            },
            _ => false
        }
    }
    pub fn is_template_middle(&self) -> bool {
        match self {
            &Token::String(ref s) => match s {
                &strings::StringLit::TemplateMiddle(_) => true,
                _ => false,
            },
            _ => false
        }
    }
    pub fn is_template_tail(&self) -> bool {
        match self {
            &Token::String(ref s) => match s {
                &strings::StringLit::TemplateTail(_) => true,
                _ => false,
            },
            _ => false
        }
    }

    pub fn is_comment(&self) -> bool {
        match self {
            &Token::Comment(_) => true,
            _ => false,
        }
    }

    pub fn matches_comment(&self, comment: comments::Comment) -> bool {
        self == &Token::Comment(comment)
    }

    pub fn matches_comment_str(&self, comment: &str) -> bool {
        match self {
            &Token::Comment(ref t) => t.content == comment,
            _ => false
        }
    }

    pub fn is_multi_line_comment(&self) -> bool {
        match self {
            &Token::Comment(ref t) => t.kind == comments::Kind::Multi,
            _ => false
        }
    }

    pub fn is_single_line_comment(&self) -> bool {
        match self {
            &Token::Comment(ref t) => t.kind == comments::Kind::Single,
            _ => false,
        }
    }

    pub fn comment(comment: &str, multi: bool) -> Token {
        Token::Comment(comments::Comment::from_parts(comment.into(), if multi {
            comments::Kind::Multi
        } else {
            comments::Kind::Single
        }))
    }
}

pub fn token<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (choice((
        try(token_not_eof()),
        try(end_of_input()),
    ))).map(|t| t)
}

pub(crate) fn token_not_eof<I>() -> impl Parser<Input = I, Output = Token>
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
        try(strings::literal()),
        try(punct::punctuation()),
        try(strings::template()),
    ))).map(|t| t)
}

pub(crate) fn boolean_literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((string("true"), string("false"))).map(|t: &str| Token::Boolean(BooleanLiteral::from(t)))
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
    string("null").map(|_| Token::Null)
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
