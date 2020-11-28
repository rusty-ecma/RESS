#[derive(Debug, PartialEq, Clone, Copy)]
/// The tokenized representation of `true` or `false`
pub enum Boolean {
    True,
    False,
}
impl PartialEq<bool> for Boolean {
    fn eq(&self, other: &bool) -> bool {
        match (self, other) {
            (Boolean::True, true) | (Boolean::False, false) => true,
            _ => false,
        }
    }
}
impl PartialEq<str> for Boolean {
    fn eq(&self, other: &str) -> bool {
        match (self, other) {
            (Boolean::True, "true") | (Boolean::False, "false") => true,
            _ => false,
        }
    }
}
impl Boolean {
    /// Test if this instance represents `true`
    pub fn is_true(self) -> bool {
        match self {
            Boolean::True => true,
            _ => false,
        }
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

impl Into<String> for Boolean {
    /// Return this Boolean to the text
    /// that was parsed to create it
    fn into(self) -> String {
        match self {
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

impl Into<bool> for Boolean {
    /// Creates a Rust bool for a js bool
    fn into(self) -> bool {
        match self {
            Boolean::True => true,
            Boolean::False => false,
        }
    }
}

impl<'a> Into<bool> for &'a Boolean {
    /// Creates a js bool for a rust bool
    fn into(self) -> bool {
        match self {
            Boolean::True => true,
            Boolean::False => false,
        }
    }
}
