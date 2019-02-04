use combine::{
    attempt, choice, eof,
    error::ParseError,
    optional,
    parser::{char::string, repeat::take_until},
    range::recognize,
    Parser, RangeStream, Stream,
};
use refs::tokens::{Comment, RefToken as Token};
use strings::line_terminator_sequence;

pub fn comment<'a, I>() -> impl Parser<Input = I, Output = Token>
where
    I: RangeStream<Item = char, Range = &'a str>,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(multi_comment::<'a, I>()),
        attempt(single_comment::<'a, I>()),
        attempt(html_comment::<'a, I>()),
    ))
    .map(Token::Comment)
}

pub(crate) fn single_comment<'a, I>() -> impl Parser<Input = I, Output = Comment>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    choice((
        attempt(single_comment_new_line::<'a, I>()),
        attempt(single_comment_eof::<'a, I>()),
    ))
    .map(|_| Comment::SingleLine)
}

fn single_comment_eof<'a, I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    recognize((string("//"), take_until::<String, _>(eof())))
}

fn single_comment_new_line<'a, I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    recognize((
        string("//"),
        take_until::<String, _>(line_terminator_sequence()),
    ))
}

pub(crate) fn multi_comment<'a, I>() -> impl Parser<Input = I, Output = Comment>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    recognize((
        multi_line_comment_start(),
        take_until::<String, _>(multi_line_comment_end()),
        multi_line_comment_end(),
    ))
    .map(|_| Comment::MultiLine)
}

fn multi_line_comment_start<'a, I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    combine::range::range("/*".into()).map(|_| ())
}

fn multi_line_comment_end<'a, I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    combine::range::range("*/".into()).map(|_| ())
}

fn html_comment<'a, I>() -> impl Parser<Input = I, Output = Comment>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    recognize((
        combine::range::range("<!--".into()),
        take_until::<String, _>(combine::range::range("-->".into())),
        combine::range::range("-->".into()),
        optional(take_until::<String, _>(attempt(line_terminator_sequence()))),
    ))
    .map(|_| Comment::Html)
}

#[cfg(test)]
mod test {
    use super::*;
    static COMMENTS: &[&str] = &[
        "//this is a comment",
        "/*this is a
multi-line comment*/",
        "<!-- This is an HTML comment -->",
        "<!-- This is an HTML comment --> with a trailer",
    ];
    #[test]
    fn ref_comments() {
        for c in COMMENTS.iter() {
            let result = comment().parse(*c);
            assert!(result.is_ok());
        }
    }
}
