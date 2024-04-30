use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// The tokenized representation of `true` or `false`
pub enum Boolean {
    True,
    False,
}

impl PartialEq<bool> for Boolean {
    fn eq(&self, other: &bool) -> bool {
        matches!(
            (self, other),
            (Boolean::True, true) | (Boolean::False, false)
        )
    }
}
impl PartialEq<str> for Boolean {
    fn eq(&self, other: &str) -> bool {
        matches!(
            (self, other),
            (Boolean::True, "true") | (Boolean::False, "false")
        )
    }
}
impl Boolean {
    /// Test if this instance represents `true`
    pub fn is_true(self) -> bool {
        matches!(self, Boolean::True)
    }
}

impl From<&str> for Boolean {
    /// Create a Boolean from raw text
    fn from(s: &str) -> Self {
        if s == "true" {
            Boolean::True
        } else if s == "false" {
            Boolean::False
        } else {
            panic!("invalid boolean: {}", s);
        }
    }
}

impl From<bool> for Boolean {
    /// Creates a JS Bool for a rust bool
    fn from(b: bool) -> Self {
        if b {
            Boolean::True
        } else {
            Boolean::False
        }
    }
}

impl From<Boolean> for String {
    /// Return this Boolean to the text
    /// that was parsed to create it
    fn from(b: Boolean) -> String {
        b.to_string()
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Boolean::True => "true",
            Boolean::False => "false",
        }
        .fmt(f)
    }
}

impl From<Boolean> for bool {
    /// Creates a Rust bool for a js bool
    fn from(b: Boolean) -> bool {
        match b {
            Boolean::True => true,
            Boolean::False => false,
        }
    }
}

impl From<&Boolean> for bool {
    /// Creates a js bool for a rust bool
    fn from(b: &Boolean) -> bool {
        match b {
            Boolean::True => true,
            Boolean::False => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(Boolean::True.to_string(), "true");
        assert_eq!(Boolean::False.to_string(), "false");
        assert_eq!(Into::<String>::into(Boolean::True), "true");
        assert_eq!(Into::<String>::into(Boolean::False), "false");
    }

    #[test]
    fn ctors_and_helpers() {
        assert!(Boolean::from("true").is_true());
        assert!(!Boolean::from("false").is_true());
        assert!(Boolean::from(true).is_true());
        assert!(!Boolean::from(false).is_true());
        assert!(Into::<bool>::into(Boolean::True));
        assert!(!Into::<bool>::into(Boolean::False));
    }
}
