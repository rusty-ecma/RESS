use combine::{
    choice, eof, error::ParseError,
    parser::{
        char::{string},
        repeat::take_until
    },
    try, Parser, Stream,
};
use tokens::TokenData;
use strings;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: Kind,
    pub content: String,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
    Single,
    Multi
}

impl Token {
    pub fn from_parts(content: String, kind: Kind) -> Self {
        Token {
            content,
            kind,
        }
    }
}

pub(crate) fn comment<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (choice((try(multi_comment()), try(single_comment()))).map(|t: Token| TokenData::Comment(t)))
}

pub(crate) fn single_comment<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (string("//"), take_until(
        choice((
            try(strings::line_terminator_sequence()),
            try(eof().map(|_| String::new())),
            ))))
        .map(|(_, content): (_, String)| Token::from_parts(content, Kind::Single))
}

pub(crate) fn multi_comment<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        multi_line_comment_start(),
        take_until(try(string("*/"))),
        multi_line_comment_end(),
    ).map(|(_s, c, _e): (String, String, String)| {
        let ret = c.lines()
            .map(|l| l.trim())
            .collect::<Vec<&str>>()
            .join("\n");
        Token::from_parts(ret, Kind::Multi)
    })
}

fn multi_line_comment_start<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (string("/*")).map(|s| s.to_string())
}

fn multi_line_comment_end<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (string("*/")).map(|s| s.to_string())
}

#[cfg(test)]
mod test {
    use tokens::token;
    use super::*;
    #[test]
    fn comments_test() {
        let tests = vec![
            "//single line comments",
            "// another one with a space",
            "/*inline multi comments*/",
            "/*multi line comments
            * that have extra decoration
            * to help with readability
            */",
        ];
        for test in tests {
            let is_multi = test.starts_with("/*");
            let p = token().parse(test.clone()).unwrap();
            let comment_contents = test.lines()
                .map(|l| {
                    l.trim()
                        .replace("//", "")
                        .replace("/*", "")
                        .replace("*/", "")
                })
                .collect::<Vec<String>>()
                .join("\n");
            assert_eq!(p, (TokenData::comment(comment_contents, is_multi), ""));
        }
    }
}