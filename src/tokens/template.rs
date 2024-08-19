use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
/// A template string
///
/// These include strings that are wrapped in back ticks (`)
/// which allows for interpolating any js expression between `${`
/// and `}`
pub enum Template<T> {
    NoSub(TemplateLiteral<T>),
    Head(TemplateLiteral<T>),
    Middle(TemplateLiteral<T>),
    Tail(TemplateLiteral<T>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TemplateLiteral<T> {
    pub content: T,
    pub contains_octal_escape: bool,
    pub contains_invalid_unicode_escape: bool,
    pub contains_invalid_hex_escape: bool,
}
impl<T> TemplateLiteral<T> {
    pub fn new(
        content: T,
        contains_octal_escape: bool,
        contains_invalid_unicode_escape: bool,
        contains_invalid_hex_escape: bool,
    ) -> Self {
        Self {
            content,
            contains_octal_escape,
            contains_invalid_unicode_escape,
            contains_invalid_hex_escape,
        }
    }
}

impl<T> Template<T> {
    pub fn no_sub_template(content: T, oct: bool, uni: bool, hex: bool) -> Self {
        Template::NoSub(TemplateLiteral::new(content, oct, uni, hex))
    }
    pub fn template_head(content: T, oct: bool, uni: bool, hex: bool) -> Self {
        Template::Head(TemplateLiteral::new(content, oct, uni, hex))
    }
    pub fn template_middle(content: T, oct: bool, uni: bool, hex: bool) -> Self {
        Template::Middle(TemplateLiteral::new(content, oct, uni, hex))
    }
    pub fn template_tail(content: T, oct: bool, uni: bool, hex: bool) -> Self {
        Template::Tail(TemplateLiteral::new(content, oct, uni, hex))
    }
    pub fn is_head(&self) -> bool {
        matches!(self, Template::Head(_))
    }
    pub fn is_middle(&self) -> bool {
        matches!(self, Template::Middle(_))
    }
    pub fn is_tail(&self) -> bool {
        matches!(self, Template::Tail(_))
    }
    pub fn is_no_sub(&self) -> bool {
        matches!(self, Template::NoSub(_))
    }
}

impl<T> Display for Template<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Template::NoSub(ref t) => write!(f, "`{}`", t.content),
            Template::Head(ref t) => write!(f, "`{}${{", t.content),
            Template::Middle(ref t) => write!(f, "}}{}${{", t.content),
            Template::Tail(ref t) => write!(f, "}}{}`", t.content),
        }
    }
}
