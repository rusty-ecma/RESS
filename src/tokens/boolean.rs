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

impl Boolean {
    /// Create a Boolean from raw text
    pub fn from(s: &str) -> Option<Self> {
        if s == "true" {
            Some(Boolean::True)
        } else if s == "false" {
            Some(Boolean::False)
        } else {
            None
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
        match b {
            Boolean::True => "true".into(),
            Boolean::False => "false".into(),
        }
    }
}

impl ToString for Boolean {
    /// Return this Boolean to the text
    /// that was parsed to create it
    fn to_string(&self) -> String {
        match self {
            Boolean::True => "true".into(),
            Boolean::False => "false".into(),
        }
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
