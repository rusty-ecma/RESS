use combine::{
    attempt, choice, eof,
    error::ParseError,
    many, not_followed_by,
    parser::char::{char as c_char, string},
    range::recognize,
    Parser, RangeStream, Stream,
};

use keywords::Keyword;
use punct::Punct;
use refs::{
    comments::comment,
    keywords::literal as keyword,
    numbers::literal as number,
    punct::punctuation,
    strings::{literal as string_lit, template_start},
};
use unicode;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StringLit {
    Double,
    Single,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Comment {
    SingleLine,
    MultiLine,
    Html,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Number {
    Dec,
    Bin,
    Hex,
    Oct,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Template {
    NoSub,
    Head,
    Body,
    Tail,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RefToken {
    /// `true` of `false`
    Boolean(bool),
    /// The end of the file
    EoF,
    /// An identifier this will be either a variable name
    /// or a function/method name
    Ident,
    /// A word that has been reserved to not be used as an identifier
    Keyword(Keyword),
    /// A `null` literal value
    Null,
    /// A number, this includes integers (`1`), decimals (`0.1`),
    /// hex (`0x8f`), binary (`0b010011010`), and octal (`0o273`)
    Numeric(Number),
    /// A punctuation mark, this includes all mathematical operators
    /// logical operators and general syntax punctuation
    Punct(Punct),
    /// A string literal, either double or single quoted, the associated
    /// value will be the unquoted string
    String(StringLit),
    /// A regular expression literal.
    /// ```js
    /// let regex = /[a-zA-Z]+/g;
    /// ```
    RegEx,
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
    Template(Template),
    /// A comment, the associated value will contain the raw comment
    /// This will capture both inline comments `// I am an inline comment`
    /// and multi-line comments
    /// ```js
    /// /*multi lines
    /// * comments
    /// */
    /// ```
    Comment(Comment),
}

impl RefToken {
    pub fn is_comment(&self) -> bool {
        match self {
            RefToken::Comment(_) => true,
            _ => false,
        }
    }

    pub fn is_punct(&self) -> bool {
        match self {
            RefToken::Punct(_) => true,
            _ => false,
        }
    }

    pub fn matches_punct(&self, p: &Punct) -> bool {
        match self {
            RefToken::Punct(o) => p == o,
            _ => false,
        }
    }

    pub fn is_keyword(&self) -> bool {
        match self {
            RefToken::Keyword(_) => true,
            _ => false,
        }
    }

    pub fn is_ident(&self) -> bool {
        match self {
            RefToken::Ident => true,
            _ => false,
        }
    }

    pub fn matches_keyword(&self, k: &Keyword) -> bool {
        match self {
            RefToken::Keyword(l) => k == l,
            _ => false,
        }
    }

    pub fn is_template_tail(&self) -> bool {
        match self {
            RefToken::Template(t) => t == &Template::Tail,
            _ => false,
        }
    }
    pub fn is_template_head(&self) -> bool {
        match self {
            RefToken::Template(t) => t == &Template::Head,
            _ => false,
        }
    }
    pub fn is_template_body(&self) -> bool {
        match self {
            RefToken::Template(t) => t == &Template::Body,
            _ => false,
        }
    }

    pub fn is_eof(&self) -> bool {
        self == &RefToken::EoF
    }
}

pub fn token<'a, I>() -> impl Parser<Input = I, Output = RefToken>
where
    I: RangeStream<Item = char, Range = &'a str>,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((token_not_eof(), end_of_input()))
}

pub(crate) fn token_not_eof<'a, I>() -> impl Parser<Input = I, Output = RefToken>
where
    I: RangeStream<Item = char, Range = &'a str>,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        comment(),
        boolean_literal(),
        attempt(keyword()),
        attempt(ident()),
        attempt(null_literal()),
        attempt(number()),
        attempt(string_lit()),
        attempt(punctuation()),
        attempt(template_start()),
    ))
}

pub fn boolean_literal<I>() -> impl Parser<Input = I, Output = RefToken>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((attempt(true_literal()), attempt(false_literal()))).map(RefToken::Boolean)
}

fn true_literal<I>() -> impl Parser<Input = I, Output = bool>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("true").skip(not_followed_by(super::super::tokens::raw_ident_part())))
        .map(|_| true)
}

