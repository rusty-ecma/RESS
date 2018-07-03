use combine::{
    between, choice, error::ParseError, many,
    parser::{
        char::{char as c_char, string, spaces}, item::satisfy,
    },
    try, Parser, Stream, not_followed_by,
};
use tokens::*;
use regex::literal as regex_literal;
use super::escaped;

pub(crate) fn literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(single_quote()), try(double_quote()))).map(|s| Token::String(s.to_owned()))
}

fn single_quote<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
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
where
    I: Stream<Item = char>,
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
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (c_char('\\'), line_terminator_sequence())
        .skip(spaces())
        .map(|_| String::new())
}

fn double_quote<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(c_char('"'), c_char('"'), many(double_quoted_content())).map(|t: String| t)
}

fn double_quoted_content<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(escaped('"').map(|c: char| format!("\\{}", c))),
        try(escaped('\\').map(|c: char| c.to_string())),
        try(string_continuation()),
        try(satisfy(|c: char| c != '"' && c != '\n' && c != '\r').map(|c: char| c.to_string())),
    )).map(|s: String| s)
}


fn line_terminator<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
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
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(string("\r\n").map(|s: &str| s.to_string())),
        try(line_terminator().map(|c: char| c.to_string())),
    )).map(|s: String| s)
}

pub fn template<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(no_sub_template()),
        try(actual_template())
    ))
}

fn no_sub_template<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(
        c_char('`'),
        c_char('`'),
        many(template_char())
    ).map(|s: String| Token::Template(vec![Token::String(s)]))
}

fn actual_template<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        template_head_pair(),
        many(middle_replace_pair()),
        template_tail(),
    ).map(|(h, m, t): (Vec<Token>, Vec<Vec<Token>>, Token)| {
        let mut ret = h;
        for mid in m {
            ret.extend(mid);
        }
        ret.push(t);
        Token::Template(ret)
    })
}

fn template_head_pair<I>() -> impl Parser<Input = I, Output = Vec<Token>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        template_head(),
        replaced()
    ).map(|(token, list):(Token, Vec<Token>)| {
        let mut ret = vec![token];
        ret.extend(list);
        ret
    })
}

fn middle_replace_pair<I>() -> impl Parser<Input = I, Output = Vec<Token>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        template_middle(),
        replaced()
    ).map(|t: (Token, Vec<Token>)| {
        let mut ret = vec![];
        ret.push(t.0);
        ret.extend(t.1);
        ret
    })
}


fn template_middle<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(
        string("}"),
        string("${"),
        many(template_char())
    ).map(|s: String| Token::String(s))
}

fn template_head<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(
        string("`"),
        string("${"),
        many(template_char())
    ).map(|s: String| Token::String(s))
}

fn template_tail<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(
        string("}"),
        string("`"),
        many(template_char())
    ).map(|s: String| Token::String(s))
}

fn template_char<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(c_char('$').skip(not_followed_by(c_char('{'))).map(|c: char| c.to_string())),
        try(escaped('`').map(|c: char| format!("\\{}", c))),
        try(escaped('\\').map(|c: char| c.to_string())),
        try(string_continuation()),
        try(satisfy(|c: char| c != '"' && c != '\n' && c != '\r').map(|c: char| c.to_string())),
    ))
}

fn replaced<I>() -> impl Parser<Input = I, Output = Vec<Token>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    many(choice((
        try(comment()),
        try(boolean_literal()),
        try(keyword()),
        try(ident()),
        try(null_literal()),
        try(numeric_literal()),
        try(regex_literal()),
        try(punctuation()),
        try(literal()),
    )).map(|t| t))
        .map(|t: Vec<Token>| t)
}



#[cfg(test)]
mod test {
    use super::super::{token, Token};
    use combine::Parser;
    #[test]
    fn strings() {
        let strings = vec![
            "junk and places",
            "things and stuff",
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
    }

    #[test]
    fn continuation() {
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
}