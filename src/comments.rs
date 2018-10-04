use combine::{
    choice, eof,
    error::ParseError,
    optional,
    parser::{char::string, repeat::take_until},
    try, Parser, Stream,
};
use strings;
use tokens::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Comment {
    pub kind: Kind,
    pub content: String,
    pub tail_content: Option<String>,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Kind {
    Single,
    Multi,
    Html,
}

impl Comment {
    pub fn from_parts(content: String, kind: Kind, tail_content: Option<String>) -> Self {
        Comment {
            content,
            kind,
            tail_content,
        }
    }

    pub fn new_single_line(content: &str) -> Self {
        Comment::from_parts(content.to_owned(), Kind::Single, None)
    }

    pub fn new_multi_line(content: &str) -> Self {
        Comment::from_parts(content.to_owned(), Kind::Multi, None)
    }

    pub fn new_html(content: &str, tail_content: Option<String>) -> Self {
        Comment::from_parts(content.to_owned(), Kind::Html, tail_content)
    }

    pub fn new_html_no_tail(content: &str) -> Self {
        Comment::new_html(content, None)
    }

    pub fn new_html_with_tail(content: &str, tail: &str) -> Self {
        Comment::new_html(content, Some(tail.to_owned()))
    }

    pub fn is_multi_line(&self) -> bool {
        self.kind == Kind::Multi
    }

    pub fn is_single_line(&self) -> bool {
        self.kind == Kind::Single
    }

    pub fn is_html(&self) -> bool {
        self.kind == Kind::Multi
    }
}

impl ToString for Comment {
    fn to_string(&self) -> String {
        match self.kind {
            Kind::Single => format!("//{}", self.content),
            Kind::Multi => format!("/*{}*/", self.content),
            Kind::Html => format!("<!--{}-->", self.content),
        }
    }
}

pub(crate) fn comment<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(multi_comment()),
        try(single_comment()),
        try(html_comment()),
    )).map(Token::Comment)
}

pub(crate) fn single_comment<I>() -> impl Parser<Input = I, Output = Comment>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        string("//"),
        take_until(choice((
            try(strings::line_terminator_sequence()),
            try(eof().map(|_| String::new())),
        ))),
    )
        .map(|(_, content): (_, String)| Comment::new_single_line(&content))
}

pub(crate) fn multi_comment<I>() -> impl Parser<Input = I, Output = Comment>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        multi_line_comment_start(),
        take_until(try(string("*/"))),
        multi_line_comment_end(),
    )
        .map(|(_s, c, _e): (String, String, String)| Comment::new_multi_line(&c))
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

fn html_comment<I>() -> impl Parser<Input = I, Output = Comment>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        string("<!--"),
        take_until(try(string("-->"))),
        string("-->"),
        optional(take_until(try(strings::line_terminator_sequence()))),
    )
        .map(|(_, content, _, tail): (_, String, _, Option<String>)| {
            Comment::new_html(&content, tail)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use tokens::token;
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
            let comment_contents = test
                .lines()
                .map(|l| {
                    l.trim()
                        .replace("//", "")
                        .replace("/*", "")
                        .replace("*/", "")
                }).collect::<Vec<String>>()
                .join("\n");
            assert_eq!(p, (Token::comment(&comment_contents, is_multi), ""));
        }
    }

    fn format_test_comment(s: &str, kind: Kind) -> String {
        let (left_matches, right_matches) = match kind {
            Kind::Single => ("//", ""),
            Kind::Multi => ("/*", "*/"),
            Kind::Html => ("<!--", "-->"),
        };
        s.lines()
            .map(|l| {
                l.trim()
                    .trim_left_matches(left_matches)
                    .trim_right_matches(right_matches)
            }).collect::<Vec<&str>>()
            .join("\n")
    }
    proptest!{
        #[test]
        fn multi_line_comments_prop(s in r#"(/\*(.+[\n\r*])+\*/)"#) {
            let r = token().easy_parse(s.as_str()).unwrap();
            assert!(r.0.is_comment(), r.0.matches_comment_str(&format_test_comment(&s, Kind::Multi)));
        }
    }

    proptest!{
        #[test]
        fn single_line_comments_prop(s in r#"(//.*)+"#) {
            let r = token().easy_parse(s.as_str()).unwrap();
            assert!(r.0.is_comment(), r.0.matches_comment_str(&format_test_comment(&s, Kind::Single)));
        }
    }

    proptest!{
        #[test]
        fn html_comments_prop(s in r#"<!--.*-->"#) {
            eprintln!("testing {:?}", s);
            let r = token().easy_parse(s.as_str()).unwrap();
            assert!(r.0.is_comment(), r.0.matches_comment_str(&format_test_comment(&s, Kind::Html)));
        }
    }

}
