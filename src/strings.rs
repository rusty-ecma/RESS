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

use super::{is_line_term, is_source_char};
use tokens::Token;
#[derive(Debug, PartialEq, Clone)]
pub enum StringLit {
    Single(String),
    Double(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Template {
    NoSub(String),
    Head(String),
    Middle(String),
    Tail(String),
}

impl ToString for StringLit {
    fn to_string(&self) -> String {
        match self {
            StringLit::Single(ref s) => format!(r#"'{}'"#, s),
            StringLit::Double(ref s) => format!(r#""{}""#, s),
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
    pub fn is_single(&self) -> bool {
        match self {
            StringLit::Single(_) => true,
            _ => false,
        }
    }
    pub fn is_double(&self) -> bool {
        match self {
            StringLit::Double(_) => true,
            _ => false,
        }
    }
    pub fn no_quote(&self) -> String {
        match self {
            StringLit::Single(ref inner) => inner.clone(),
            StringLit::Double(ref inner) => inner.clone(),
        }
    }
}

impl Template {
    pub fn no_sub_template(content: &str) -> Self {
        Template::NoSub(content.into())
    }
    pub fn template_head(content: &str) -> Self {
        Template::Head(content.into())
    }
    pub fn template_middle(content: &str) -> Self {
        Template::Middle(content.into())
    }
    pub fn template_tail(content: &str) -> Self {
        Template::Tail(content.into())
    }
    pub fn is_head(&self) -> bool {
        match self {
            Template::Head(_) => true,
            _ => false,
        }
    }
    pub fn is_middle(&self) -> bool {
        match self {
            Template::Middle(_) => true,
            _ => false,
        }
    }
    pub fn is_tail(&self) -> bool {
        match self {
            Template::Tail(_) => true,
            _ => false,
        }
    }
}

impl ToString for Template {
    fn to_string(&self) -> String {
        match self {
            Template::NoSub(ref c) => format!("`{}`", c),
            Template::Head(ref c) => format!("`{}${{", c),
            Template::Middle(ref c) => format!("}}{}${{", c),
            Template::Tail(ref c) => format!("}}{}`", c),
        }
    }
}

pub(crate) fn literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(single_quote()), try(double_quote()))).map(Token::String)
}

fn single_quote<I>() -> impl Parser<Input = I, Output = StringLit>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (between(c_char('\''), c_char('\''), many(single_quoted_content()))).map(StringLit::Single)
}

fn single_quoted_content<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(string(r#"\'"#).map(|s: &str| s.to_string())),
        try(string(r#"\\"#).map(|s: &str| s.to_string())),
        try(string_continuation()),
        try(satisfy(|c: char| c != '\'' && !is_line_term(c)).map(|c: char| c.to_string())),
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
    between(c_char('"'), c_char('"'), many(double_quoted_content())).map(StringLit::Double)
}

fn double_quoted_content<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(string(r#"\""#).map(|s: &str| s.to_string())),
        try(string(r#"\\"#).map(|s: &str| s.to_string())),
        try(string_continuation()),
        try(satisfy(|c: char| c != '"' && !is_line_term(c)).map(|c: char| c.to_string())),
    )).map(|c: String| c)
}

fn line_terminator<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(is_line_term).map(|c: char| c)
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

pub(crate) fn template_start<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(no_sub_template()), try(template_head()))).map(Token::Template)
}

pub(crate) fn template_continuation<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(template_middle()), try(template_tail()))).map(Token::Template)
}

fn no_sub_template<I>() -> impl Parser<Input = I, Output = Template>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(c_char('`'), c_char('`'), many(template_char())).map(Template::NoSub)
}

fn template_head<I>() -> impl Parser<Input = I, Output = Template>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(string("`"), string("${"), many(template_char())).map(Template::Head)
}

fn template_middle<I>() -> impl Parser<Input = I, Output = Template>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (many(template_char()), string("${")).map(|(s, _): (String, _)| Template::Middle(s))
}

fn template_tail<I>() -> impl Parser<Input = I, Output = Template>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (try(many(template_char())), c_char('`')).map(|(s, _): (String, _)| Template::Tail(s))
}

fn template_char<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(c_char('$')
            .skip(not_followed_by(c_char('{')))
            .map(|c: char| c.to_string())),
        try(string(r#"\${"#).map(|s: &str| s.to_string())),
        try(string(r#"\`"#).map(|s: &str| s.to_string())),
        try(string(r#"\"#).map(|s: &str| s.to_string())),
        try(satisfy(|c: char| is_source_char(c) && c != '`' && c != '$')
            .map(|c: char| c.to_string())),
    )).map(|s: String| s)
}

#[cfg(test)]
mod test {
    use combine::Parser;
    use tokens::{token, Token};
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

    proptest! {
        #[test]
        fn string_prop(s in r#"("((\\n)|(\\")|[^\n\r"\\])")|('((\\n)|(\\')|[^\n\r\\])')"#) {
            let r = token().easy_parse(s.as_str()).unwrap();
            assert!(r.0.is_string())
        }
    }
}
