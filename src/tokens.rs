use combine::{
    choice, eof, error::ParseError, many, not_followed_by,
    parser::{
        char::{char as c_char, string},
        repeat::take_until,
    },
    try, Parser, Stream,
};

use regex;
use strings;
use unicode;
use numeric;

pub struct Token {
    data: TokenData,
    span: Span,
}

pub struct Span {
    start: usize,
    end: usize,
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
    Keyword(String),
    /// A `null` literal value
    Null,
    /// A number, this includes integers (`1`), decimals (`0.1`),
    /// hex (`0x8f`), binary (`0b010011010`), and octal (`0o273`)
    Numeric(numeric::Token),
    /// A punctuation mark, this includes all mathematical operators
    /// logical operators and general syntax punctuation
    Punct(String),
    /// A string literal, either double or single quoted, the associated
    /// value will be the unquoted string
    String(String),
    /// A regex literal (`/[a-fA-F0-9]+/g`) the first associated value
    /// will be the pattern, the second will be the optional flags
    RegEx(String, Option<String>),
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
    Comment(String),
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

    pub fn is_punct_with(&self, s: &str) -> bool {
        self == &TokenData::punct(s)
    }

    pub fn punct(s: impl Into<String>) -> TokenData {
        TokenData::Punct(s.into())
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
        TokenData::Keyword(name.into())
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

    pub fn is_string_with(&self, s: &str) -> bool {
        self == &TokenData::string(s)
    }

    pub fn string(s: impl Into<String>) -> TokenData {
        TokenData::String(s.into())
    }

    pub fn is_regex(&self) -> bool {
        if let TokenData::RegEx(ref _b, ref _f) = self {
            true
        } else {
            false
        }
    }

    pub fn is_regex_with(&self, body: &str, flags: Option<&str>) -> bool {
        self == &TokenData::regex(body, flags)
    }

    pub fn regex(body: impl Into<String>, flags: Option<impl Into<String>>) -> TokenData {
        TokenData::RegEx(body.into(), flags.map(|s| s.into()))
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
        self == &TokenData::comment(comment)
    }

    pub fn comment(comment: impl Into<String>) -> TokenData {
        TokenData::Comment(comment.into())
    }
}

pub fn token<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (choice((
        try(comment()),
        try(boolean_literal()),
        try(keyword()),
        try(ident()),
        try(null_literal()),
        try(numeric::literal()),
        try(regex::literal()),
        try(punctuation()),
        try(strings::literal()),
        try(end_of_input()),
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

pub(crate) fn keyword<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(future_reserved()),
        try(strict_mode_reserved()),
        try(restricted()),
        try(reserved()),
    )).skip(not_followed_by(ident_part()))
        .map(|t| t)
}

pub(crate) fn reserved<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice([
        try(string("break")),
        try(string("case")),
        try(string("catch")),
        try(string("continue")),
        try(string("debugger")),
        try(string("default")),
        try(string("delete")),
        try(string("do")),
        try(string("else")),
        try(string("finally")),
        try(string("for")),
        try(string("function")),
        try(string("if")),
        try(string("instanceof")),
        try(string("in")),
        try(string("new")),
        try(string("return")),
        try(string("switch")),
        try(string("this")),
        try(string("throw")),
        try(string("try")),
        try(string("typeof")),
        try(string("var")),
        try(string("void")),
        try(string("while")),
        try(string("with")),
    ]).map(|t| TokenData::Keyword(t.to_owned()))
}

pub(crate) fn future_reserved<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(string("export")),
        try(string("import")),
        try(string("super")),
        try(string("enum")),
    )).map(|t| TokenData::Keyword(t.to_owned()))
}

pub(crate) fn strict_mode_reserved<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(string("implements")),
        try(string("interface")),
        try(string("package")),
        try(string("private")),
        try(string("protected")),
        try(string("public")),
        try(string("static")),
        try(string("yield")),
        try(string("let")),
    )).map(|t| TokenData::Keyword(t.to_owned()))
}

pub(crate) fn restricted<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(string("eval")), try(string("arguments")))).map(|t| TokenData::Keyword(t.to_owned()))
}

pub(crate) fn null_literal<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("null").map(|_| TokenData::Null)
}

pub(crate) fn punctuation<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(multi_punct()), try(single_punct()))).map(|t: String| TokenData::Punct(t))
}

