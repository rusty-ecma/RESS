use combine::{
    between, choice, error::ParseError, many, parser::char::char as c_char, satisfy, attempt, Parser,
    Stream,
    range::recognize,
};
use super::{
    super::{
        is_source_char,
        is_line_term,
        tokens::ident_part,
    },
    RefToken as Token,
};
/// Parse a regex literal starting after the first /
pub fn regex_tail<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    recognize((
        attempt(regex_body()),
        c_char('/'),
        attempt(regex_flags()),
    )).map(|_| Token::RegEx)
}

fn regex_flags<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    recognize(
        many::<Vec<String>, _>(ident_part())
    )
}
/// Parse the body portion of the regex literal
fn regex_body<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    recognize((
        regex_first_char(),
        many::<String, _>(regex_char())
    ))
}

fn regex_first_char<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    choice((
        attempt(regex_body_first_source_char()),
        attempt(regular_expression_backslash_sequence()),
        attempt(regular_expression_class()),
    ))
}

fn regex_body_first_source_char<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>
{
    recognize(
        satisfy(|c: char| {
            is_source_char(c)
            && !is_line_term(c)
            && c != '*'
            && c != '\\'
            && c != '/'
            && c != '['
        })
    )
}

fn regex_body_source_char<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>
{
    recognize(satisfy(|c: char| is_source_char(c) && !is_line_term(c) && c != '\\' && c != '/' && c != '['))
}

fn regex_char<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    choice((
        attempt(regex_body_source_char()),
        attempt(regular_expression_backslash_sequence()),
        attempt(regular_expression_class()),
    ))
}

fn regular_expression_class<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    recognize(between(
        c_char('['),
        c_char(']'),
        many::<String, _>(regular_expression_class_char()),
    ))
}

fn regular_expression_class_char<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{

    choice((
        recognize(attempt(satisfy(|c: char| {
            is_source_char(c) && !is_line_term(c) && c != '\u{005C}' && c != '\u{005D}'
        }))),
        attempt(regular_expression_backslash_sequence()),
    ))

}
pub(crate) fn source_char_not_line_term<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>
{
    recognize(satisfy(|c: char| is_source_char(c) && !is_line_term(c)).map(|c: char| c))
}

fn regular_expression_backslash_sequence<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>
{
    recognize(c_char('\\')
        .and(source_char_not_line_term())
    )
}
