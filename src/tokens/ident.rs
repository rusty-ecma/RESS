use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
/// An identifier
pub struct Ident<T>(T);

impl<T> AsRef<str> for Ident<T>
where
    T: AsRef<str>,
{
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T> PartialEq<str> for &Ident<T>
where
    T: AsRef<str>,
{
    fn eq(&self, other: &str) -> bool {
        self.0.as_ref().eq(other)
    }
}

impl<'a> From<&'a str> for Ident<&'a str> {
    fn from(s: &'a str) -> Self {
        Ident(s)
    }
}

impl<T> Display for Ident<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<Ident<T>> for String
where
    T: ToString,
{
    fn from(id: Ident<T>) -> Self {
        id.0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        for raw in ["ident1", "ident2", "ident3"] {
            let ident = Ident::from(raw);
            assert_eq!(ident.to_string(), raw);
            assert_eq!(Into::<String>::into(ident), raw);
        }
    }
}
