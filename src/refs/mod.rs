pub mod keywords;
pub mod punct;
pub mod strings;
pub mod comments;
pub mod numbers;
pub mod regex;
pub mod tokens;

use tokens::Span;

pub use refs::tokens::RefToken;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RefItem {
    pub token: RefToken,
    pub span: Span,
}

#[allow(unused)]
pub struct RefScanner {
    pub stream: String,
    pub eof: bool,
    pub cursor: usize,
    pub spans: Vec<Span>,
    last_open_paren_idx: usize,
    template: usize,
    replacement: usize,
    pub pending_new_line: bool,
}

impl RefScanner {
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let cursor = text.len() - text.trim_left_matches(super::whitespace).len();
        Self {
            stream: text,
            eof: false,
            cursor,
            spans: vec![],
            last_open_paren_idx: 0,
            template: 0,
            replacement: 0,
            pending_new_line: false,
        }
    }
}