pub(crate) fn raw_ident_part<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(choice((
        super::super::unicode::id_continue(),
        c_char('$'),
        c_char('\\').skip(c_char('u')),
        c_char('\u{200C}'),
        c_char('\u{200D}'),
    )))
}

fn false_literal<I>() -> impl Parser<Input = I, Output = bool>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("false").skip(not_followed_by(super::super::tokens::raw_ident_part())))
        .map(|_| false)
}

pub fn null_literal<I>() -> impl Parser<Input = I, Output = RefToken>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("null").skip(not_followed_by(super::super::tokens::raw_ident_part())))
        .map(|_| RefToken::Null)
}

pub fn ident<I>() -> impl Parser<Input = I, Output = RefToken>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    recognize((ident_start(), many::<String, _>(ident_part()))).map(|_| RefToken::Ident)
}

pub(crate) fn ident_part<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(choice((attempt(ident_start()), attempt(raw_ident_part()))))
}

fn ident_start<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(choice((
        attempt(unicode::id_start().map(|c: char| c.to_string())),
        attempt(c_char('$').map(|c: char| c.to_string())),
        attempt(c_char('_').map(|c: char| c.to_string())),
        attempt(unicode::char_literal()),
    )))
}

pub(crate) fn end_of_input<I>() -> impl Parser<Input = I, Output = RefToken>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(eof()).map(|_| RefToken::EoF)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ref_tokens() {
        static TOKENS: &[&str] = &[
            "//this is a comment",
            "/*this is a\nmulti-line comment*/",
            "<!-- This is an HTML comment -->",
            "<!-- This is an HTML comment --> with a trailer",
            "implements",
            "interface",
            "package",
            "private",
            "protected",
            "public",
            "static",
            "yield",
            "let",
            "enum",
            "export",
            "import",
            "super",
            "break",
            "case",
            "catch",
            "continue",
            "debugger",
            "default",
            "delete",
            "do",
            "else",
            "finally",
            "for",
            "function",
            "if",
            "instanceof",
            "in",
            "new",
            "return",
            "switch",
            "this",
            "throw",
            "try",
            "typeof",
            "var",
            "void",
            "while",
            "with",
            "0",
            "00",
            "1234567890",
            "01234567",
            "0.",
            "0.00",
            "10.00",
            ".0",
            ".0",
            "0e0",
            "0E0",
            "0.e0",
            "0.00e+0",
            ".00e-0",
            "0x0",
            "0X0",
            "0x0123456789abcdefABCDEF",
            "0b0",
            "0b0100101",
            "0o0",
            "0o777",
            "2e308",
            "{",
            "}",
            "(",
            ")",
            ".",
            ";",
            ",",
            "[",
            "]",
            ":",
            "?",
            "~",
            ">",
            "<",
            "=",
            "!",
            "+",
            "-",
            "/",
            "*",
            "%",
            "&",
            "|",
            "^",
            ">>>=",
            "...",
            "===",
            "!==",
            ">>>",
            "<<=",
            ">>=",
            "**=",
            "&&",
            "||",
            "==",
            "!=",
            "+=",
            "-=",
            "*=",
            "/=",
            "++",
            "--",
            "<<",
            ">>",
            "&=",
            "|=",
            "^=",
            "%=",
            "<=",
            ">=",
            "=>",
            "**",
            "$",
            "_",
            "\\u0078",
            "x$",
            "x_",
            "x\\u0030",
            "xa",
            "x0",
            "x0a",
            "x0123456789",
            "qwertyuiopasdfghjklzxcvbnm",
            "QWERTYUIOPASDFGHJKLZXCVBNM",
            "œ一",
            "ǻ둘",
            "ɤ〩",
            "φ",
            "ﬁⅷ",
            "ユニコード",
            "x\u{200c}\u{200d}",
            "true",
            "false",
            "null",
            "`things and stuff times ${",
            "`things and stuff`",
            "`a\\${b`",
            "`\\0\\n\\x0A\\u000A\\u{A}${",
        ];
        for t in TOKENS {
            let s = t.to_string();
            let s = s.as_str();
            let _r = token().easy_parse(s).unwrap();
        }
    }
}
