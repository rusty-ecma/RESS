#[derive(Debug, PartialEq, Clone)]
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

impl<T> ToString for Comment<T>
where
    T: AsRef<str>,
{
    fn to_string(&self) -> String {
        match self.kind {
            CommentKind::Single => format!("//{}", self.content.as_ref()),
            CommentKind::Multi => format!("/*{}*/", self.content.as_ref()),
            CommentKind::Html => format!("<!--{}-->", self.content.as_ref()),
            CommentKind::Hashbang => format!("#!{}", self.content.as_ref()),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// The 4 kinds of comments
pub enum CommentKind {
    Single,
    Multi,
    Html,
    Hashbang,
}
