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
        has_octal_escape: bool,
        found_invalid_unicode_escape: bool,
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

impl Copy for Keyword<()> {}

impl RawToken {
    pub fn is_punct(&self) -> bool {
        if let RawToken::Punct(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_comment(&self) -> bool {
        if let RawToken::Comment { .. } = self {
            true
        } else {
            false
        }
    }
    pub fn is_div_punct(&self) -> bool {
        if let RawToken::Punct(ref p) = self {
            match p {
                Punct::ForwardSlash => true,
                Punct::ForwardSlashEqual => true,
                _ => false,
            }
        } else {
            false
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
    pub fn with_str(self, s: &str) -> crate::tokens::Keyword<&str> {
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

impl<T> From<&Keyword<T>> for RawKeyword {
    fn from(k: &Keyword<T>) -> Self {
        match k {
            Keyword::Await(_) => RawKeyword::Await,
            Keyword::Break(_) => RawKeyword::Break,
            Keyword::Case(_) => RawKeyword::Case,
            Keyword::Catch(_) => RawKeyword::Catch,
            Keyword::Class(_) => RawKeyword::Class,
            Keyword::Const(_) => RawKeyword::Const,
            Keyword::Continue(_) => RawKeyword::Continue,
            Keyword::Debugger(_) => RawKeyword::Debugger,
            Keyword::Default(_) => RawKeyword::Default,
            Keyword::Delete(_) => RawKeyword::Delete,
            Keyword::Do(_) => RawKeyword::Do,
            Keyword::Else(_) => RawKeyword::Else,
            Keyword::Enum(_) => RawKeyword::Enum,
            Keyword::Export(_) => RawKeyword::Export,
            Keyword::Extends(_) => RawKeyword::Extends,
            Keyword::Finally(_) => RawKeyword::Finally,
            Keyword::For(_) => RawKeyword::For,
            Keyword::Function(_) => RawKeyword::Function,
            Keyword::If(_) => RawKeyword::If,
            Keyword::Implements(_) => RawKeyword::Implements,
            Keyword::Import(_) => RawKeyword::Import,
            Keyword::In(_) => RawKeyword::In,
            Keyword::InstanceOf(_) => RawKeyword::InstanceOf,
            Keyword::Interface(_) => RawKeyword::Interface,
            Keyword::Let(_) => RawKeyword::Let,
            Keyword::New(_) => RawKeyword::New,
            Keyword::Package(_) => RawKeyword::Package,
            Keyword::Private(_) => RawKeyword::Private,
            Keyword::Protected(_) => RawKeyword::Protected,
            Keyword::Public(_) => RawKeyword::Public,
            Keyword::Return(_) => RawKeyword::Return,
            Keyword::Static(_) => RawKeyword::Static,
            Keyword::Super(_) => RawKeyword::Super,
            Keyword::Switch(_) => RawKeyword::Switch,
            Keyword::This(_) => RawKeyword::This,
            Keyword::Throw(_) => RawKeyword::Throw,
            Keyword::Try(_) => RawKeyword::Try,
            Keyword::TypeOf(_) => RawKeyword::TypeOf,
            Keyword::Var(_) => RawKeyword::Var,
            Keyword::Void(_) => RawKeyword::Void,
            Keyword::While(_) => RawKeyword::While,
            Keyword::With(_) => RawKeyword::With,
            Keyword::Yield(_) => RawKeyword::Yield,
        }
    }
}