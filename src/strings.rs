use combine::{
    between, choice,
    error::ParseError,
    many, not_followed_by,
    parser::{
        char::{char as c_char, spaces, string},
        item::satisfy,
    },
    try, Parser, Stream,
};

use super::{escaped, is_line_term};
use tokens::Token;
#[derive(Debug, PartialEq, Clone)]
pub enum StringLit {
    Single(String),
    Double(String),
    NoSubTemplate(String),
    TemplateHead(String),
    TemplateMiddle(String),
    TemplateTail(String),
}

impl ToString for StringLit {
    fn to_string(&self) -> String {
        match self {
            &StringLit::Single(ref s) => format!("'{}'", s),
            &StringLit::Double(ref s) => format!(r#""{}""#, s),
            &StringLit::NoSubTemplate(ref s) => format!("`{}`", s),
            &StringLit::TemplateHead(ref s) => format!("`{}${{", s),
            &StringLit::TemplateMiddle(ref s) => format!("}}{}${{", s),
            &StringLit::TemplateTail(ref s) => format!("}}{}`", s),
        }
    }
}

impl StringLit {
    pub fn single(content: &str) -> Self {
        StringLit::Single(content.into())
    }
    pub fn double(content: &str) -> Self {
        StringLit::Double(content.into())
    }
    pub fn no_sub_template(content: &str) -> Self {
        StringLit::NoSubTemplate(content.into())
    }
    pub fn template_head(content: &str) -> Self {
        StringLit::TemplateHead(content.into())
    }
    pub fn template_middle(content: &str) -> Self {
        StringLit::TemplateMiddle(content.into())
    }
    pub fn template_tail(content: &str) -> Self {
        StringLit::TemplateTail(content.into())
    }
    pub fn is_single(&self) -> bool {
        match self {
            &StringLit::Single(_) => true,
            _ => false,
        }
    }
    pub fn is_double(&self) -> bool {
        match self {
            &StringLit::Double(_) => true,
            _ => false,
        }
    }
    pub fn is_template_head(&self) -> bool {
        match self {
            &StringLit::TemplateHead(_) => true,
            _ => false,
        }
    }
    pub fn is_template_middle(&self) -> bool {
        match self {
            &StringLit::TemplateMiddle(_) => true,
            _ => false,
        }
    }
    pub fn is_template_tail(&self) -> bool {
        match self {
            &StringLit::TemplateTail(_) => true,
            _ => false,
        }
    }
}

pub(crate) fn literal<I>() -> impl Parser<Input = I, Output = Token>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    choice((try(single_quote()), try(double_quote()))).map(|t| Token::String(t))
}

fn single_quote<I>() -> impl Parser<Input = I, Output = StringLit>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    (between(c_char('\''), c_char('\''), many(single_quoted_content())))
        .map(|t: String| StringLit::Single(t))
}

fn single_quoted_content<I>() -> impl Parser<Input = I, Output = String>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    choice((
        try(escaped('\'').map(|c: char| c.to_string())),
        try(escaped('\\').map(|c: char| c.to_string())),
        try(string_continuation()),
        try(satisfy(|c: char| c != '\'' && !is_line_term(c)).map(|c: char| c.to_string())),
    )).map(|s: String| s)
}

fn string_continuation<I>() -> impl Parser<Input = I, Output = String>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    (c_char('\\'), line_terminator_sequence()).skip(spaces()).map(|_| String::new())
}

fn double_quote<I>() -> impl Parser<Input = I, Output = StringLit>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    between(c_char('"'), c_char('"'), many(double_quoted_content())).map(|t: String| {
                                                                             StringLit::Double(t)
                                                                         })
}

fn double_quoted_content<I>() -> impl Parser<Input = I, Output = String>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    choice((
        try(escaped('"').map(|c: char| c.to_string())),
        try(escaped('\\').map(|c: char| c.to_string())),
        try(string_continuation()),
        try(satisfy(|c: char| c != '"' && !is_line_term(c)).map(|c: char| c.to_string())),
    )).map(|c: String| c)
}

