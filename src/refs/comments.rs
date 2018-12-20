use combine::{
    attempt, choice, eof,
    error::ParseError,
    optional,
    parser::{char::string, repeat::take_until},
    range::recognize,
    Parser, Stream,
};
use refs::tokens::{Comment, RefToken as Token};
use strings::line_terminator_sequence;

pub fn comment<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>:
        std::convert::From<<I as combine::StreamOnce>::Range>,
{
    choice((
        attempt(multi_comment()),
        attempt(single_comment()),
        attempt(html_comment()),
    ))
    .map(Token::Comment)
}

pub(crate) fn single_comment<I>() -> impl Parser<Input = I, Output = Comment>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>:
        std::convert::From<<I as combine::StreamOnce>::Range>,
{
    choice((
        attempt(single_comment_eof()),
        attempt(single_comment_new_line()),
    ))
    .map(|_| Comment::SingleLine)
}

fn single_comment_eof<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>:
        std::convert::From<<I as combine::StreamOnce>::Range>,
{
    recognize((string("//"), take_until::<String, _>(eof())))
}

fn single_comment_new_line<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>:
        std::convert::From<<I as combine::StreamOnce>::Range>,
{
    recognize((
        string("//"),
        take_until::<String, _>(line_terminator_sequence()),
    ))
}

pub(crate) fn multi_comment<I>() -> impl Parser<Input = I, Output = Comment>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>:
        std::convert::From<<I as combine::StreamOnce>::Range>,
{
    recognize((
        multi_line_comment_start(),
        take_until::<String, _>(string("*/")),
    ))
    .map(|_| Comment::MultiLine)
}

fn multi_line_comment_start<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>:
        std::convert::From<<I as combine::StreamOnce>::Range>,
{
    recognize(string("/*"))
}

fn html_comment<I>() -> impl Parser<Input = I, Output = Comment>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>:
        std::convert::From<<I as combine::StreamOnce>::Range>,
{
    recognize((
        string("<!--"),
        take_until::<String, _>(string("-->")),
        string("-->"),
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
    fn comments_ref() {
        for c in COMMENTS.iter() {
            let result = comment().parse(*c);
            assert!(result.is_ok());
        }
    }
}
