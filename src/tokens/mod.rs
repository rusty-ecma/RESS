pub mod owned;
pub mod refs;

pub trait Token {
    fn is_boolean(&self) -> bool;

    fn is_boolean_true(&self) -> bool;

    fn is_boolean_false(&self) -> bool;

    fn is_eof(&self) -> bool;

    fn is_ident(&self) -> bool;

    fn is_keyword(&self) -> bool;

    fn is_strict_reserved(&self) -> bool;

    fn is_restricted(&self) -> bool;

    fn is_null(&self) -> bool;

    fn is_number(&self) -> bool;

    fn is_hex_literal(&self) -> bool;

    fn is_bin_literal(&self) -> bool;

    fn is_oct_literal(&self) -> bool;

    fn is_punct(&self) -> bool;

    fn is_string(&self) -> bool;

    fn is_double_quoted_string(&self) -> bool;

    fn is_single_quoted_string(&self) -> bool;

    fn is_regex(&self) -> bool;

    fn is_template(&self) -> bool;

    fn is_template_no_sub(&self) -> bool;

    fn is_template_head(&self) -> bool;

    fn is_template_body(&self) -> bool;

    fn is_template_tail(&self) -> bool;

    fn is_literal(&self) -> bool;

    fn is_comment(&self) -> bool;

    fn is_multi_line_comment(&self) -> bool;

    fn is_single_line_comment(&self) -> bool;

    fn matches_boolean(&self, b: BooleanLiteral) -> bool;

    fn matches_boolean_str(&self, b: &str) -> bool;

    fn matches_ident_str(&self, name: &str) -> bool;

    fn matches_keyword(&self, keyword: Keyword) -> bool;

    fn matches_keyword_str(&self, name: &str) -> bool;

    fn matches_number_str(&self, number: &str) -> bool;

    fn matches_punct(&self, p: Punct) -> bool;

    fn matches_punct_str(&self, s: &str) -> bool;

    fn matches_comment_str(&self, comment: &str) -> bool;

