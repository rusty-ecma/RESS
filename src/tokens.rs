use combine::{
    between,
    choice,
    eof,
    many,
    many1,
    not_followed_by,
    optional,
    Parser,
    Stream,
    try,
    parser::{
        char::{
            char as c_char,
            digit,
            hex_digit,
            oct_digit,
            spaces,
            string,
        },
        item::satisfy
    },
    error::ParseError,
};
use regex;
use unicode;

#[derive(Debug, PartialEq, Clone)]
/// The representation of a single JS token
pub enum Token {
    /// True of false, will contain the value
    Boolean(bool),
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
    Numeric(String),
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
    Template(String),
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
#[derive(Debug, PartialEq)]
pub enum NumericToken {
    Decimal(String),
    Hex(String),
    Bin(String),
    Octal(String)
}


pub fn token<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        choice((
            try(comment()),
            try(boolean_literal()),
            try(keyword()),
            try(ident()),
            try(null_literal()),
            try(numeric_literal()),
            try(regex::literal()),
            try(punctuation()),
            try(string_literal()),
            try(end_of_input())
            //TODO add template
        ))
    ).map(|t| t)
}

pub(crate) fn boolean_literal<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        string("true"),
        string("false"),
    )).map(|t| Token::Boolean(t == "true"))
}

pub(crate) fn end_of_input<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    eof().map(|_| Token::EoF)
}

pub(crate) fn ident<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        ident_start(),
        many(
            ident_part()
        )
    ).map(|(start, body): (char, String)| {
        let mut ret = String::new();
        ret.push(start);
        ret.push_str(&body);
        Token::Ident(ret)
    })
}

pub(crate) fn keyword<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        future_reserved(),
        strict_mode_reserved(),
        restricted(),
        reserved(),
    )).skip(not_followed_by(ident_part())).map(|t| t)
}

pub(crate) fn reserved<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
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
    ]).map(|t| Token::Keyword(t.to_owned()))
}

pub(crate) fn future_reserved<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(string("export")),
        try(string("import")),
        try(string("super")),
        try(string("enum")),
    )).map(|t| Token::Keyword(t.to_owned()))
}

pub(crate) fn strict_mode_reserved<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
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
    )).map(|t| Token::Keyword(t.to_owned()))
}

pub(crate) fn restricted<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(string("eval")),
        try(string("arguments")),
    )).map(|t| Token::Keyword(t.to_owned()))
}

pub(crate) fn null_literal<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("null")
        .map(|_| Token::Null)
}

pub(crate) fn numeric_literal<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(bin_literal()),
        try(octal_literal()),
        try(hex_literal()),
        try(decimal_literal()),
    )).map(|t| t)
}

fn decimal_literal<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(full_decimal_literal()),
        try(no_leading_decimal()),
    ))

}

fn full_decimal_literal<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        optional(choice([c_char('-'), c_char('+')])),
        //any number of digits
        many1(digit()),
        //optionally followed by a . and any number of digits
        optional((
            c_char('.'),
            many(digit()),
        )),
        //optionally followed by e|E and any number of digits
        optional((
            choice((c_char('e'), c_char('E'))),
            many1(digit())
        ))
    ).map(|t: (Option<char>, String, Option<(char, String)>, Option<(char, String)>)| {
        let mut ret = String::new();
        if let Some(sign) = t.0 {
            ret.push(sign);
        }
        ret.push_str(&t.1);
        if let Some(decimal) = t.2 {
            ret.push(decimal.0);
            ret.push_str(&decimal.1);
        }
        if let Some(exp) = t.3 {
            ret.push(exp.0);
            ret.push_str(&exp.1);
        }
        Token::Numeric(ret)
    })
}

fn no_leading_decimal<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        optional(choice([c_char('-'), c_char('+')])),
        c_char('.'),
        many1(digit()),
        optional((
            choice([c_char('e'), c_char('E')]),
            many1(digit())
        ))
    ).map(|t: (Option<char>, char, String, Option<(char, String)>)| {
        let mut ret = String::new();
        if let Some(sign) = t.0 {
            ret.push(sign);
        }
        ret.push(t.1);
        ret.push_str(&t.2);
        if let Some(exp) = t.3 {
            ret.push(exp.0);
            ret.push_str(&exp.1);
        }
        Token::Numeric(ret)
    })
}

