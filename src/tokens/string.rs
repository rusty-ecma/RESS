#[derive(Debug, PartialEq, Eq, Clone)]
/// A single or double quoted string
/// literal
pub enum StringLit<T> {
    Single(InnerString<T>),
    Double(InnerString<T>),
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InnerString<T> {
    pub content: T,
    pub contains_octal_escape: bool,
}

impl<T> ToString for StringLit<T>
where
    T: AsRef<str>,
{
    fn to_string(&self) -> String {
        match self {
            StringLit::Single(ref s) => format!(r#"'{}'"#, s.content.as_ref()),
            StringLit::Double(ref s) => format!(r#""{}""#, s.content.as_ref()),
        }
    }
}

impl<T> AsRef<str> for StringLit<T>
where
    T: AsRef<str>,
{
    fn as_ref(&self) -> &str {
        match self {
            StringLit::Single(s) | StringLit::Double(s) => s.as_ref(),
        }
    }
}

impl<T> AsRef<str> for InnerString<T>
where
    T: AsRef<str>,
{
    fn as_ref(&self) -> &str {
        self.content.as_ref()
    }
}

impl<T> StringLit<T> {
    pub fn single(content: T, oct: bool) -> Self {
        StringLit::Single(InnerString {
            content,
            contains_octal_escape: oct,
        })
    }
    pub fn double(content: T, oct: bool) -> Self {
        StringLit::Double(InnerString {
            content,
            contains_octal_escape: oct,
        })
    }
    pub fn is_single(&self) -> bool {
        matches!(self, StringLit::Single(_))
    }
    pub fn is_double(&self) -> bool {
        matches!(self, StringLit::Double(_))
    }
    pub fn has_octal_escape(&self) -> bool {
        match self {
            StringLit::Single(ref inner) | StringLit::Double(ref inner) => {
                inner.contains_octal_escape
            }
        }
    }
}
