use super::{
    NumberKind,
    CommentKind,
    Keyword,
    Punct,
    BooleanLiteral,
};
#[derive(Debug, PartialEq, Clone)]
/// The representation of a single JS token
/// with an owned string 
pub enum Token {
    /// `true` of `false`
    Boolean(BooleanLiteral),
    /// The end of the file
    EoF,
    /// An identifier this will be either a variable name
    /// or a function/method name
    Ident(Ident),
    /// A word that has been reserved to not be used as an identifier
    Keyword(Keyword),
    /// A `null` literal value
    Null,
    /// A number, this includes integers (`1`), decimals (`0.1`),
    /// hex (`0x8f`), binary (`0b010011010`), and octal (`0o273`)
    Numeric(Number),
    /// A punctuation mark, this includes all mathematical operators
    /// logical operators and general syntax punctuation
    Punct(Punct),
    /// A string literal, either double or single quoted, the associated
    /// value will be the unquoted string
    String(StringLit),
    /// A regular expression literal.
    /// ```js
    /// let regex = /[a-zA-Z]+/g;
    /// ```
    RegEx(RegEx),
    /// The string parts of a template string
    Template(Template),
    /// A comment, the associated value will contain the raw comment
    /// This will capture both inline comments `// I am an inline comment`
    /// and multi-line comments
    /// ```js
    /// /*multi lines
    /// * comments
    /// */
    /// ```
    Comment(Comment),
}

#[derive(Debug, PartialEq, Clone)]
/// An identifier
pub struct Ident(String);

impl PartialEq<&str> for &Ident {
    fn eq(&self, other: &&str) -> bool {
        self.0.eq(other)
    }
}

impl<'a> PartialEq<str> for &Ident {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

impl Ident {
    pub fn matches(&self, other: &str) -> bool {
        &self.0 == other
    }
}

impl<'a> From<&'a str> for Ident {
    fn from(s: &'a str) -> Self {
        Ident(s.into())
    }
}