fn hex_literal<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        optional(choice([c_char('-'), c_char('+')])),
        c_char('0'),
        choice([c_char('x'), c_char('X')]),
        many1(hex_digit())
    ).map(|t: (Option<char>, char, char, String)| {
        let mut ret = String::new();
        if let Some(sign) = t.0 {
            ret.push(sign);
        }
        ret.push(t.1);
        ret.push(t.2);
        ret.push_str(&t.3);
        Token::Numeric(ret)
    })
}

fn bin_literal<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        optional(choice([c_char('-'), c_char('+')])),
        c_char('0'),
        choice([c_char('b'), c_char('B')]),
        many1(choice([c_char('1'), c_char('0')]))
    ).map(|t: (Option<char>, char, char, String)| {
        let mut ret = String::new();
        if let Some(sign) = t.0 {
            ret.push(sign);
        }
        ret.push(t.1);
        ret.push(t.2);
        ret.push_str(&t.3);
        Token::Numeric(ret)
    })
}

fn octal_literal<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        optional(choice([c_char('-'), c_char('+')])),
        c_char('0'),
        choice([c_char('o'), c_char('O')]),
        many1(oct_digit())
    ).map(|t: (Option<char>, char, char, String)| {
        let mut ret = String::new();
        if let Some(sign) = t.0 {
            ret.push(sign);
        }
        ret.push(t.1);
        ret.push(t.2);
        ret.push_str(&t.3);
        Token::Numeric(ret)
    })
}

pub(crate) fn punctuation<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(multi_punct()),
        try(single_punct()),
    )).map(|t: String| Token::Punct(t))
}

fn single_punct<I>() -> impl Parser<Input = I, Output = String>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(normal_punct()),
        try(div_punct()),
    )).map(|c: String| c)
}

fn normal_punct<I>() -> impl Parser<Input = I, Output = String>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice([
        c_char('{'), c_char('}'), c_char('('), c_char(')'), c_char('.'),
        c_char(';'), c_char(','), c_char('['), c_char(']'), c_char(':'),
        c_char('?'), c_char('~'), c_char('>'), c_char('<'), c_char('='),
        c_char('!'), c_char('+'), c_char('-'), c_char('*'),
        c_char('%'), c_char('&'), c_char('|'), c_char('^'),
    ]).map(|c: char| c.to_string())
}

fn div_punct<I>() -> impl Parser<Input = I, Output = String>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("/")
        .skip(
            not_followed_by(c_char('*'))
        ).map(|c| c.to_string())
}

fn multi_punct<I>() -> impl Parser<Input = I, Output = String>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice([
        //4 char
        try(string(">>>=")),
        //3 char
        try(string("...")),
        try(string("===")), try(string("!==")), try(string(">>>")),
        try(string("<<=")), try(string(">>=")), try(string("**=")),
        //2 char
        try(string("&&")), try(string("||")), try(string("==")), try(string("!=")),
        try(string("+=")), try(string("-=")), try(string("*=")), try(string("/=")),
        try(string("++")), try(string("--")), try(string("<<")), try(string(">>")), 
        try(string("&=")), try(string("|=")), try(string("^=")), try(string("%=")),
        try(string("<=")), try(string(">=")), try(string("=>")), try(string("**")),
    ]).map(|t| t.to_string())
}

pub(crate) fn string_literal<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(single_quote()),
        try(double_quote()),
    )).map(|s| Token::String(s.to_owned()))
}

fn single_quote<I>() -> impl Parser<Input = I, Output = String>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        between(
            c_char('\''),
            c_char('\''),
            many(single_quoted_content())
        )//TODO: better string literal letter construct
    ).map(|t: String| t)
}

fn single_quoted_content<I>() -> impl Parser<Input = I, Output = String>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(escaped('\'').map(|c: char| format!("\\{}", c))),
        try(escaped('\\').map(|c: char| c.to_string())),
        try(string_continuation()),
        try(satisfy(|c: char| c != '\'' && c != '\n' && c != '\r').map(|c: char| c.to_string())),
    )).map(|s: String| s)
}

fn string_continuation<I>() -> impl Parser<Input = I, Output = String>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        c_char('\\'),
        line_terminator_sequence()
    ).skip(spaces())
    .map(|_| {
        String::new()
    })
}

