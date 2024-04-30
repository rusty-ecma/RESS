use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
/// A comment, effectively should be treated
/// as white space. There are 3 kinds of comments
/// according to the specification.
///
/// - Single line comments: //comment
/// - Multi line comments: /* comment */
/// - HTML comments: <!-- comment --> plus more!
pub struct Comment<T> {
    pub kind: CommentKind,
    pub content: T,
    pub tail_content: Option<T>,
}

impl<T> Comment<T> {
    pub fn from_parts(content: T, kind: CommentKind, tail_content: Option<T>) -> Self {
        Comment {
            content,
            kind,
            tail_content,
        }
    }
    pub fn new_single_line(content: T) -> Self {
        Comment::from_parts(content, CommentKind::Single, None)
    }

    pub fn new_multi_line(content: T) -> Self {
        Comment::from_parts(content, CommentKind::Multi, None)
    }

    pub fn new_html(content: T, tail_content: Option<T>) -> Self {
        Comment::from_parts(content, CommentKind::Html, tail_content)
    }

    pub fn new_html_no_tail(content: T) -> Self {
        Comment::new_html(content, None)
    }

    pub fn new_html_with_tail(content: T, tail: T) -> Self {
        Comment::new_html(content, Some(tail))
    }

    pub fn new_hashbang(content: T) -> Self {
        Comment::from_parts(content, CommentKind::Hashbang, None)
    }
    pub fn is_multi_line(&self) -> bool {
        self.kind == CommentKind::Multi
    }

    pub fn is_single_line(&self) -> bool {
        self.kind == CommentKind::Single
    }

    pub fn is_html(&self) -> bool {
        self.kind == CommentKind::Html
    }

    pub fn is_hashbang(&self) -> bool {
        self.kind == CommentKind::Hashbang
    }
}

impl<T> Display for Comment<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            CommentKind::Single => write!(f, "//{}", self.content),
            CommentKind::Multi => write!(f, "/*{}*/", self.content),
            CommentKind::Html => write!(f, "<!--{}-->", self.content),
            CommentKind::Hashbang => write!(f, "#!{}", self.content),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// The 4 kinds of comments
pub enum CommentKind {
    Single,
    Multi,
    Html,
    Hashbang,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(
            "// line comment",
            format!(
                "{}",
                Comment::from_parts(" line comment", CommentKind::Single, None)
            )
        );
        assert_eq!(
            "/* multi-line comment */",
            format!(
                "{}",
                Comment::from_parts(" multi-line comment ", CommentKind::Multi, None)
            )
        );
        assert_eq!(
            "<!-- HTML comment -->",
            format!(
                "{}",
                Comment::from_parts(" HTML comment ", CommentKind::Html, None)
            )
        );
        assert_eq!(
            "#! hash-bang comment",
            format!(
                "{}",
                Comment::from_parts(" hash-bang comment", CommentKind::Hashbang, None)
            )
        );
    }

    #[test]
    fn ctors_and_helpers() {
        let sl = Comment::new_single_line("single line");
        assert_helpers(
            "single line",
            &sl,
            Comment::is_single_line,
            &[
                &Comment::is_hashbang,
                &Comment::is_html,
                &Comment::is_multi_line,
            ],
        );
        let sl = Comment::new_multi_line("single line");
        assert_helpers(
            "multi-line",
            &sl,
            Comment::is_multi_line,
            &[
                &Comment::is_hashbang,
                &Comment::is_html,
                &Comment::is_single_line,
            ],
        );
        let sl = Comment::new_hashbang("hash-bang");
        assert_helpers(
            "hash-bang",
            &sl,
            Comment::is_hashbang,
            &[
                &Comment::is_multi_line,
                &Comment::is_html,
                &Comment::is_single_line,
            ],
        );
        let sl = Comment::new_html_no_tail("html");
        assert_helpers(
            "html",
            &sl,
            Comment::is_html,
            &[
                &Comment::is_multi_line,
                &Comment::is_hashbang,
                &Comment::is_single_line,
            ],
        );
        let sl = Comment::new_html_with_tail("html", "tail");
        assert_helpers(
            "html+tail",
            &sl,
            Comment::is_html,
            &[
                &Comment::is_multi_line,
                &Comment::is_hashbang,
                &Comment::is_single_line,
            ],
        );
    }

    fn assert_helpers<'a>(
        name: &'static str,
        comment: &'a Comment<&'a str>,
        yes: impl Fn(&'a Comment<&'a str>) -> bool,
        nos: &'a [&dyn Fn(&Comment<&'a str>) -> bool],
    ) {
        assert!(yes(comment), "`{}` ({}) failed for yes", comment, name);
        for (i, f) in nos.into_iter().enumerate() {
            assert!(!f(comment), "`{}` ({}) failed for no {}", comment, name, i)
        }
    }
}
