use combine::{
    between, choice, error::ParseError, many, optional, parser::char::char as c_char, satisfy, try,
    Parser, Stream,
};
use tokens::{escaped, ident_part, Token};
/// Parse a regex literal
pub(crate) fn literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        between(c_char('/'), c_char('/'), regex_body()),
        optional(regex_flags()),
    ).map(|(body, flags): (String, Option<String>)| {
        let f = if flags == Some(String::new()) {
            None
        } else {
            flags
        };
        Token::RegEx(body, f)
    })
}
/// Parse the body portion of the regex literal
fn regex_body<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (regex_first_char(), many(regex_char())).map(|(c, s): (String, String)| format!("{}{}", c, s))
}

fn regex_flags<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    many(ident_part()).map(|s: String| s)
}

fn regex_first_char<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(source_char_not_line_term().map(|c: char| c.to_string())),
        try(regular_expression_backslash_sequence()),
        try(regular_expression_class()),
    )).map(|c: String| c)
}

fn regex_char<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(escaped('/')),
        try(satisfy(|c: char| {
            c as u16 <= 4095 && c != '/' && c != '\r' && c != '\n'
        })),
    )).map(|c: char| {
        if c == '/' {
            String::from("\\/")
        } else {
            c.to_string()
        }
    })
}

fn regular_expression_class<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(
        c_char('['),
        c_char(']'),
        many(regular_expression_class_char()),
    ).map(|s: String| format!("[{}]", s))
}

fn regular_expression_class_char<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(
            satisfy(|c: char| c as u32 <= 4095 && c != '\n' && c != '\u{005C}' && c != '\u{005D}')
                .map(|c: char| c.to_string()),
        ),
        try(regular_expression_backslash_sequence()),
    )).map(|s: String| s)
}
pub(crate) fn source_char_not_line_term<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|c: char| c as u32 <= 4095 && c != '\n' && c != '\u{005C}').map(|c: char| c)
}

fn regular_expression_backslash_sequence<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    c_char('\\')
        .and(source_char_not_line_term())
        .map(|(slash, c): (char, char)| format!("{}{}", slash, c))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn regex_test() {
        let simple = r#"/[a-zA-Z]/"#;
        let s_r = super::literal().parse(simple.clone()).unwrap();
        println!("simple: {:?}", s_r);
        assert_eq!(s_r, (Token::RegEx(simple[1..9].to_string(), None), ""));
        let flagged = r#"/[0-9]+/g"#;
        let f_r = super::literal().parse(flagged).unwrap();
        println!("flagged: {:?}", f_r);
        assert_eq!(
            f_r,
            (
                Token::RegEx(flagged[1..7].to_string(), Some("g".to_string())),
                ""
            )
        );
        let complex = r#"/^[\s\uFEFF\xA0]+|[\s\uFEFF\xA0]+$/g"#;
        let sc_r = (
            c_char('/'),
            regex_first_char(),
            regular_expression_class(),
            source_char_not_line_term(),
            source_char_not_line_term(),
            regular_expression_class(),
            source_char_not_line_term(),
            source_char_not_line_term(),
            c_char('/'),
            regex_flags(),
        ).parse(complex.clone());
        println!("as source chars {:?}", sc_r);
        let r = super::literal().parse(complex.clone()).unwrap();
        println!("complex result: {:?}", r);
        let escaped = r#"/\D/"#;
        let e_r = (
            c_char('/'),
            regular_expression_backslash_sequence(),
            c_char('/'),
        ).parse(escaped)
            .unwrap();
        println!("escaped: {:?}", e_r);
    }
}
