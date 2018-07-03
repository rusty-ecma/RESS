use combine::{
    between, choice, error::ParseError, many,
    parser::{
        char::{char as c_char, string, spaces}, 
        item::satisfy,
        repeat::take_until,
    },
    try, Parser, Stream, not_followed_by,
};

use tokens::*;
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
    ).map(|s: String| Token::Template(vec![Token::String(s)]))
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
    c_char('}').and(
        take_until(try(string("${")))
    ).map(|(_, s): (_, String)| Token::String(s))
}

fn template_head<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    c_char('`')
        .and(
            take_until(
                try(
                    string("${")
                )
            )
        )
        .map(|(_, s): (char, String)| Token::String(s))
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
        try(escaped('\\').map(|c: char| format!("\\{}", c))),
        try(satisfy(|c: char| c != '`' && c != '$').map(|c: char| c.to_string())),
    ))
}

fn replaced<I>() -> impl Parser<Input = I, Output = Vec<Token>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("${").and(take_until(choice((
        try(template_middle()),
        try(template_tail()),
    ))
    )).map(|(_,s): (_, String)| {
        let s = super::Scanner::new(s);
        s.filter(|t| t != &Token::EoF).collect()
    })
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

    #[test]
    fn template() {
        let empty = "`things and stuff`";
        let e_r = token().parse(empty).unwrap();
        println!("empty: {:?}", e_r);
        assert_eq!(e_r, (Token::Template(vec![Token::String("things and stuff".into())]), ""));
        let th = "`things and stuff ${";
        let th_r = super::template_head().parse(th).unwrap();
        println!("template head: {:?}", th_r);
        let tt = "}`";
        let tt_r = super::template_tail().parse(tt).unwrap();
        println!("template tail: {:?}", tt_r);
        // let r = super::replaced().parse("x}`").unwrap();
        // println!("replaced: {:?}", r);
        let one_sub = "`things and stuff times ${x}`";
        let manual = (
            super::template_head(),
            super::replaced(),
            super::template_tail(),
        ).map(|x: (Token, Vec<Token>, Token)| x).parse(one_sub).unwrap();
        println!("manual: {:?}", manual);
        let o_r = super::token().parse(one_sub).unwrap();
        println!("one: {:?}", o_r);
        assert_eq!(o_r, (Token::Template(vec![Token::String("things and stuff times ".into()),
                                                Token::Ident("x".into()),
                                                Token::String("".into())]), ""));
        let two_subs = "`things and stuff times ${x} divided by ${y}`";
        let t_r = super::token().parse(two_subs).unwrap();
        assert_eq!(t_r, (Token::Template(vec![
            Token::String("things and stuff times ".into()),
            Token::Ident("x".into()),
            Token::String(" divided by ".into()),
            Token::Ident("y".into()),
            Token::String("".into())
        ]), ""))
    }

    #[test]
    fn multi_template() {
        let plain = "`things and
        stuff`";
        let p_r = token().parse(plain).unwrap();
        assert_eq!(p_r, (Token::Template(vec![
            Token::String(plain[1..plain.len() - 1].into())
        ]), ""));
        let subbed = "`things and
        stuff times ${x}`";
        let s_r = token().parse(subbed).unwrap();
        assert_eq!(s_r, 
            (
                Token::Template(vec![
                    Token::String(subbed[1..subbed.len() - 5].to_string()),
                    Token::Ident("x".into()),
                    Token::String("".into())
                    ]
                ), ""
            )
        )

    }
}