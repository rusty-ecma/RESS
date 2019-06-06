use super::{BooleanLiteral, CommentKind, Keyword, NumberKind, Punct};
#[derive(Debug, PartialEq, Clone)]
/// The representation of a single JS token
/// with a str
pub enum Token<'a> {
    /// `true` of `false`
    Boolean(BooleanLiteral),
    /// The end of the file
    EoF,
    /// An identifier this will be either a variable name
    /// or a function/method name
    Ident(Ident<'a>),
    /// A word that has been reserved to not be used as an identifier
    Keyword(Keyword),
    /// A `null` literal value
    Null,
    /// A number, this includes integers (`1`), decimals (`0.1`),
    /// hex (`0x8f`), binary (`0b010011010`), and octal (`0o273`)
    Numeric(Number<'a>),
    /// A punctuation mark, this includes all mathematical operators
    /// logical operators and general syntax punctuation
    Punct(Punct),
    /// A string literal, either double or single quoted, the associated
    /// value will be the unquoted string
    String(StringLit<'a>),
    /// A regular expression literal.
    /// ```js
    /// let regex = /[a-zA-Z]+/g;
    /// ```
    RegEx(RegEx<'a>),
    /// The string parts of a template string
    Template(Template<'a>),
    /// A comment, the associated value will contain the raw comment
    /// This will capture both inline comments `// I am an inline comment`
    /// and multi-line comments
    /// ```js
    /// /*multi lines
    /// * comments
    /// */
    /// ```
    Comment(Comment<'a>),
}
#[derive(Debug, PartialEq, Clone)]
/// An identifier
pub struct Ident<'a>(&'a str);

impl<'a> PartialEq<str> for Ident<'a> {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

impl<'a> Ident<'a> {
    pub fn matches(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl<'a> From<&'a str> for Ident<'a> {
    fn from(s: &'a str) -> Self {
        Ident(s.into())
    }
}

impl<'a> ToString for Ident<'a> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<'a> Into<String> for Ident<'a> {
    fn into(self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Comment<'a> {
    pub kind: CommentKind,
    pub content: &'a str,
    pub tail_content: Option<&'a str>,
}

impl<'a> Comment<'a> {
    pub fn from_parts(content: &'a str, kind: CommentKind, tail_content: Option<&'a str>) -> Self {
        Comment {
            content,
            kind,
            tail_content,
        }
    }

    pub fn new_single_line(content: &'a str) -> Self {
        Comment::from_parts(content, CommentKind::Single, None)
    }

    pub fn new_multi_line(content: &'a str) -> Self {
        Comment::from_parts(content, CommentKind::Multi, None)
    }

    pub fn new_html(content: &'a str, tail_content: Option<&'a str>) -> Self {
        Comment::from_parts(content, CommentKind::Html, tail_content)
    }

    pub fn new_html_no_tail(content: &'a str) -> Self {
        Comment::new_html(content, None)
    }

    pub fn new_html_with_tail(content: &'a str, tail: &'a str) -> Self {
        Comment::new_html(content, Some(tail))
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

impl<'a> ToString for Comment<'a> {
    fn to_string(&self) -> String {
        match self.kind {
            CommentKind::Single => format!("//{}", self.content),
            CommentKind::Multi => format!("/*{}*/", self.content),
            CommentKind::Html => format!("<!--{}-->", self.content),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Number<'a>(&'a str);

impl<'a> Number<'a> {
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

impl<'a> From<&'a str> for Number<'a> {
    fn from(s: &'a str) -> Self {
        Number(s)
    }
}

impl<'a> ToString for Number<'a> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<'a> PartialEq<str> for Number<'a> {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegEx<'a> {
    pub body: &'a str,
    pub flags: Option<&'a str>,
}

impl<'a> RegEx<'a> {
    pub fn from_parts(body: &'a str, flags: Option<&'a str>) -> Self {
        let flags = if let Some(flags) = flags {
            if flags == "" {
                None
            } else {
                Some(flags)
            }
        } else {
            None
        };
        RegEx { body: body, flags }
    }
}

impl<'a> ToString for RegEx<'a> {
    fn to_string(&self) -> String {
        let f = if let Some(ref f) = self.flags {
            f.to_string()
        } else {
            String::new()
        };
        format!("/{}/{}", self.body, f)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StringLit<'a> {
    Single(&'a str),
    Double(&'a str),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Template<'a> {
    NoSub(&'a str),
    Head(&'a str),
    Middle(&'a str),
    Tail(&'a str),
}

impl<'a> ToString for StringLit<'a> {
    fn to_string(&self) -> String {
        match self {
            StringLit::Single(ref s) => format!(r#"'{}'"#, s),
            StringLit::Double(ref s) => format!(r#""{}""#, s),
        }
    }
}

impl<'a> StringLit<'a> {
    pub fn single(content: &'a str) -> Self {
        StringLit::Single(content)
    }
    pub fn double(content: &'a str) -> Self {
        StringLit::Double(content)
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
            StringLit::Single(ref inner) => inner.to_string(),
            StringLit::Double(ref inner) => inner.to_string(),
        }
    }
}

impl<'a> Template<'a> {
    pub fn no_sub_template(content: &'a str) -> Self {
        Template::NoSub(content)
    }
    pub fn template_head(content: &'a str) -> Self {
        Template::Head(content)
    }
    pub fn template_middle(content: &'a str) -> Self {
        Template::Middle(content)
    }
    pub fn template_tail(content: &'a str) -> Self {
        Template::Tail(content)
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

impl<'a> ToString for Template<'a> {
    fn to_string(&self) -> String {
        match self {
            Template::NoSub(ref c) => format!("`{}`", c),
            Template::Head(ref c) => format!("`{}${{", c),
            Template::Middle(ref c) => format!("}}{}${{", c),
            Template::Tail(ref c) => format!("}}{}`", c),
        }
    }
}
