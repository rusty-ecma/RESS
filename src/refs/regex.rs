use super::{
    super::{is_line_term, is_source_char, tokens::ident_part},
    RefToken as Token,
};
use combine::{
    attempt, between, choice, error::ParseError, many, parser::char::char as c_char,
    range::recognize, satisfy, Parser, Stream,
};
/// Parse a regex literal starting after the first /
pub fn regex_tail<'a, I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    recognize((attempt(regex_body::<'a, I>()), c_char('/'), attempt(regex_flags::<'a, I>()))).map(|_| Token::RegEx)
}

fn regex_flags<'a, I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    recognize(many::<Vec<String>, _>(ident_part())).map(|_| ())
}
/// Parse the body portion of the regex literal
fn regex_body<'a, I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    recognize((
        regex_first_char(),
        many::<Vec<()>, _>(regex_char())
    )).map(|_| ())
}

fn regex_first_char<'a, I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    choice((
        attempt(regex_body_first_source_char()),
        attempt(regular_expression_backslash_sequence()),
        attempt(regular_expression_class()),
    )).map(|_|())
}

fn regex_body_first_source_char<'a, I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    recognize(satisfy(|c: char| {
        is_source_char(c) && !is_line_term(c) && c != '*' && c != '\\' && c != '/' && c != '['
    })).map(|_| ())
}

fn regex_body_source_char<'a, I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    recognize(satisfy(|c: char| {
        is_source_char(c) && !is_line_term(c) && c != '\\' && c != '/' && c != '['
    })).map(|_|())
}

fn regex_char<'a, I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    choice((
        attempt(regex_body_source_char()),
        attempt(regular_expression_backslash_sequence()),
        attempt(regular_expression_class()),
    )).map(|_|())
}

fn regular_expression_class<'a, I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    recognize(between(
        c_char('['),
        c_char(']'),
        many::<Vec<()>, _>(regular_expression_class_char()),
    )).map(|_|())
}

fn regular_expression_class_char<'a, I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    choice((
        attempt(recognize(satisfy(|c: char| {
            is_source_char(c) && !is_line_term(c) && c != '\u{005C}' && c != '\u{005D}'
        })).map(|_|())),
        attempt(regular_expression_backslash_sequence()),
    )).map(|_| ())
}
pub(crate) fn source_char_not_line_term<'a, I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    recognize(satisfy(|c: char| is_source_char(c) && !is_line_term(c))).map(|_| ())
}

fn regular_expression_backslash_sequence<'a, I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    recognize(c_char('\\').and(source_char_not_line_term())).map(|_|())
}