fn double_quote<I>() -> impl Parser<Input = I, Output = String>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(
        c_char('"'),
        c_char('"'),
        many(double_quoted_content())
    )
    .map(|t: String| t)
}

pub(crate) fn escaped<I>(q: char) -> impl Parser<Input = I, Output = char>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    c_char('\\').and(c_char(q)).map(|(_slash, c): (char, char)| c)
}

fn double_quoted_content<I>() -> impl Parser<Input = I, Output = String>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(escaped('"').map(|c: char| format!("\\{}", c))),
        try(escaped('\\').map(|c: char| c.to_string())),
        try(string_continuation()),
        try(satisfy(|c: char| c != '"' && c != '\n' && c != '\r').map(|c: char| c.to_string())),
    )).map(|s: String| s)
}

pub(crate) fn comment<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        choice((
            try(multi_comment()),
            try(single_comment()),
        )).map(|t: Token| t)
    )
}

pub(crate) fn single_comment<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        string("//"),
        many(regex::source_char_not_line_term()),
    ).map(|(_, content): (_, String)| Token::Comment(content.to_owned()))
}
use combine::parser::repeat::take_until;
pub(crate) fn multi_comment<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        multi_line_comment_start(),
        take_until(
                try(string("*/")),
        ),
        multi_line_comment_end()
    ).map(|(_s, c, _e): (String, String, String)| {
        let ret = c.lines().map(|l| l.trim()).collect::<Vec<&str>>().join("\n");
        Token::Comment(ret)
    })
}

fn multi_line_comment_start<I>() -> impl Parser<Input = I, Output = String>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        string("/*")
    ).map(|s| s.to_string())
}

fn multi_line_comment_end<I>() -> impl Parser<Input = I, Output = String>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        string("*/")
    ).map(|s| s.to_string())
}

fn unicode_char<I>() -> impl Parser<Input = I, Output = char>
    where  I: Stream<Item = char>,
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
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(unicode_char()),
        try(c_char('$')),
        try(c_char('_')),
        try(unicode::char_literal())
    )).map(|c: char| c)
}

pub(crate) fn ident_part<I>() -> impl Parser<Input = I, Output = char>
    where  I: Stream<Item = char>,
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

fn line_terminator<I>() -> impl Parser<Input = I, Output = char>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice([
        try(c_char('\u{000A}')),
        try(c_char('\u{000D}')),
        try(c_char('\u{2028}')),
        try(c_char('\u{2029}')),
    ]).map(|c: char| c)
}

