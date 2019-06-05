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
        
    fn is_numeric(&self) -> bool;
        
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
                
    fn matches_numeric_str(&self, number: &str) -> bool;
        
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

impl<'a> From<&'a str> for BooleanLiteral {
    /// Create a BooleanLiteral from raw text
    ///
    /// panics if argument is not `true` or `false`
    fn from(s: &'a str) -> Self {
        if s == "true" {
            BooleanLiteral::True
        } else if s == "false" {
            BooleanLiteral::False
        } else {
            panic!(r#"BooleanLiteral can only be created for "true" or "false"."#)
        }
    }
}

impl From<String> for BooleanLiteral {
    /// Create a BooleanLiteral from raw text
    ///
    /// panics if argument is not `true` or `false`
    fn from(s: String) -> Self {
        BooleanLiteral::from(s.as_str())
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
    And,
    Assign,
    Asterisk,
    BitwiseNot,
    Caret,
    CloseBrace,
    CloseBracket,
    CloseParen,
    Colon,
    Comma,
    ForwardSlash,
    GreaterThan,
    LessThan,
    Minus,
    Modulo,
    Not,
    OpenBrace,
    OpenBracket,
    OpenParen,
    Period,
    Pipe,
    Plus,
    QuestionMark,
    SemiColon,
    Spread,
    UnsignedRightShiftAssign,
    StrictEquals,
    StrictNotEquals,
    UnsignedRightShift,
    LeftShiftAssign,
    RightShiftAssign,
    ExponentAssign,
    LogicalAnd,
    LogicalOr,
    Equal,
    NotEqual,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    Increment,
    Decrement,
    LeftShift,
    RightShift,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXOrAssign,
    ModuloAssign,
    FatArrow,
    GreaterThanEqual,
    LessThanEqual,
    Exponent,
    Private,
}

impl Punct {
    fn matches_str(&self, s: &str) -> bool {
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
            Punct::BitwiseNot => "~" == s,
            Punct::GreaterThan => ">" == s,
            Punct::LessThan => "<" == s,
            Punct::Assign => "=" == s,
            Punct::Not => "!" == s,
            Punct::Plus => "+" == s,
            Punct::Minus => "-" == s,
            Punct::Asterisk => "*" == s,
            Punct::Modulo => "%" == s,
            Punct::Pipe => "|" == s,
            Punct::And => "&" == s,
            Punct::Caret => "^" == s,
            Punct::ForwardSlash => "/" == s,
            Punct::UnsignedRightShiftAssign => ">>>=" == s,
            Punct::Spread => "..." == s,
            Punct::StrictEquals => "===" == s,
            Punct::StrictNotEquals => "!==" == s,
            Punct::UnsignedRightShift => ">>>" == s,
            Punct::LeftShiftAssign => "<<=" == s,
            Punct::RightShiftAssign => ">>=" == s,
            Punct::ExponentAssign => "**=" == s,
            Punct::LogicalAnd => "&&" == s,
            Punct::LogicalOr => "||" == s,
            Punct::Equal => "==" == s,
            Punct::NotEqual => "!=" == s,
            Punct::AddAssign => "+=" == s,
            Punct::SubtractAssign => "-=" == s,
            Punct::MultiplyAssign => "*=" == s,
            Punct::DivideAssign => "/=" == s,
            Punct::Increment => "++" == s,
            Punct::Decrement => "--" == s,
            Punct::LeftShift => "<<" == s,
            Punct::RightShift => ">>" == s,
            Punct::BitwiseAndAssign => "&=" == s,
            Punct::BitwiseOrAssign => "|=" == s,
            Punct::BitwiseXOrAssign => "^=" == s,
            Punct::ModuloAssign => "%=" == s,
            Punct::FatArrow => "=>" == s,
            Punct::GreaterThanEqual => ">=" == s,
            Punct::LessThanEqual => "<=" == s,
            Punct::Exponent => "**" == s,
            Punct::Private => "#" == s,
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
            Punct::BitwiseNot => "~",
            Punct::GreaterThan => ">",
            Punct::LessThan => "<",
            Punct::Assign => "=",
            Punct::Not => "!",
            Punct::Plus => "+",
            Punct::Minus => "-",
            Punct::Asterisk => "*",
            Punct::Modulo => "%",
            Punct::Pipe => "|",
            Punct::And => "&",
            Punct::Caret => "^",
            Punct::ForwardSlash => "/",
            Punct::UnsignedRightShiftAssign => ">>>=",
            Punct::Spread => "...",
            Punct::StrictEquals => "===",
            Punct::StrictNotEquals => "!==",
            Punct::UnsignedRightShift => ">>>",
            Punct::LeftShiftAssign => "<<=",
            Punct::RightShiftAssign => ">>=",
            Punct::ExponentAssign => "**=",
            Punct::LogicalAnd => "&&",
            Punct::LogicalOr => "||",
            Punct::Equal => "==",
            Punct::NotEqual => "!=",
            Punct::AddAssign => "+=",
            Punct::SubtractAssign => "-=",
            Punct::MultiplyAssign => "*=",
            Punct::DivideAssign => "/=",
            Punct::Increment => "++",
            Punct::Decrement => "--",
            Punct::LeftShift => "<<",
            Punct::RightShift => ">>",
            Punct::BitwiseAndAssign => "&=",
            Punct::BitwiseOrAssign => "|=",
            Punct::BitwiseXOrAssign => "^=",
            Punct::ModuloAssign => "%=",
            Punct::FatArrow => "=>",
            Punct::GreaterThanEqual => ">=",
            Punct::LessThanEqual => "<=",
            Punct::Exponent => "**",
            Punct::Private => "#",
        }.into()
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

impl<'a> From<&'a str> for Keyword {
    /// convert a &str into a Keyword
    ///
    /// panics if invalid keyword
    fn from(s: &'a str) -> Self {
        match s {
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
            _ => panic!("Unknown Keyword, `{}`", s),
        }
    }
}

impl From<String> for Keyword {
    /// converts from a String to a Keyword
    ///
    /// panics if an invalid keyword
    fn from(s: String) -> Self {
        Self::from(s.as_str())
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
    fn is_future_reserved(self) -> bool {
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
    fn is_strict_reserved(self) -> bool {
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
    fn is_reserved(self) -> bool {
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

    fn as_str(&self) -> &str {
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

    fn is_numeric(&self) -> bool {
        if let owned::Token::Numeric(ref _n) = self {
            true
        } else {
            false
        }
    }
    fn is_hex_literal(&self) -> bool {
        match self {
            owned::Token::Numeric(ref n) => n.is_hex(),
            _ => false,
        }
    }
    fn is_bin_literal(&self) -> bool {
        match self {
            owned::Token::Numeric(ref n) => n.is_bin(),
            _ => false,
        }
    }
    fn is_oct_literal(&self) -> bool {
        match self {
            owned::Token::Numeric(ref n) => n.is_oct(),
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
            owned::Token::Numeric(_) => true,
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

    fn matches_numeric_str(&self, number: &str) -> bool {
        match self {
            owned::Token::Numeric(n) => n == number,
            _ => false
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

    fn is_numeric(&self) -> bool {
        if let refs::Token::Numeric(ref _n) = self {
            true
        } else {
            false
        }
    }
    fn is_hex_literal(&self) -> bool {
        match self {
            refs::Token::Numeric(ref n) => n.is_hex(),
            _ => false,
        }
    }
    fn is_bin_literal(&self) -> bool {
        match self {
            refs::Token::Numeric(ref n) => n.is_bin(),
            _ => false,
        }
    }
    fn is_oct_literal(&self) -> bool {
        match self {
            refs::Token::Numeric(ref n) => n.is_oct(),
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
            refs::Token::Numeric(_) => true,
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
    fn matches_numeric_str(&self, number: &str) -> bool {
        match self {
            refs::Token::Numeric(n) => n == number,
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