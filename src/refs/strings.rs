use combine::{
    attempt, between, choice,
    error::ParseError,
    many, not_followed_by,
    parser::{
        char::{char as c_char, spaces, string},
        item::satisfy,
        range::recognize,
    },
    Parser, Stream,
};

use super::super::{is_line_term, is_source_char};
use refs::tokens::{RefToken as Token, StringLit, Template};

pub fn literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>:
        std::convert::From<<I as combine::StreamOnce>::Range>,
{
    choice((attempt(single_quote()), attempt(double_quote()))).map(Token::String)
}

fn single_quote<I>() -> impl Parser<Input = I, Output = StringLit>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    recognize(between(
        c_char('\''),
        c_char('\''),
        many::<String, _>(single_quoted_content()),
    ))
    .map(|_| StringLit::Single)
}

fn single_quoted_content<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(escaped_single_quote()),
        attempt(escaped_escape()),
        attempt(string_continuation()),
        attempt(recognize(satisfy(|c: char| c != '\'' && !is_line_term(c)))),
    ))
}

fn escaped_single_quote<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string(r#"\'"#))
}

fn escaped_escape<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string(r#"\\"#))
}

fn string_continuation<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize((c_char('\\'), line_terminator_sequence())).skip(spaces())
}

fn double_quote<I>() -> impl Parser<Input = I, Output = StringLit>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    recognize(between(
        c_char('"'),
        c_char('"'),
        many::<String, _>(double_quoted_content()),
    ))
    .map(|_| StringLit::Double)
}

fn double_quoted_content<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(escaped_double_quote()),
        attempt(escaped_escape()),
        attempt(string_continuation()),
        attempt(recognize(satisfy(|c: char| c != '"' && !is_line_term(c)))),
    ))
}

fn escaped_double_quote<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string(r#"\""#))
}

fn line_terminator<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(satisfy(is_line_term))
}

pub(crate) fn line_terminator_sequence<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(choice((
        attempt(recognize(string("\r\n"))),
        attempt(line_terminator()),
    )))
}

pub fn template_start<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    choice((attempt(no_sub_template()), attempt(template_head()))).map(Token::Template)
}

pub fn template_continuation<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    choice((attempt(template_middle()), attempt(template_tail()))).map(Token::Template)
}

fn no_sub_template<I>() -> impl Parser<Input = I, Output = Template>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    recognize(between(
        c_char('`'),
        c_char('`'),
        many::<String, _>(template_char()),
    ))
    .map(|_| Template::NoSub)
}

fn template_head<I>() -> impl Parser<Input = I, Output = Template>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    recognize(between(
        string("`"),
        string("${"),
        many::<String, _>(template_char()),
    ))
    .map(|_| Template::Head)
}

fn template_middle<I>() -> impl Parser<Input = I, Output = Template>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    recognize((many::<String, _>(template_char()), string("${"))).map(|_| Template::Body)
}

fn template_tail<I>() -> impl Parser<Input = I, Output = Template>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    (many::<String, _>(template_char()), recognize(c_char('`'))).map(|_| Template::Tail)
}

fn template_char<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    choice((
        attempt(single_dollar_sign()),
        attempt(escaped_template_start()),
        attempt(escaped_back_tick()),
        attempt(solo_back_slash()),
        attempt(template_char_catch_all()),
    ))
}

fn single_dollar_sign<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    recognize(c_char('$').skip(not_followed_by(c_char('{'))))
}

fn escaped_template_start<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    recognize(string(r#"\${"#))
}

fn escaped_back_tick<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    recognize(string(r#"\`"#))
}

fn solo_back_slash<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    recognize(string(r#"\"#))
}

fn template_char_catch_all<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    recognize(satisfy(|c: char| is_source_char(c) && c != '`' && c != '$'))
}