impl From<String> for Ident {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl ToString for Ident {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Into<String> for Ident {
    fn into(self) -> String {
        self.0
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Comment {
    pub kind: CommentKind,
    pub content: String,
    pub tail_content: Option<String>,
}


impl Comment {
    pub fn from_parts(content: String, kind: CommentKind, tail_content: Option<String>) -> Self {
        Comment {
            content,
            kind,
            tail_content,
        }
    }

    pub fn new_single_line(content: &str) -> Self {
        Comment::from_parts(content.to_owned(), CommentKind::Single, None)
    }

    pub fn new_multi_line(content: &str) -> Self {
        Comment::from_parts(content.to_owned(), CommentKind::Multi, None)
    }

    pub fn new_html(content: &str, tail_content: Option<String>) -> Self {
        Comment::from_parts(content.to_owned(), CommentKind::Html, tail_content)
    }

    pub fn new_html_no_tail(content: &str) -> Self {
        Comment::new_html(content, None)
    }

    pub fn new_html_with_tail(content: &str, tail: &str) -> Self {
        Comment::new_html(content, Some(tail.to_owned()))
    }

    pub fn is_multi_line(&self) -> bool {
        self.kind == CommentKind::Multi
    }

    pub fn is_single_line(&self) -> bool {
        self.kind == CommentKind::Single
    }

    pub fn is_html(&self) -> bool {
        self.kind == CommentKind::Multi
    }
}

impl ToString for Comment {
    fn to_string(&self) -> String {
        match self.kind {
            CommentKind::Single => format!("//{}", self.content),
            CommentKind::Multi => format!("/*{}*/", self.content),
            CommentKind::Html => format!("<!--{}-->", self.content),
        }
    }
}



#[derive(Debug, PartialEq, Clone)]
pub struct Number(String);

impl Number {
    pub fn kind(&self) -> NumberKind {
        if self.0.starts_with("0x") {
            NumberKind::Hex
        } else if self.0.starts_with("0b") {
            NumberKind::Bin
        } else if self.0.starts_with("0o") {
            NumberKind::Oct
        } else {
            NumberKind::Dec
        }
    }

    pub fn is_hex(&self) -> bool {
        self.kind() == NumberKind::Hex
    }
    pub fn is_bin(&self) -> bool {
        self.kind() == NumberKind::Bin
    }
    pub fn is_oct(&self) -> bool {
        self.kind() == NumberKind::Oct
    }
    pub fn is_dec(&self) -> bool {
        self.kind() == NumberKind::Dec
    }
    pub fn has_exponent(&self) -> bool {
        match self.kind() {
            NumberKind::Dec => self.0.contains(|c| c == 'e' || c == 'E'),
            _ => false,
        }
    }
}

impl From<String> for Number {
    fn from(s: String) -> Self {
        Number(s)
    }
}

impl<'a> From<&'a str> for Number {
    fn from(s: &'a str) -> Self {
        Number(s.into())
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl PartialEq<str> for Number {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegEx {
    pub body: String,
    pub flags: Option<String>,
}

impl RegEx {
    pub fn from_parts(body: &str, flags: Option<String>) -> Self {
        let flags = if let Some(flags) = flags {
            if flags == "" {
                None
            } else {
                Some(flags.to_string())
            }
        } else {
            None
        };
        RegEx {
            body: body.to_string(),
            flags,
        }
    }
}

impl ToString for RegEx {
    fn to_string(&self) -> String {
        let f = if let Some(ref f) = self.flags {
            f.clone()
        } else {
            String::new()
        };
        format!("/{}/{}", self.body, f)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StringLit {
    Single(String),
    Double(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Template {
    NoSub(String),
    Head(String),
    Middle(String),
    Tail(String),
}

impl ToString for StringLit {
    fn to_string(&self) -> String {
        match self {
            StringLit::Single(ref s) => format!(r#"'{}'"#, s),
            StringLit::Double(ref s) => format!(r#""{}""#, s),
        }
    }
}

impl StringLit {
    pub fn single(content: &str) -> Self {
        StringLit::Single(content.into())
    }
    pub fn double(content: &str) -> Self {
        StringLit::Double(content.into())
    }
    pub fn is_single(&self) -> bool {
        match self {
            StringLit::Single(_) => true,
            _ => false,
        }
    }
    pub fn is_double(&self) -> bool {
        match self {
            StringLit::Double(_) => true,
            _ => false,
        }
    }
    pub fn no_quote(&self) -> String {
        match self {
            StringLit::Single(ref inner) => inner.clone(),
            StringLit::Double(ref inner) => inner.clone(),
        }
    }
}

impl Template {
    pub fn no_sub_template(content: &str) -> Self {
        Template::NoSub(content.into())
    }
    pub fn template_head(content: &str) -> Self {
        Template::Head(content.into())
    }
    pub fn template_middle(content: &str) -> Self {
        Template::Middle(content.into())
    }
    pub fn template_tail(content: &str) -> Self {
        Template::Tail(content.into())
    }
    pub fn is_head(&self) -> bool {
        match self {
            Template::Head(_) => true,
            _ => false,
        }
    }
    pub fn is_middle(&self) -> bool {
        match self {
            Template::Middle(_) => true,
            _ => false,
        }
    }
    pub fn is_tail(&self) -> bool {
        match self {
            Template::Tail(_) => true,
            _ => false,
        }
    }
    pub fn is_no_sub(&self) -> bool {
        match self {
            Template::NoSub(_) => true,
            _ => false,
        }
    }
}

impl ToString for Template {
    fn to_string(&self) -> String {
        match self {
            Template::NoSub(ref c) => format!("`{}`", c),
            Template::Head(ref c) => format!("`{}${{", c),
            Template::Middle(ref c) => format!("}}{}${{", c),
            Template::Tail(ref c) => format!("}}{}`", c),
        }
    }
}