fn single_punct<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(normal_punct()), try(div_punct()))).map(|c: String| c)
}

fn normal_punct<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice([
        c_char('{'),
        c_char('}'),
        c_char('('),
        c_char(')'),
        c_char('.'),
        c_char(';'),
        c_char(','),
        c_char('['),
        c_char(']'),
        c_char(':'),
        c_char('?'),
        c_char('~'),
        c_char('>'),
        c_char('<'),
        c_char('='),
        c_char('!'),
        c_char('+'),
        c_char('-'),
        c_char('*'),
        c_char('%'),
        c_char('&'),
        c_char('|'),
        c_char('^'),
    ]).map(|c: char| c.to_string())
}

fn div_punct<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("/")
        .skip(not_followed_by(c_char('*')))
        .map(|c| c.to_string())
}

fn multi_punct<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice([
        //4 char
        try(string(">>>=")),
        //3 char
        try(string("...")),
        try(string("===")),
        try(string("!==")),
        try(string(">>>")),
        try(string("<<=")),
        try(string(">>=")),
        try(string("**=")),
        //2 char
        try(string("&&")),
        try(string("||")),
        try(string("==")),
        try(string("!=")),
        try(string("+=")),
        try(string("-=")),
        try(string("*=")),
        try(string("/=")),
        try(string("++")),
        try(string("--")),
        try(string("<<")),
        try(string(">>")),
        try(string("&=")),
        try(string("|=")),
        try(string("^=")),
        try(string("%=")),
        try(string("<=")),
        try(string(">=")),
        try(string("=>")),
        try(string("**")),
    ]).map(|t| t.to_string())
}

pub(crate) fn comment<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (choice((try(multi_comment()), try(single_comment()))).map(|t: TokenData| t))
}

pub(crate) fn single_comment<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (string("//"), take_until(
        choice((
            try(strings::line_terminator_sequence()),
            try(eof().map(|_| String::new())),
            ))))
        .map(|(_, content): (_, String)| TokenData::Comment(content.to_owned()))
}

pub(crate) fn multi_comment<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        multi_line_comment_start(),
        take_until(try(string("*/"))),
        multi_line_comment_end(),
    ).map(|(_s, c, _e): (String, String, String)| {
        let ret = c.lines()
            .map(|l| l.trim())
            .collect::<Vec<&str>>()
            .join("\n");
        TokenData::Comment(ret)
    })
}

fn multi_line_comment_start<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (string("/*")).map(|s| s.to_string())
}

