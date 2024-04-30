use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl<T> Display for RegEx<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/{}/", self.body)?;

        if let Some(flags) = &self.flags {
            write!(f, "{}", flags)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(RegEx::from_parts("regex", None).to_string(), "/regex/");
        assert_eq!(
            RegEx::from_parts("regex", Some("g")).to_string(),
            "/regex/g"
        );
    }
}
