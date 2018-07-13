use combine::{
    between, choice, error::ParseError, many,
    parser::{
        char::{char as c_char, string, spaces}, 
        item::satisfy,
    },
    try, Parser, Stream, not_followed_by,
};

use tokens;
use tokens::{Token};
use regex;
use keywords;
use numeric;
use punct;
use comments;
use super::escaped;
#[derive(Debug, PartialEq, Clone)]
pub struct StringLit {
    pub quote: Quote,
    pub content: String,
}

impl StringLit {
    pub fn into_simple(&self) -> String {
        let q = match self.quote {
            Quote::Single => '\'',
            Quote::Double => '"',
            _ => panic!("Cannot convert template string to simple string")
        };
        format!("{0}{1}{0}",q, self.content)
    }
}

impl StringLit {
    pub fn from_parts(quote: Quote, content: &str) -> Self {
        StringLit {
            quote,
            content: content.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Quote {
    Single,
    Double,
    BackTick,
}

pub(crate) fn literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(single_quote()), try(double_quote()))).map(|t| Token::String(t))
}

fn single_quote<I>() -> impl Parser<Input = I, Output = StringLit>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        between(
            c_char('\''),
            c_char('\''),
            many(single_quoted_content())
        )
    ).map(|t: String| StringLit::from_parts(Quote::Single, t.as_str()))
}

fn single_quoted_content<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(escaped('\'').map(|c: char| c.to_string())),
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

fn double_quote<I>() -> impl Parser<Input = I, Output = StringLit>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(
        c_char('"'), 
        c_char('"'), 
        many(double_quoted_content())
    ).map(|t: String| StringLit::from_parts(Quote::Double, t.as_str()))
}

fn double_quoted_content<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(escaped('"').map(|c: char| c.to_string())),
        try(escaped('\\').map(|c: char| c.to_string())),
        try(string_continuation()),
        try(satisfy(|c: char| c != '"' && c != '\n' && c != '\r').map(|c: char| c.to_string())),
    )).map(|c: String| c)
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

pub(crate) fn line_terminator_sequence<I>() -> impl Parser<Input = I, Output = String>
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
        try(actual_template()),
        try(no_sub_template()),
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
    ).map(|s: String| Token::Template(vec![Token::String(StringLit::from_parts(Quote::BackTick, s.as_str()))]))
}

fn actual_template<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        template_head(),
        replaced(),
        many((
            try(template_middle()),
            replaced(),
        )),
        template_tail(),
    ).map(|(h, r1, m, t): (Token, Vec<Token>, Vec<(Token, Vec<Token>)>, Token)| {
        let mut ret = vec![h];
        ret.extend(r1);
        for mid in m {
            ret.push(mid.0);
            ret.extend(mid.1);
        }
        ret.push(t);
        Token::Template(ret)
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
        many(template_char()),
    ).map(|s: String| Token::String(StringLit::from_parts(Quote::BackTick, s.as_str())))
}

fn template_head<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(
        string("`"),
        string("${"),
        many(template_char()),
    )
    .map(|s: String| Token::String(StringLit::from_parts(Quote::BackTick, s.as_str())))
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
    ).map(|s: String| Token::String(StringLit::from_parts(Quote::BackTick, s.as_str())))
}

fn template_char<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(c_char('$').skip(not_followed_by(c_char('{')))),
        try(escaped('`')),
        try(escaped('\\')),
        try(satisfy(|c: char| c != '`' && c != '$')),
    ))
}

fn replaced_<I>() -> impl Parser<Input = I, Output = Vec<Token>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    many(replaced_content()).map(|ts: Vec<Token>| ts)
}

fn replaced_content<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    tokens::token_not_eof()
}

parser!{
    fn replaced[I]()(I) -> Vec<Token>
    where [I: Stream<Item = char>]
    {
        replaced_()
        
    }
}

#[cfg(test)]
mod test {
    use tokens::{token, Token};
    use super::*;
    use combine::Parser;
    #[test]
    fn strings() {
        let strings = vec![
            "junk and places",
            "things and stuff",
            "✨✨✨✨ ✨✨✨✨",
        ];
        for s in strings.into_iter() {
            let dq_test = format!("\"{}\"", &s.clone());
            let dq = token().parse(dq_test.as_str()).unwrap();
            let sq_test = format!("'{}'", &s.clone());
            let sq = token().parse(sq_test.as_str()).unwrap();
            assert_eq!(dq, (Token::String(super::StringLit::from_parts(Quote::Double, s)), ""));
            assert_eq!(sq, (Token::String(super::StringLit::from_parts(Quote::Single, s)), ""));
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
        assert_eq!(d_r, (Token::String(super::StringLit::from_parts(Quote::Double, target.clone())), ""));
        assert_eq!(s_r, (Token::String(super::StringLit::from_parts(Quote::Single, target.clone())), ""));
    }
    #[test]
    fn template_no_sub() {
        let empty = "`things and stuff`";
        let e_r = tokens::token().easy_parse(empty).unwrap();
        assert_eq!(e_r, (Token::Template(vec![Token::String(super::StringLit::from_parts(Quote::BackTick, "things and stuff"))]), ""));
    }
    #[test]
    fn template_one_sub() {
        let one_sub = "`things and stuff times ${x}`";
        match tokens::token().easy_parse(one_sub) {
            Ok(o_r) => assert_eq!(o_r, (Token::Template(vec![Token::String(super::StringLit::from_parts(Quote::BackTick, "things and stuff times ")),
                                                Token::Ident("x".into()),
                                                Token::String(super::StringLit::from_parts(Quote::BackTick, ""))]), "")),
            Err(e) => panic!("Error in template one sub\nposition: {:?}\nerrors: {:?}", e.position.translate_position(one_sub.clone()), e.errors),
        }
    }
    #[test]
    fn template_two_subs() {
        let two_subs = "`things and stuff times ${x} divided by ${y}`";
        let t_r = tokens::token().parse(two_subs).unwrap();
        assert_eq!(t_r, (Token::Template(vec![
            Token::String(super::StringLit::from_parts(Quote::BackTick, "things and stuff times ")),
            Token::Ident("x".into()),
            Token::String(super::StringLit::from_parts(Quote::BackTick, " divided by ")),
            Token::Ident("y".into()),
            Token::String(super::StringLit::from_parts(Quote::BackTick, ""))
        ]), ""))
    }
    #[test]
    fn multi_template() {
        let plain = "`things and
        stuff`";
        let p_r = token().parse(plain).unwrap();
        assert_eq!(p_r, (Token::Template(vec![
            Token::String(super::StringLit::from_parts(Quote::BackTick, &plain[1..plain.len() - 1]))
        ]), ""));
        let subbed = "`things and
        stuff times ${x}`";
        let s_r = token().parse(subbed).unwrap();
        assert_eq!(s_r, 
            (
                Token::Template(vec![
                    Token::String(super::StringLit::from_parts(Quote::BackTick, &subbed[1..subbed.len() - 5])),
                    Token::Ident("x".into()),
                    Token::String(super::StringLit::from_parts(Quote::BackTick, "")),
                    ]
                ), ""
            )
        )

    }
}