fn multi_line_comment_end<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (string("*/")).map(|s| s.to_string())
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
        assert_eq!(t, (TokenData::Boolean(true), ""));
        assert_eq!(f, (TokenData::Boolean(false), ""));
    }

    #[test]
    fn eof() {
        let e = super::end_of_input().parse("").unwrap();
        assert_eq!(e, (TokenData::EoF, ""));
    }

    #[test]
    fn future_reserved() {
        let keywords = ["enum", "export", "import", "super"];
        for keyword in keywords.iter() {
            let k = super::future_reserved().parse(keyword.clone()).unwrap();
            assert_eq!(k, (TokenData::Keyword(keyword.to_string()), ""))
        }
        match super::future_reserved().parse("junk") {
            Ok(k) => panic!("parsed junk as {:?}", k),
            Err(_) => (),
        }
    }

    #[test]
    fn strict_mode_reserved() {
        let keywords = [
            "implements",
            "interface",
            "package",
            "private",
            "protected",
            "public",
            "static",
            "yield",
            "let",
        ];
        for keyword in keywords.iter() {
            let k = super::strict_mode_reserved()
                .parse(keyword.clone())
                .unwrap();
            assert_eq!(k, (TokenData::Keyword(keyword.to_string()), ""));
        }
        match super::strict_mode_reserved().parse("junk") {
            Ok(k) => panic!("parsed junk as {:?}", k),
            Err(_) => (),
        }
    }

    #[test]
    fn restricted_reserved() {
        let k = super::restricted().parse("eval").unwrap();
        assert_eq!(k, (TokenData::Keyword("eval".into()), ""));
        let k2 = super::restricted().parse("arguments").unwrap();
        assert_eq!(k2, (TokenData::Keyword("arguments".into()), ""))
    }

    #[test]
    fn reserved_keywords() {
        let keys = vec![
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
        ];
        for key in keys {
            let k = reserved().parse(key.clone()).unwrap();
            assert_eq!(k, (TokenData::Keyword(key.to_owned()), ""));
        }
    }

    #[test]
    fn keywords_test() {
        let keys = vec![
            "enum",
            "export",
            "import",
            "super",
            "implements",
            "interface",
            "package",
            "private",
            "protected",
            "public",
            "static",
            "yield",
            "let",
            "eval",
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
        ];
        for key in keys {
            let k = keyword().parse(key.clone()).unwrap();
            assert_eq!(k, (TokenData::Keyword(key.to_owned()), ""));
        }
    }
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
            let d = full_decimal_literal().parse(val.clone()).unwrap();
            assert_eq!(d, (TokenData::Numeric(val.to_owned()), ""));
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
            let d = no_leading_decimal().parse(val.clone()).unwrap();
            assert_eq!(d, (TokenData::Numeric(val.to_owned()), ""))
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
            let h = hex_literal().parse(val.clone()).unwrap();
            assert_eq!(h, (TokenData::Numeric(val.to_owned()), ""))
        }

        if let Ok(_) = hex_literal().parse("555") {
            panic!("parsed 555 as hex literal")
        }
    }
    #[test]
    fn bin() {
        let vals = vec!["0b000", "0B111", "-0B0101", "+0b1010"];
        for val in vals {
            let h = bin_literal().parse(val.clone()).unwrap();
            assert_eq!(h, (TokenData::Numeric(val.to_owned()), ""))
        }

        if let Ok(_) = bin_literal().parse("0b") {
            panic!("parsed 0b as hex literal")
        }
    }

    #[test]
    fn oct() {
        let vals = vec!["0o7", "0O554", "-0o12345670", "+0O12345670"];
        for val in vals {
            let h = octal_literal().parse(val.clone()).unwrap();
            assert_eq!(h, (TokenData::Numeric(val.to_owned()), ""))
        }

        if let Ok(_) = octal_literal().parse("0O8") {
            panic!("parsed 0O8 as hex literal")
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
            let d = token().parse(val.clone()).unwrap();
            assert_eq!(d, (TokenData::Numeric(val.to_owned()), ""));
        }
        if let Ok(f) = token().parse("asdfghjk") {
            match f {
                (TokenData::Numeric(d), _) => panic!("parsed asdfghjk as decimal {:?}", d),
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
            let d = token().parse(val.clone()).unwrap();
            assert_eq!(d, (TokenData::Numeric(val.to_owned()), ""));
        }
        match token().parse("asdfghjk").unwrap() {
            (TokenData::Numeric(f), "") => panic!("parsed asdfghjk as number {:?}", f),
            _ => (),
        }
    }

    #[test]
    fn punct() {
        let single = vec![
            "{", "}", "(", ")", ".", ";", ",", "[", "]", ":", "?", "~", ">", "<", "=", "!", "+",
            "-", "/", "*", "%", "&", "|", "^",
        ];
        for p in single.clone() {
            let t = token().parse(p.clone()).unwrap();
            assert_eq!(t, (TokenData::Punct(p.to_string()), ""));
        }
        let multi = vec![
            ">>>=",
            //3 char
            "...",
            "===",
            "!==",
            ">>>",
            "<<=",
            ">>=",
            "**=",
            //2 char
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
        ];
        for p in multi.clone() {
            let t = token().parse(p.clone()).unwrap();
            assert_eq!(t, (TokenData::Punct(p.to_string()), ""));
        }
        for p in single.iter().chain(multi.iter()) {
            let t = token().parse(p.clone()).unwrap();
            assert_eq!(t, (TokenData::Punct(p.to_string()), ""))
        }
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
    #[test]
    fn comments_test() {
        let tests = vec![
            "//single line comments",
            "// another one with a space",
            "/*inline multi comments*/",
            "/*multi line comments
            * that have extra decoration
            * to help with readability
            */",
        ];
        for test in tests {
            let p = comment().parse(test.clone()).unwrap();
            let comment_contents = test.lines()
                .map(|l| {
                    l.trim()
                        .replace("//", "")
                        .replace("/*", "")
                        .replace("*/", "")
                })
                .collect::<Vec<String>>()
                .join("\n");
            assert_eq!(p, (TokenData::Comment(comment_contents), ""));
        }
    }
}
