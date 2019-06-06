use tokens::{Keyword, Punct, NumberKind, CommentKind,};
#[derive(PartialEq, Debug)]
pub enum RawToken {
    /// `true` of `false`
    Boolean(bool),
    /// The end of the file
    EoF,
    /// An identifier this will be either a variable name
    /// or a function/method name
    Ident,
    /// A word that has been reserved to not be used as an identifier
    Keyword(Keyword),
    /// A `null` literal value
    Null,
    /// A number, this includes integers (`1`), decimals (`0.1`),
    /// hex (`0x8f`), binary (`0b010011010`), and octal (`0o273`)
    Numeric(NumberKind),
    /// A punctuation mark, this includes all mathematical operators
    /// logical operators and general syntax punctuation
    Punct(Punct),
    /// A string literal, either double or single quoted, the associated
    /// value will be the unquoted string
    String(StringKind),
    /// A regular expression literal.
    /// ```js
    /// let regex = /[a-zA-Z]+/g;
    /// ```
    RegEx(usize),
    /// The string parts of a template string
    /// ```js
    ///    `things and stuff times ${10}`
    /// //  ^^^^^^^^^^^^^^^^^^^^^^      ^
    /// ```
    Template(TemplateKind),
    /// A comment, the associated value will contain the raw comment
    /// This will capture both inline comments `// I am an inline comment`
    /// and multi-line comments
    /// ```js
    /// /*multi lines
    /// * comments
    /// */
    /// ```
    Comment(CommentKind),
}

impl RawToken {
    pub fn is_punct(&self) -> bool {
        match self {
            RawToken::Punct(_) => true,
            _ => false,
        }
    }

    pub fn is_comment(&self) -> bool {
        match self {
            RawToken::Comment(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StringKind {
    Double,
    Single,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TemplateKind {
    NoSub,
    Head,
    Body,
    Tail,
}