fn line_terminator_sequence<I>() -> impl Parser<Input = I, Output = String>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(string("\r\n").map(|s: &str| s.to_string())),
        try(line_terminator().map(|c: char| c.to_string())),
    )).map(|s: String| s)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn bool() {
        let t = super::boolean_literal().parse("true").unwrap();
        let f = super::boolean_literal().parse("false").unwrap();
        assert_eq!(t, (Token::Boolean(true), ""));
        assert_eq!(f, (Token::Boolean(false), ""));
    }

    #[test]
    fn eof() {
        let e = super::end_of_input().parse("").unwrap();
        assert_eq!(e, (Token::EoF, ""));
    }

    #[test]
    fn future_reserved() {
        let keywords = [
            "enum", "export", "import", "super",
        ];
        for keyword in keywords.iter() {
            let k = super::future_reserved().parse(keyword.clone()).unwrap();
            assert_eq!(k, (Token::Keyword(keyword.to_string()), ""))
        }
        match super::future_reserved().parse("junk") {
            Ok(k) => panic!("parsed junk as {:?}", k),
            Err(_) => (),
        }
    }

    #[test]
    fn strict_mode_reserved() {
        let keywords = [
            "implements", "interface", "package", "private", "protected",
            "public", "static", "yield", "let",
        ];
        for keyword in keywords.iter() {
            let k = super::strict_mode_reserved().parse(keyword.clone()).unwrap();
            assert_eq!(k, (Token::Keyword(keyword.to_string()), ""));
        }
        match super::strict_mode_reserved().parse("junk") {
            Ok(k) => panic!("parsed junk as {:?}", k),
            Err(_) => (),
        }
    }

    #[test]
    fn restricted_reserved() {
        let k = super::restricted().parse("eval").unwrap();
        assert_eq!(k, (Token::Keyword("eval".into()), ""));
        let k2 = super::restricted().parse("arguments").unwrap();
        assert_eq!(k2, (Token::Keyword("arguments".into()), ""))
    }

    #[test]
    fn reserved_keywords() {
        let keys = vec![
            "break", "case", "catch", "continue", "debugger",
            "default", "delete", "do", "else", "finally",
            "for", "function", "if", "instanceof", "in",
            "new", "return", "switch", "this", "throw",
            "try", "typeof", "var", "void", "while",
            "with",];
        for key in keys {
            let k = reserved().parse(key.clone()).unwrap();
            assert_eq!(k, (Token::Keyword(key.to_owned()), ""));
        }
    }

    #[test]
    fn keywords_test() {
        let keys = vec![
            "enum", "export", "import", "super", "implements",
            "interface", "package", "private", "protected", "public",
            "static", "yield", "let", "eval", "break",
            "case", "catch", "continue", "debugger", "default",
            "delete", "do", "else", "finally", "for",
            "function", "if", "instanceof", "in", "new",
            "return", "switch", "this", "throw", "try",
            "typeof", "var", "void", "while", "with",
        ];
        for key in keys {
            let k = keyword().parse(key.clone()).unwrap();
            assert_eq!(k, (Token::Keyword(key.to_owned()), ""));
        }
    }
    #[test]
    fn full_decimal() {
        let vals = vec![
            "0.1", "1.1", "888888888.88888888888",
            "+8", "-6", "+1E5", "-1E2", "1.8876e2",
            "-1.009987e87"
        ];
        for val in vals {
            let d = full_decimal_literal().parse(val.clone()).unwrap();
            assert_eq!(d, (Token::Numeric(val.to_owned()), ""));
        }
        if let Ok(_) = full_decimal_literal().parse(".00") {
            panic!("parsed .00 as full decimal literal");
        }
    }

    #[test]
    fn no_leading() {
        let vals = vec![
            ".2", "-.2", ".2E1", "+.8", "+.2E4",
            ".7e34", "-.7e2", "+.4e5"
        ];
        for val in vals {
            let d = no_leading_decimal().parse(val.clone()).unwrap();
            assert_eq!(d, (Token::Numeric(val.to_owned()), ""))
        }
        if let Ok(_) = no_leading_decimal().parse("00.0") {
            panic!("parsed 00.0 as no leading decimal")
        }
    }

    #[test]
    fn hex() {
        let vals = vec![
            "0x123", "0X456", "-0x789", "+0X0abc",
            "0xdef", "0xABC", "0xDEF"
        ];
        for val in vals {
            let h = hex_literal().parse(val.clone()).unwrap();
            assert_eq!(h, (Token::Numeric(val.to_owned()), ""))
        }

        if let Ok(_) = hex_literal().parse("555") {
            panic!("parsed 555 as hex literal")
        }
    }
    #[test]
    fn bin() {
        let vals = vec![
            "0b000", "0B111", "-0B0101", "+0b1010",
        ];
        for val in vals {
            let h = bin_literal().parse(val.clone()).unwrap();
            assert_eq!(h, (Token::Numeric(val.to_owned()), ""))
        }

        if let Ok(_) = bin_literal().parse("0b") {
            panic!("parsed 0b as hex literal")
        }
    }

    #[test]
    fn oct() {
        let vals = vec![
            "0o7", "0O554", "-0o12345670", "+0O12345670",
        ];
        for val in vals {
            let h = octal_literal().parse(val.clone()).unwrap();
            assert_eq!(h, (Token::Numeric(val.to_owned()), ""))
        }

        if let Ok(_) = octal_literal().parse("0O8") {
            panic!("parsed 0O8 as hex literal")
        }
    }

    #[test]
    fn decimal() {
        let vals = vec![
            "0.1", "1.1", "888888888.88888888888",
            "+8", "-6", "+1E5", "-1E2", "1.8876e2",
            "-1.009987e87",".2", "-.2", ".2E1", "+.8", "+.2E4",
            ".7e34", "-.7e2", "+.4e5", 
        ];
        for val in vals {
            let d = token().parse(val.clone()).unwrap();
            assert_eq!(d, (Token::Numeric(val.to_owned()), ""));
        }
        if let Ok(f) = token().parse("asdfghjk") {
            match f {
                (Token::Numeric(d), _) => panic!("parsed asdfghjk as decimal {:?}", d),
                _ => ()
            }
        }
    }

    #[test]
    fn number() {
        let vals = vec![
            "0.1", "1.1", "888888888.88888888888",
            "+8", "-6", "+1E5", "-1E2", "1.8876e2",
            "-1.009987e87",".2", "-.2", ".2E1", "+.8", "+.2E4",
            ".7e34", "-.7e2", "+.4e5", "0x123", "0X456", "-0x789", "+0X0abc",
            "0xdef", "0xABC", "0xDEF", "0o7", "0O554", 
            "-0o12345670", "+0O12345670","0b000", "0B111", "-0B0101", "+0b1010",
        ];
        for val in vals {
            let d = token().parse(val.clone()).unwrap();
            assert_eq!(d, (Token::Numeric(val.to_owned()), ""));
        }
        match token().parse("asdfghjk").unwrap() {
            (Token::Numeric(f), "") => panic!("parsed asdfghjk as number {:?}", f),
            _ => ()
        }
    }

    #[test]
    fn punct() {
        let single = vec!["{", "}", "(", ")", ".",
        ";", ",", "[", "]", ":",
        "?", "~", ">", "<", "=",
        "!", "+", "-", "/", "*",
        "%", "&", "|", "^",];
        for p in single.clone() {
            let t = token().parse(p.clone()).unwrap();
            assert_eq!(t, (Token::Punct(p.to_string()), ""));
        }
        let multi = vec![
            ">>>=",
            //3 char
            "...",
            "===", "!==", ">>>",
            "<<=", ">>=", "**=",
            //2 char
            "&&", "||", "==", "!=",
            "+=", "-=", "*=", "/=",
            "++", "--", "<<", ">>", 
            "&=", "|=", "^=", "%=",
            "<=", ">=", "=>", "**",
        ];
        for p in multi.clone() {
            let t = token().parse(p.clone()).unwrap();
            assert_eq!(t, (Token::Punct(p.to_string()), ""));
        }
        for p in single.iter().chain(multi.iter()) {
            let t = token().parse(p.clone()).unwrap();
            assert_eq!(t, (Token::Punct(p.to_string()), ""))
        }
    }

    #[test]
    fn strings() {
        let strings = vec![
            "junk and places", "things and stuff", 
            "✨✨✨✨ ✨✨✨✨",
        ];
        for s in strings.into_iter() {
            println!("testing {}", s);
            let dq_test = format!("\"{}\"", &s.clone());
            let dq = token().parse(dq_test.as_str()).unwrap();
            let sq_test = format!("'{}'", &s.clone());
            let sq = token().parse(sq_test.as_str()).unwrap();
            assert_eq!(dq, (Token::String(s.to_string().clone()), ""));
            assert_eq!(dq, sq);
        }
        let continued = r#"things and stuff \
        and people and places"#;
        let double = format!("\"{}\"", continued.clone());
        let single = format!("'{}'", continued.clone());
        let target = "things and stuff and people and places";
        let d_r = token().parse(double.as_str()).unwrap();
        let s_r = token().parse(single.as_str()).unwrap();
        assert_eq!(d_r, (Token::String(target.to_string()), ""));
        assert_eq!(s_r, d_r);
    }

    #[test]
    fn regex_tests() {
        let tests = vec![
            "/.jsx?/", "/.+/",
            "/(a-fA-F0-9)/g"
        ];
        for test in tests {
            let r = token().parse(test.clone()).unwrap();
            let mut parts = test.split('/');
            let _empty = parts.next();
            let pattern = parts.next().unwrap();
            let flags = if let Some(f) = parts.next() {
                if f == "" {
                    None
                } else {
                    Some(f.to_string())
                }
            } else {
                None
            };
            assert_eq!(r, (Token::RegEx(pattern.to_string(), flags), ""))
        }
    }

    #[test]
    fn ident_tests() {
        let idents = vec![
            "$", "x", "thing", "num",
            "stuff", "anotherThing", "snake_thing",
            "junk", "_", "_private"
        ];
        for i in idents {
            let t = token().parse(i.clone()).unwrap();
            assert_eq!(t, (Token::Ident(i.to_owned()), ""))
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
            let comment_contents =
                test.lines().map(|l| {
                    l.trim().replace("//", "").replace("/*", "").replace("*/", "")
                }).collect::<Vec<String>>().join("\n");
            assert_eq!(p, (Token::Comment(comment_contents), ""));
        }
    }
}


