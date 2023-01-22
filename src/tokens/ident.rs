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

impl<T> ToString for Ident<T>
where
    T: AsRef<str>,
{
    fn to_string(&self) -> String {
        self.0.as_ref().to_string()
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
