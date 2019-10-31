use crate::tokens::{CommentKind, Keyword, NumberKind, Punct};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RawToken {
    /// `true` of `false`
    Boolean(bool),
    /// The end of the file
    EoF,
    /// An identifier this will be either a variable name
    /// or a function/method name
    Ident,
    /// A word that has been reserved to not be used as an identifier
    Keyword(RawKeyword),
    /// A `null` literal value
    Null,
    /// A number, this includes integers (`1`), decimals (`0.1`),
    /// hex (`0x8f`), binary (`0b010011010`), and octal (`0o273`)
    Number(NumberKind),
    /// A punctuation mark, this includes all mathematical operators
    /// logical operators and general syntax punctuation
    Punct(Punct),
    /// A string literal, either double or single quoted, the associated
    /// value will be the unquoted string
    String {
        kind: StringKind,
        new_line_count: usize,
        last_len: usize,
    },
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
    Template {
        kind: TemplateKind,
        new_line_count: usize,
        last_len: usize,
    },
    /// A comment, the associated value will contain the raw comment
    /// This will capture both inline comments `// I am an inline comment`
    /// and multi-line comments
    /// ```js
    /// /*multi lines
    /// * comments
    /// */
    /// ```
    Comment {
        kind: CommentKind,
        new_line_count: usize,
        last_len: usize,
    },
}

impl Copy for Keyword<()> { }

impl RawToken {
    pub fn is_punct(&self) -> bool {
        match self {
            RawToken::Punct(_) => true,
            _ => false,
        }
    }

    pub fn is_comment(&self) -> bool {
        match self {
            RawToken::Comment { .. } => true,
            _ => false,
        }
    }
    pub fn is_div_punct(&self) -> bool {
        match self {
            RawToken::Punct(ref p) => match p {
                Punct::ForwardSlash => true,
                Punct::ForwardSlashEqual => true,
                _ => false,
            },
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
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RawKeyword {
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    Finally,
    For,
    Function,
    If,
    Implements,
    Import,
    In,
    InstanceOf,
    Interface,
    Let,
    New,
    Package,
    Private,
    Protected,
    Public,
    Return,
    Static,
    Super,
    Switch,
    This,
    Throw,
    Try,
    TypeOf,
    Var,
    Void,
    While,
    With,
    Yield,
}

impl RawKeyword {
    pub fn with_str<'a>(&self, s: &'a str) -> crate::tokens::Keyword<&'a str> {
        match self {
            RawKeyword::Await => Keyword::Await(s),
            RawKeyword::Break => Keyword::Break(s),
            RawKeyword::Case => Keyword::Case(s),
            RawKeyword::Catch => Keyword::Catch(s),
            RawKeyword::Class => Keyword::Class(s),
            RawKeyword::Const => Keyword::Const(s),
            RawKeyword::Continue => Keyword::Continue(s),
            RawKeyword::Debugger => Keyword::Debugger(s),
            RawKeyword::Default => Keyword::Default(s),
            RawKeyword::Delete => Keyword::Delete(s),
            RawKeyword::Do => Keyword::Do(s),
            RawKeyword::Else => Keyword::Else(s),
            RawKeyword::Enum => Keyword::Enum(s),
            RawKeyword::Export => Keyword::Export(s),
            RawKeyword::Extends => Keyword::Extends(s),
            RawKeyword::Finally => Keyword::Finally(s),
            RawKeyword::For => Keyword::For(s),
            RawKeyword::Function => Keyword::Function(s),
            RawKeyword::If => Keyword::If(s),
            RawKeyword::Implements => Keyword::Implements(s),
            RawKeyword::Import => Keyword::Import(s),
            RawKeyword::In => Keyword::In(s),
            RawKeyword::InstanceOf => Keyword::InstanceOf(s),
            RawKeyword::Interface => Keyword::Interface(s),
            RawKeyword::Let => Keyword::Let(s),
            RawKeyword::New => Keyword::New(s),
            RawKeyword::Package => Keyword::Package(s),
            RawKeyword::Private => Keyword::Private(s),
            RawKeyword::Protected => Keyword::Protected(s),
            RawKeyword::Public => Keyword::Public(s),
            RawKeyword::Return => Keyword::Return(s),
            RawKeyword::Static => Keyword::Static(s),
            RawKeyword::Super => Keyword::Super(s),
            RawKeyword::Switch => Keyword::Switch(s),
            RawKeyword::This => Keyword::This(s),
            RawKeyword::Throw => Keyword::Throw(s),
            RawKeyword::Try => Keyword::Try(s),
            RawKeyword::TypeOf => Keyword::TypeOf(s),
            RawKeyword::Var => Keyword::Var(s),
            RawKeyword::Void => Keyword::Void(s),
            RawKeyword::While => Keyword::While(s),
            RawKeyword::With => Keyword::With(s),
            RawKeyword::Yield => Keyword::Yield(s),
        }
    }
}