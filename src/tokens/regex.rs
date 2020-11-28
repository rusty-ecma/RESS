#[derive(Debug, PartialEq, Clone)]
/// A Regular Expression Literal
///
/// These being with a `/` and the
/// body ends with another `/`
/// optionally a series of one letter
/// flags can be included after the `/`
pub struct RegEx<T> {
    pub body: T,
    pub flags: Option<T>,
}

impl<T> RegEx<T> {
    pub fn from_parts(body: T, flags: Option<T>) -> Self {
        RegEx { body, flags }
    }
}

impl<T> ToString for RegEx<T>
where
    T: AsRef<str>,
{
    fn to_string(&self) -> String {
        let f = if let Some(f) = &self.flags {
            f.as_ref().to_string()
        } else {
            String::new()
        };
        format!("/{}/{}", self.body.as_ref(), f)
    }
}