fn line_terminator<I>() -> impl Parser<Input = I, Output = char>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    satisfy(|c| is_line_term(c)).map(|c: char| c)
}

pub(crate) fn line_terminator_sequence<I>() -> impl Parser<Input = I, Output = String>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    choice((
        try(string("\r\n").map(|s: &str| s.to_string())),
        try(line_terminator().map(|c: char| c.to_string())),
    )).map(|s: String| s)
}

pub(crate) fn template_start<I>() -> impl Parser<Input = I, Output = Token>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    choice((try(no_sub_template()), try(template_head()))).map(|s: StringLit| Token::String(s))
}

pub(crate) fn template_continuation<I>() -> impl Parser<Input = I, Output = Token>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    choice((try(template_middle()), try(template_tail()))).map(|s: StringLit| Token::String(s))
}

fn no_sub_template<I>() -> impl Parser<Input = I, Output = StringLit>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    between(c_char('`'), c_char('`'), many(template_char())).map(|s: String| {
                                                                     StringLit::NoSubTemplate(s)
                                                                 })
}

fn template_head<I>() -> impl Parser<Input = I, Output = StringLit>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    between(string("`"), string("${"), many(template_char())).map(|s: String| {
                                                                      StringLit::TemplateHead(s)
                                                                  })
}

fn template_middle<I>() -> impl Parser<Input = I, Output = StringLit>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    (many(template_char()), string("${")).map(|(s, _): (String, _)| StringLit::TemplateMiddle(s))
}

fn template_tail<I>() -> impl Parser<Input = I, Output = StringLit>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    (try(many(template_char())), c_char('`')).map(|(s, _): (String, _)| StringLit::TemplateTail(s))
}

fn template_char<I>() -> impl Parser<Input = I, Output = char>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    choice((try(c_char('$').skip(not_followed_by(c_char('{')))),
           try(escaped('`')),
           try(escaped('\\')),
           try(satisfy(|c: char| c != '`' && c != '$'))))
}

#[cfg(test)]
mod test {
    use combine::Parser;
    use tokens::{token, Token};
    #[test]
    fn strings() {
        let strings = vec!["junk and places", "things and stuff", "✨✨✨✨ ✨✨✨✨",];
        for s in strings.into_iter() {
            let dq_test = format!("\"{}\"", &s.clone());
            let dq = token().parse(dq_test.as_str()).unwrap();
            let sq_test = format!("'{}'", &s.clone());
            let sq = token().parse(sq_test.as_str()).unwrap();
            assert_eq!(dq, (Token::double_quoted_string(s), ""));
            assert_eq!(sq, (Token::single_quoted_string(s), ""));
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
        assert_eq!(d_r, (Token::double_quoted_string(target.clone()), ""));
        assert_eq!(s_r, (Token::single_quoted_string(target.clone()), ""));
    }

    #[test]
    fn template_no_sub() {
        let empty = "`things and stuff`";
        let e_r = token().easy_parse(empty).unwrap();
        assert_eq!(e_r, (Token::no_sub_template("things and stuff"), ""))
    }

    #[test]
    fn template_head() {
        let h = "`things and stuff times ${";
        let r = token().easy_parse(h).unwrap();
        assert_eq!(r, (Token::template_head("things and stuff times "), ""));
    }

    #[test]
    fn template_middle() {
        let m = " and places and people ${";
        let r = super::template_continuation().easy_parse(m).unwrap();
        assert_eq!(r, (Token::template_middle(" and places and people "), ""));
    }

    #[test]
    fn template_tail() {
        let t = " and animals and minerals`";
        let r = super::template_continuation().easy_parse(t).unwrap();
        assert_eq!(r, (Token::template_tail(" and animals and minerals"), ""));
    }
    #[test]
    fn double_tail() {
        let e = "`}`";
        let r = super::template_continuation().easy_parse(e).unwrap();
        assert_eq!(r, (Token::template_tail(""), "}`"));
    }
}