    fn matches_string_content(&self, content: &str) -> bool;
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// The tokenized representation of `true` or `false`
pub enum BooleanLiteral {
    True,
    False,
}
impl BooleanLiteral {
    /// Test if this instance represents `true`
    pub fn is_true(self) -> bool {
        match self {
            BooleanLiteral::True => true,
            _ => false,
        }
    }
}

impl BooleanLiteral {
    /// Create a BooleanLiteral from raw text
    pub fn from(s: &str) -> Option<Self> {
        if s == "true" {
            Some(BooleanLiteral::True)
        } else if s == "false" {
            Some(BooleanLiteral::False)
        } else {
            None
        }
    }
}

impl From<bool> for BooleanLiteral {
    /// Creates a JS Bool for a rust bool
    fn from(b: bool) -> Self {
        if b {
            BooleanLiteral::True
        } else {
            BooleanLiteral::False
        }
    }
}

impl Into<String> for BooleanLiteral {
    /// Return this BooleanLiteral to the text
    /// that was parsed to create it
    fn into(self) -> String {
        match self {
            BooleanLiteral::True => "true".into(),
            BooleanLiteral::False => "false".into(),
        }
    }
}

impl ToString for BooleanLiteral {
    /// Return this BooleanLiteral to the text
    /// that was parsed to create it
    fn to_string(&self) -> String {
        match self {
            BooleanLiteral::True => "true".into(),
            BooleanLiteral::False => "false".into(),
        }
    }
}

impl Into<bool> for BooleanLiteral {
    /// Creates a Rust bool for a js bool
    fn into(self) -> bool {
        match self {
            BooleanLiteral::True => true,
            BooleanLiteral::False => false,
        }
    }
}

impl<'a> Into<bool> for &'a BooleanLiteral {
    /// Creates a js bool for a rust bool
    fn into(self) -> bool {
        match self {
            BooleanLiteral::True => true,
            BooleanLiteral::False => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NumberKind {
    Dec,
    Hex,
    Bin,
    Oct,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Punct {
    Ampersand,
    AmpersandEqual,
    Asterisk,
    AsteriskEqual,
    AtMark,
    Bang,
    BangDoubleEqual,
    BangEqual,
    Caret,
    CaretEqual,
    CloseBrace,
    CloseBracket,
    CloseParen,
    Colon,
    Comma,
    Dash,
    DoubleDash,
    DashEqual,
    DoubleAmpersand,
    DoubleAsterisk,
    DoubleAsteriskEqual,
    DoubleEqual,
    DoubleGreaterThan,
    DoubleGreaterThanEqual,
    DoubleLessThan,
    DoubleLessThanEqual,
    DoublePipe,
    DoublePlus,
    Ellipsis,
    Equal,
    EqualGreaterThan,
    ForwardSlash,
    ForwardSlashEqual,
    GreaterThan,
    GreaterThanEqual,
    Hash,
    LessThan,
    LessThanEqual,
    OpenBrace,
    OpenBracket,
    OpenParen,
    Percent,
    PercentEqual,
    Period,
    Pipe,
    PipeEqual,
    Plus,
    PlusEqual,
    QuestionMark,
    SemiColon,
    Tilde,
    TripleEqual,
    TripleGreaterThanEqual,
    TripleGreaterThan,
}

impl Punct {
    fn matches_str(self, s: &str) -> bool {
        match self {
            Punct::OpenBrace => "{" == s,
            Punct::CloseBrace => "}" == s,
            Punct::OpenParen => "(" == s,
            Punct::CloseParen => ")" == s,
            Punct::Period => "." == s,
            Punct::SemiColon => ";" == s,
            Punct::Comma => "," == s,
            Punct::OpenBracket => "[" == s,
            Punct::CloseBracket => "]" == s,
            Punct::Colon => ":" == s,
            Punct::QuestionMark => "?" == s,
            Punct::Tilde => "~" == s,
            Punct::GreaterThan => ">" == s,
            Punct::LessThan => "<" == s,
            Punct::Equal => "=" == s,
            Punct::Bang => "!" == s,
            Punct::Plus => "+" == s,
            Punct::Dash => "-" == s,
            Punct::Asterisk => "*" == s,
            Punct::Percent => "%" == s,
            Punct::Pipe => "|" == s,
            Punct::Ampersand => "&" == s,
            Punct::Caret => "^" == s,
            Punct::ForwardSlash => "/" == s,
            Punct::TripleGreaterThanEqual => ">>>=" == s,
            Punct::Ellipsis => "..." == s,
            Punct::TripleEqual => "===" == s,
            Punct::BangDoubleEqual => "!==" == s,
            Punct::TripleGreaterThan => ">>>" == s,
            Punct::DoubleLessThanEqual => "<<=" == s,
            Punct::DoubleGreaterThanEqual => ">>=" == s,
            Punct::DoubleAsteriskEqual => "**=" == s,
            Punct::DoubleAmpersand => "&&" == s,
            Punct::DoublePipe => "||" == s,
            Punct::DoubleEqual => "==" == s,
            Punct::BangEqual => "!=" == s,
            Punct::PlusEqual => "+=" == s,
            Punct::DashEqual => "-=" == s,
            Punct::AsteriskEqual => "*=" == s,
            Punct::ForwardSlashEqual => "/=" == s,
            Punct::DoublePlus => "++" == s,
            Punct::DoubleDash => "--" == s,
            Punct::DoubleLessThan => "<<" == s,
            Punct::DoubleGreaterThan => ">>" == s,
            Punct::AmpersandEqual => "&=" == s,
            Punct::PipeEqual => "|=" == s,
            Punct::CaretEqual => "^=" == s,
            Punct::PercentEqual => "%=" == s,
            Punct::EqualGreaterThan => "=>" == s,
            Punct::GreaterThanEqual => ">=" == s,
            Punct::LessThanEqual => "<=" == s,
            Punct::DoubleAsterisk => "**" == s,
            Punct::Hash => "#" == s,
            Punct::AtMark => "@" == s,
        }
    }
}

impl ToString for Punct {
    fn to_string(&self) -> String {
        match self {
            Punct::OpenBrace => "{",
            Punct::CloseBrace => "}",
            Punct::OpenParen => "(",
            Punct::CloseParen => ")",
            Punct::Period => ".",
            Punct::SemiColon => ";",
            Punct::Comma => ",",
            Punct::OpenBracket => "[",
            Punct::CloseBracket => "]",
            Punct::Colon => ":",
            Punct::QuestionMark => "?",
            Punct::Tilde => "~",
            Punct::GreaterThan => ">",
            Punct::LessThan => "<",
            Punct::Equal => "=",
            Punct::Bang => "!",
            Punct::Plus => "+",
            Punct::Dash => "-",
            Punct::Asterisk => "*",
            Punct::Percent => "%",
            Punct::Pipe => "|",
            Punct::Ampersand => "&",
            Punct::Caret => "^",
            Punct::ForwardSlash => "/",
            Punct::TripleGreaterThanEqual => ">>>=",
            Punct::Ellipsis => "...",
            Punct::TripleEqual => "===",
            Punct::BangDoubleEqual => "!==",
            Punct::TripleGreaterThan => ">>>",
            Punct::DoubleLessThanEqual => "<<=",
            Punct::DoubleGreaterThanEqual => ">>=",
            Punct::DoubleAsteriskEqual => "**=",
            Punct::DoubleAmpersand => "&&",
            Punct::DoublePipe => "||",
            Punct::DoubleEqual => "==",
            Punct::BangEqual => "!=",
            Punct::PlusEqual => "+=",
            Punct::DashEqual => "-=",
            Punct::AsteriskEqual => "*=",
            Punct::ForwardSlashEqual => "/=",
            Punct::DoublePlus => "++",
            Punct::DoubleDash => "--",
            Punct::DoubleLessThan => "<<",
            Punct::DoubleGreaterThan => ">>",
            Punct::AmpersandEqual => "&=",
            Punct::PipeEqual => "|=",
            Punct::CaretEqual => "^=",
            Punct::PercentEqual => "%=",
            Punct::EqualGreaterThan => "=>",
            Punct::GreaterThanEqual => ">=",
            Punct::LessThanEqual => "<=",
            Punct::DoubleAsterisk => "**",
            Punct::Hash => "#",
            Punct::AtMark => "@",
        }
        .into()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CommentKind {
    Single,
    Multi,
    Html,
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// A JS Keyword
///
/// #Standard
/// await
/// break
/// case
/// catch
/// class
/// const
/// continue
/// debugger
/// default
/// delete (10)
/// do
/// else
/// export
/// extends
/// finally
/// for
/// function
/// if
/// import
/// in (20)
/// instanceof
/// new
/// return
/// super
/// switch
/// this
/// throw
/// try
/// typeof
/// var (30)
/// void
/// while
/// with
/// yield
/// # Future Reserved
/// enum
/// # Strict Mode Future Reserved
/// implements
/// package
/// protected
/// interface
/// private (40)
/// public
pub enum Keyword {
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete, //10
    Do,
    Else,
    Enum,
    Export,
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

impl Keyword {
    /// convert a &str into a Keyword
    pub fn from(s: &str) -> Option<Self> {
        Some(match s {
            "await" => Keyword::Await,
            "break" => Keyword::Break,
            "case" => Keyword::Case,
            "catch" => Keyword::Catch,
            "class" => Keyword::Class,
            "const" => Keyword::Const,
            "continue" => Keyword::Continue,
            "debugger" => Keyword::Debugger,
            "default" => Keyword::Default,
            "delete" => Keyword::Delete,
            "do" => Keyword::Do,
            "else" => Keyword::Else,
            "finally" => Keyword::Finally,
            "for" => Keyword::For,
            "function" => Keyword::Function,
            "if" => Keyword::If,
            "instanceof" => Keyword::InstanceOf,
            "in" => Keyword::In,
            "new" => Keyword::New,
            "return" => Keyword::Return,
            "switch" => Keyword::Switch,
            "this" => Keyword::This,
            "throw" => Keyword::Throw,
            "try" => Keyword::Try,
            "typeof" => Keyword::TypeOf,
            "var" => Keyword::Var,
            "void" => Keyword::Void,
            "while" => Keyword::While,
            "with" => Keyword::With,
            "export" => Keyword::Export,
            "import" => Keyword::Import,
            "super" => Keyword::Super,
            "enum" => Keyword::Enum,
            "implements" => Keyword::Implements,
            "interface" => Keyword::Interface,
            "package" => Keyword::Package,
            "private" => Keyword::Private,
            "protected" => Keyword::Protected,
            "public" => Keyword::Public,
            "static" => Keyword::Static,
            "yield" => Keyword::Yield,
            "let" => Keyword::Let,
            _ => return None,
        })
    }
}

impl ::std::string::ToString for Keyword {
    /// Convert a keyword into a string
    fn to_string(&self) -> String {
        self.as_str().into()
    }
}

impl Keyword {
    /// Is this keyword one of the future reserved words
    ///
    /// - enum
    /// - export
    /// - implements
    /// - super
    pub fn is_future_reserved(self) -> bool {
        match self {
            Keyword::Enum => true,
            Keyword::Export => true,
            Keyword::Implements => true,
            Keyword::Super => true,
            _ => false,
        }
    }
    /// Is this keyword a reserved word when the context
    /// has a 'use strict' directive.
    ///
    /// ## Keywords
    /// - implements
    /// - interface
    /// - package
    /// - private
    /// - protected
    /// - public
    /// - static
    /// - yield
    /// - let
    pub fn is_strict_reserved(self) -> bool {
        match self {
            Keyword::Implements => true,
            Keyword::Interface => true,
            Keyword::Package => true,
            Keyword::Private => true,
            Keyword::Protected => true,
            Keyword::Public => true,
            Keyword::Static => true,
            Keyword::Yield => true,
            Keyword::Let => true,
            _ => false,
        }
    }
    /// Is this keyword a reserved word
    ///
    /// ## Keywords
    /// - break
    /// - case
    /// - catch
    /// - continue
    /// - debugger
    /// - default
    /// - delete
    /// - do
    /// - else
    /// - for
    /// - function
    /// - if
    /// - instanceof
    /// - in
    /// - new
    /// - return
    /// - switch
    /// - this
    /// - throw
    /// - try
    /// - typeof
    /// - var
    /// - void
    /// - while
    /// - with
    pub fn is_reserved(self) -> bool {
        match self {
            Keyword::Break => true,
            Keyword::Case => true,
            Keyword::Catch => true,
            Keyword::Continue => true,
            Keyword::Debugger => true,
            Keyword::Default => true,
            Keyword::Delete => true,
            Keyword::Do => true,
            Keyword::Else => true,
            Keyword::Finally => true,
            Keyword::For => true,
            Keyword::Function => true,
            Keyword::If => true,
            Keyword::InstanceOf => true,
            Keyword::In => true,
            Keyword::New => true,
            Keyword::Return => true,
            Keyword::Switch => true,
            Keyword::This => true,
            Keyword::Throw => true,
            Keyword::Try => true,
            Keyword::TypeOf => true,
            Keyword::Var => true,
            Keyword::Void => true,
            Keyword::While => true,
            Keyword::With => true,
            _ => false,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Keyword::Await => "await",
            Keyword::Break => "break",
            Keyword::Case => "case",
            Keyword::Catch => "catch",
            Keyword::Class => "class",
            Keyword::Const => "const",
            Keyword::Continue => "continue",
            Keyword::Debugger => "debugger",
            Keyword::Default => "default",
            Keyword::Import => "import",
            Keyword::Delete => "delete",
            Keyword::Do => "do",
            Keyword::Else => "else",
            Keyword::Enum => "enum",
            Keyword::Export => "export",
            Keyword::Finally => "finally",
            Keyword::For => "for",
            Keyword::Function => "function",
            Keyword::If => "if",
            Keyword::In => "in",
            Keyword::Implements => "implements",
            Keyword::InstanceOf => "instanceof",
            Keyword::Interface => "interface",
            Keyword::Let => "let",
            Keyword::New => "new",
            Keyword::Package => "package",
            Keyword::Private => "private",
            Keyword::Protected => "protected",
            Keyword::Public => "public",
            Keyword::Static => "static",
            Keyword::Return => "return",
            Keyword::Super => "super",
            Keyword::Switch => "switch",
            Keyword::This => "this",
            Keyword::Throw => "throw",
            Keyword::Try => "try",
            Keyword::TypeOf => "typeof",
            Keyword::Var => "var",
            Keyword::Void => "void",
            Keyword::While => "while",
            Keyword::With => "with",
            Keyword::Yield => "yield",
        }
    }
}

impl Token for owned::Token {
    fn is_boolean(&self) -> bool {
        match self {
            owned::Token::Boolean(_) => true,
            _ => false,
        }
    }
    fn is_boolean_true(&self) -> bool {
        match self {
            owned::Token::Boolean(ref b) => b.into(),
            _ => false,
        }
    }
    fn is_boolean_false(&self) -> bool {
        match self {
            owned::Token::Boolean(ref b) => {
                let b: bool = b.into();
                !b
            }
            _ => false,
        }
    }
    fn is_eof(&self) -> bool {
        self == &owned::Token::EoF
    }
    fn is_ident(&self) -> bool {
        match self {
            owned::Token::Ident(_) => true,
            _ => false,
        }
    }
    fn is_keyword(&self) -> bool {
        match self {
            owned::Token::Keyword(_) => true,
            _ => false,
        }
    }
    fn is_strict_reserved(&self) -> bool {
        match self {
            owned::Token::Keyword(ref k) => k.is_strict_reserved(),
            _ => false,
        }
    }
    fn is_restricted(&self) -> bool {
        match self {
            owned::Token::Ident(ref i) => i == "arguments" || i == "eval",
            _ => false,
        }
    }
    fn is_null(&self) -> bool {
        self == &owned::Token::Null
    }

    fn is_number(&self) -> bool {
        if let owned::Token::Number(ref _n) = self {
            true
        } else {
            false
        }
    }
    fn is_hex_literal(&self) -> bool {
        match self {
            owned::Token::Number(ref n) => n.is_hex(),
            _ => false,
        }
    }
    fn is_bin_literal(&self) -> bool {
        match self {
            owned::Token::Number(ref n) => n.is_bin(),
            _ => false,
        }
    }
    fn is_oct_literal(&self) -> bool {
        match self {
            owned::Token::Number(ref n) => n.is_oct(),
            _ => false,
        }
    }
    fn is_punct(&self) -> bool {
        match self {
            owned::Token::Punct(_) => true,
            _ => false,
        }
    }
    fn is_string(&self) -> bool {
        if let owned::Token::String(ref _s) = self {
            true
        } else {
            false
        }
    }
    fn is_double_quoted_string(&self) -> bool {
        match self {
            owned::Token::String(ref s) => match s {
                owned::StringLit::Double(_) => true,
                _ => false,
            },
            _ => false,
        }
    }
    fn is_single_quoted_string(&self) -> bool {
        match self {
            owned::Token::String(ref s) => match s {
                owned::StringLit::Single(_) => true,
                _ => false,
            },
            _ => false,
        }
    }
    fn is_regex(&self) -> bool {
        match self {
            owned::Token::RegEx(_) => true,
            _ => false,
        }
    }
    fn is_template(&self) -> bool {
        match self {
            owned::Token::Template(_) => true,
            _ => false,
        }
    }
    fn is_template_no_sub(&self) -> bool {
        match self {
            owned::Token::Template(ref s) => s.is_no_sub(),
            _ => false,
        }
    }
    fn is_template_head(&self) -> bool {
        match self {
            owned::Token::Template(ref s) => s.is_head() || s.is_no_sub(),
            _ => false,
        }
    }
    fn is_template_body(&self) -> bool {
        match self {
            owned::Token::Template(ref s) => s.is_middle(),
            _ => false,
        }
    }
    fn is_template_tail(&self) -> bool {
        match self {
            owned::Token::Template(ref s) => s.is_tail() || s.is_no_sub(),
            _ => false,
        }
    }
    fn is_literal(&self) -> bool {
        match self {
            owned::Token::Boolean(_) => true,
            owned::Token::String(_) => true,
            owned::Token::Null => true,
            owned::Token::Number(_) => true,
            owned::Token::RegEx(_) => true,
            owned::Token::Template(_) => true,
            _ => false,
        }
    }
    fn is_comment(&self) -> bool {
        match self {
            owned::Token::Comment(_) => true,
            _ => false,
        }
    }
    fn is_multi_line_comment(&self) -> bool {
        match self {
            owned::Token::Comment(ref t) => t.kind == CommentKind::Multi,
            _ => false,
        }
    }

    fn is_single_line_comment(&self) -> bool {
        match self {
            owned::Token::Comment(ref t) => t.kind == CommentKind::Single,
            _ => false,
        }
    }
    fn matches_boolean(&self, b: BooleanLiteral) -> bool {
        self == &owned::Token::Boolean(b)
    }
    fn matches_boolean_str(&self, b: &str) -> bool {
        match self {
            owned::Token::Boolean(ref lit) => match (lit, b) {
                (&BooleanLiteral::True, "true") | (&BooleanLiteral::False, "false") => true,
                _ => false,
            },
            _ => false,
        }
    }
    fn matches_ident_str(&self, name: &str) -> bool {
        match self {
            owned::Token::Ident(ref i) => i.matches(name),
            _ => false,
        }
    }
    fn matches_keyword(&self, keyword: Keyword) -> bool {
        self == &owned::Token::Keyword(keyword)
    }
    fn matches_keyword_str(&self, name: &str) -> bool {
        match self {
            owned::Token::Keyword(ref k) => k.as_str() == name,
            _ => false,
        }
    }

    fn matches_number_str(&self, number: &str) -> bool {
        match self {
            owned::Token::Number(n) => n == number,
            _ => false,
        }
    }
    fn matches_punct(&self, p: Punct) -> bool {
        match self {
            owned::Token::Punct(inner) => inner == &p,
            _ => false,
        }
    }
    fn matches_punct_str(&self, s: &str) -> bool {
        match self {
            owned::Token::Punct(ref p) => p.matches_str(s),
            _ => false,
        }
    }

    fn matches_comment_str(&self, comment: &str) -> bool {
        match self {
            owned::Token::Comment(ref t) => t.content == comment,
            _ => false,
        }
    }

    fn matches_string_content(&self, content: &str) -> bool {
        match self {
            owned::Token::String(ref lit) => match lit {
                owned::StringLit::Single(ref s) => content == s,
                owned::StringLit::Double(ref s) => content == s,
            },
            _ => false,
        }
    }
}

impl<'a> Token for refs::Token<'a> {
    fn is_boolean(&self) -> bool {
        match self {
            refs::Token::Boolean(_) => true,
            _ => false,
        }
    }
    fn is_boolean_true(&self) -> bool {
        match self {
            refs::Token::Boolean(ref b) => b.into(),
            _ => false,
        }
    }
    fn is_boolean_false(&self) -> bool {
        match self {
            refs::Token::Boolean(ref b) => {
                let b: bool = b.into();
                !b
            }
            _ => false,
        }
    }
    fn is_eof(&self) -> bool {
        self == &refs::Token::EoF
    }
    fn is_ident(&self) -> bool {
        match self {
            refs::Token::Ident(_) => true,
            _ => false,
        }
    }
    fn is_keyword(&self) -> bool {
        match self {
            refs::Token::Keyword(_) => true,
            _ => false,
        }
    }
    fn is_strict_reserved(&self) -> bool {
        match self {
            refs::Token::Keyword(ref k) => k.is_strict_reserved(),
            _ => false,
        }
    }
    fn is_restricted(&self) -> bool {
        match self {
            refs::Token::Ident(ref i) => i == "arguments" || i == "eval",
            _ => false,
        }
    }
    fn is_null(&self) -> bool {
        self == &refs::Token::Null
    }

    fn is_number(&self) -> bool {
        if let refs::Token::Number(ref _n) = self {
            true
        } else {
            false
        }
    }
    fn is_hex_literal(&self) -> bool {
        match self {
            refs::Token::Number(ref n) => n.is_hex(),
            _ => false,
        }
    }
    fn is_bin_literal(&self) -> bool {
        match self {
            refs::Token::Number(ref n) => n.is_bin(),
            _ => false,
        }
    }
    fn is_oct_literal(&self) -> bool {
        match self {
            refs::Token::Number(ref n) => n.is_oct(),
            _ => false,
        }
    }
    fn is_punct(&self) -> bool {
        match self {
            refs::Token::Punct(_) => true,
            _ => false,
        }
    }
    fn is_string(&self) -> bool {
        if let refs::Token::String(ref _s) = self {
            true
        } else {
            false
        }
    }
    fn is_double_quoted_string(&self) -> bool {
        match self {
            refs::Token::String(ref s) => match s {
                refs::StringLit::Double(_) => true,
                _ => false,
            },
            _ => false,
        }
    }
    fn is_single_quoted_string(&self) -> bool {
        match self {
            refs::Token::String(ref s) => match s {
                refs::StringLit::Single(_) => true,
                _ => false,
            },
            _ => false,
        }
    }
    fn is_regex(&self) -> bool {
        match self {
            refs::Token::RegEx(_) => true,
            _ => false,
        }
    }
    fn is_template(&self) -> bool {
        match self {
            refs::Token::Template(_) => true,
            _ => false,
        }
    }
    fn is_template_no_sub(&self) -> bool {
        match self {
            refs::Token::Template(ref s) => s.is_no_sub(),
            _ => false,
        }
    }
    fn is_template_head(&self) -> bool {
        match self {
            refs::Token::Template(ref s) => s.is_head() || s.is_no_sub(),
            _ => false,
        }
    }
    fn is_template_body(&self) -> bool {
        match self {
            refs::Token::Template(ref s) => s.is_middle(),
            _ => false,
        }
    }
    fn is_template_tail(&self) -> bool {
        match self {
            refs::Token::Template(ref s) => s.is_tail() || s.is_no_sub(),
            _ => false,
        }
    }
    fn is_literal(&self) -> bool {
        match self {
            refs::Token::Boolean(_) => true,
            refs::Token::String(_) => true,
            refs::Token::Null => true,
            refs::Token::Number(_) => true,
            refs::Token::RegEx(_) => true,
            refs::Token::Template(_) => true,
            _ => false,
        }
    }
    fn is_comment(&self) -> bool {
        match self {
            refs::Token::Comment(_) => true,
            _ => false,
        }
    }
    fn is_multi_line_comment(&self) -> bool {
        match self {
            refs::Token::Comment(ref t) => t.kind == CommentKind::Multi,
            _ => false,
        }
    }

    fn is_single_line_comment(&self) -> bool {
        match self {
            refs::Token::Comment(ref t) => t.kind == CommentKind::Single,
            _ => false,
        }
    }
    fn matches_boolean(&self, b: BooleanLiteral) -> bool {
        self == &refs::Token::Boolean(b)
    }
    fn matches_boolean_str(&self, b: &str) -> bool {
        match self {
            refs::Token::Boolean(ref lit) => match (lit, b) {
                (&BooleanLiteral::True, "true") | (&BooleanLiteral::False, "false") => true,
                _ => false,
            },
            _ => false,
        }
    }
    fn matches_ident_str(&self, name: &str) -> bool {
        match self {
            refs::Token::Ident(i) => i.matches(name),
            _ => false,
        }
    }
    fn matches_keyword(&self, keyword: Keyword) -> bool {
        self == &refs::Token::Keyword(keyword)
    }
    fn matches_keyword_str(&self, name: &str) -> bool {
        match self {
            refs::Token::Keyword(n) => n.as_str() == name,
            _ => false,
        }
    }
    fn matches_number_str(&self, number: &str) -> bool {
        match self {
            refs::Token::Number(n) => n == number,
            _ => false,
        }
    }
    fn matches_punct(&self, p: Punct) -> bool {
        self == &refs::Token::Punct(p)
    }

    fn matches_punct_str(&self, s: &str) -> bool {
        match self {
            refs::Token::Punct(ref p) => p.matches_str(s),
            _ => false,
        }
    }

    fn matches_comment_str(&self, comment: &str) -> bool {
        match self {
            refs::Token::Comment(ref t) => t.content == comment,
            _ => false,
        }
    }

    fn matches_string_content(&self, content: &str) -> bool {
        match self {
            refs::Token::String(ref lit) => match lit {
                refs::StringLit::Single(s) => content == *s,
                refs::StringLit::Double(s) => content == *s,
            },
            _ => false,
        }
    }
}
