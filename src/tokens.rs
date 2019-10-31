pub mod prelude {
    pub use super::{
        Boolean, Comment, CommentExt, Ident, IdentExt, Keyword, Number, NumberExt, Punct, RegEx,
        RegExExt, StringLit, StringLitExt, Template, TemplateExt, Token, TokenExt,
    };
}

#[derive(PartialEq, Clone, Debug)]
/// The representation of any single
/// JS part
pub enum Token<T> {
    /// `true` of `false`
    Boolean(Boolean),
    /// The end of the file
    EoF,
    /// An identifier this will be either a variable name
    /// or a function/method name
    Ident(Ident<T>),
    /// A word that has been reserved to not be used as an identifier
    Keyword(Keyword<T>),
    /// A `null` literal value
    Null,
    /// A number, this includes integers (`1`), decimals (`0.1`),
    /// hex (`0x8f`), binary (`0b010011010`), and octal (`0o273`)
    Number(Number<T>),
    /// A punctuation mark, this includes all mathematical operators
    /// logical operators and general syntax punctuation
    Punct(Punct),
    /// A string literal, either double or single quoted, the associated
    /// value will be the unquoted string
    String(StringLit<T>),
    /// A regular expression literal.
    /// ```js
    /// let regex = /[a-zA-Z]+/g;
    /// ```
    RegEx(RegEx<T>),
    /// The string parts of a template string
    Template(Template<T>),
    /// A comment, the associated value will contain the raw comment
    /// This will capture inline comments `// I am an inline comment`,
    /// multi-line comments, HTML-style comments and Unix hashbangs.
    /// ```js
    /// #!/usr/bin/env node
    /// /*multi lines
    /// * comments
    /// */
    /// ```
    Comment(Comment<T>),
}

/// Extension methods for
/// implementing allowing Token
/// to work with both &str and String
pub trait TokenExt {
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

    fn matches_boolean(&self, b: Boolean) -> bool;

    fn matches_boolean_str(&self, b: &str) -> bool;

    fn matches_ident_str(&self, name: &str) -> bool;

    fn matches_keyword<T>(&self, keyword: Keyword<T>) -> bool;

    fn matches_keyword_str(&self, name: &str) -> bool;

    fn matches_number_str(&self, number: &str) -> bool;

    fn matches_punct(&self, p: Punct) -> bool;

    fn matches_punct_str(&self, s: &str) -> bool;

    fn matches_comment_str(&self, comment: &str) -> bool;

    fn matches_string_content(&self, content: &str) -> bool;
}

impl<'a> ToString for Token<&'a str> {
    fn to_string(&self) -> String {
        match self {
            Token::Boolean(ref b) => b.to_string(),
            Token::Comment(ref c) => c.to_string(),
            Token::EoF => String::new(),
            Token::Ident(ref i) => i.to_string(),
            Token::Keyword(ref k) => k.to_string(),
            Token::Null => "null".to_string(),
            Token::Number(ref n) => n.to_string(),
            Token::Punct(ref p) => p.to_string(),
            Token::RegEx(ref r) => r.to_string(),
            Token::String(ref s) => s.to_string(),
            Token::Template(ref t) => t.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
/// An identifier
pub struct Ident<T>(T);
/// Extention methods for allowing Ident
/// to work with both &str and String
pub trait IdentExt<T>
where
    T: ?Sized,
{
    fn matches(&self, other: &T) -> bool;
    fn as_str(&self) -> &str;
}

impl<'a> PartialEq<str> for Ident<&'a str> {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

impl<'a> IdentExt<str> for Ident<&'a str> {
    fn matches(&self, other: &str) -> bool {
        self.0 == other
    }

    fn as_str(&self) -> &str {
        self.0
    }
}

impl IdentExt<String> for Ident<String> {
    fn matches(&self, other: &String) -> bool {
        &self.0 == other
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'a> From<&'a str> for Ident<&'a str> {
    fn from(s: &'a str) -> Self {
        Ident(s)
    }
}

impl<T> ToString for Ident<T>
where
    T: ToString,
{
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<T> Into<String> for Ident<T>
where
    T: ToString,
{
    fn into(self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, PartialEq, Clone)]
/// A comment, effectively should be treated
/// as white space. There are 3 kinds of comments
/// according to the specification.
///
/// - Single line comments: //comment
/// - Multi line comments: /* comment */
/// - HTML comments: <!-- comment --> plus more!
pub struct Comment<T> {
    pub kind: CommentKind,
    pub content: T,
    pub tail_content: Option<T>,
}
/// Extension methods for comment
/// to work with both &str and String
pub trait CommentExt<T> {
    fn from_parts(content: T, kind: CommentKind, tail_content: Option<T>) -> Comment<T>;
    fn new_single_line(content: T) -> Comment<T>;
    fn new_multi_line(content: T) -> Comment<T>;
    fn new_html(content: T, tail_content: Option<T>) -> Comment<T>;
    fn new_html_no_tail(content: T) -> Comment<T>;
    fn new_html_with_tail(content: T, tail: T) -> Comment<T>;
    fn new_hashbang(content: T) -> Comment<T>;
    fn is_multi_line(&self) -> bool;
    fn is_single_line(&self) -> bool;
    fn is_html(&self) -> bool;
    fn is_hashbang(&self) -> bool;
}

impl<'a> CommentExt<&'a str> for Comment<&'a str> {
    fn from_parts(content: &'a str, kind: CommentKind, tail_content: Option<&'a str>) -> Self {
        Comment {
            content,
            kind,
            tail_content,
        }
    }

    fn new_single_line(content: &'a str) -> Self {
        Comment::from_parts(content, CommentKind::Single, None)
    }

    fn new_multi_line(content: &'a str) -> Self {
        Comment::from_parts(content, CommentKind::Multi, None)
    }

    fn new_html(content: &'a str, tail_content: Option<&'a str>) -> Self {
        Comment::from_parts(content, CommentKind::Html, tail_content)
    }

    fn new_html_no_tail(content: &'a str) -> Self {
        Comment::new_html(content, None)
    }

    fn new_html_with_tail(content: &'a str, tail: &'a str) -> Self {
        Comment::new_html(content, Some(tail))
    }

    fn new_hashbang(content: &'a str) -> Self {
        Comment::from_parts(content, CommentKind::Hashbang, None)
    }

    fn is_multi_line(&self) -> bool {
        self.kind == CommentKind::Multi
    }

    fn is_single_line(&self) -> bool {
        self.kind == CommentKind::Single
    }

    fn is_html(&self) -> bool {
        self.kind == CommentKind::Multi
    }

    fn is_hashbang(&self) -> bool {
        self.kind == CommentKind::Hashbang
    }
}
impl CommentExt<String> for Comment<String> {
    fn from_parts(content: String, kind: CommentKind, tail_content: Option<String>) -> Self {
        Comment {
            content: content,
            kind,
            tail_content: tail_content,
        }
    }

    fn new_single_line(content: String) -> Self {
        Comment::from_parts(content, CommentKind::Single, None)
    }

    fn new_multi_line(content: String) -> Self {
        Comment::from_parts(content, CommentKind::Multi, None)
    }

    fn new_html(content: String, tail_content: Option<String>) -> Self {
        Comment::from_parts(content, CommentKind::Html, tail_content)
    }

    fn new_html_no_tail(content: String) -> Self {
        Comment::new_html(content, None)
    }

    fn new_html_with_tail(content: String, tail: String) -> Self {
        Comment::new_html(content, Some(tail))
    }

    fn new_hashbang(content: String) -> Self {
        Comment::from_parts(content, CommentKind::Hashbang, None)
    }

    fn is_multi_line(&self) -> bool {
        self.kind == CommentKind::Multi
    }

    fn is_single_line(&self) -> bool {
        self.kind == CommentKind::Single
    }

    fn is_html(&self) -> bool {
        self.kind == CommentKind::Multi
    }

    fn is_hashbang(&self) -> bool {
        self.kind == CommentKind::Hashbang
    }
}

impl ToString for Comment<String> {
    fn to_string(&self) -> String {
        match self.kind {
            CommentKind::Single => format!("//{}", self.content),
            CommentKind::Multi => format!("/*{}*/", self.content),
            CommentKind::Html => format!("<!--{}-->", self.content),
            CommentKind::Hashbang => format!("#!{}", self.content),
        }
    }
}
impl ToString for Comment<&str> {
    fn to_string(&self) -> String {
        match self.kind {
            CommentKind::Single => format!("//{}", self.content),
            CommentKind::Multi => format!("/*{}*/", self.content),
            CommentKind::Html => format!("<!--{}-->", self.content),
            CommentKind::Hashbang => format!("#!{}", self.content),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
/// A JS number literal. There are 4 kinds of number
/// literals allowed in JS.
///
/// - Decimal Literals - This includes integers and decimals with
///     optional exponent notation
/// - Hexadecimal Literals - These begin with 0x and consist of numbers
///     0-9 and letters A-F (case insensitive)
/// - Octal Literals - These being with 0o and consist of numbers
///     0-7
/// - Binary Literals - These begin with 0b and consist of numbers 0 and 1
pub struct Number<T>(T);

/// Extension methods for allowing Number
/// to work with both &str and String
pub trait NumberExt {
    fn kind(&self) -> NumberKind;
    fn is_hex(&self) -> bool;
    fn is_bin(&self) -> bool;
    fn is_oct(&self) -> bool;
    fn is_dec(&self) -> bool;
    fn has_exponent(&self) -> bool;
    fn is_big_int(&self) -> bool;
}

impl<'a> NumberExt for Number<&'a str> {
    fn kind(&self) -> NumberKind {
        if self.0.starts_with("0x") {
            NumberKind::Hex
        } else if self.0.starts_with("0b") {
            NumberKind::Bin
        } else if self.0.starts_with("0o") {
            NumberKind::Oct
        } else if self.0.ends_with("n") {
            NumberKind::BigInt
        } else {
            NumberKind::Dec
        }
    }

    fn is_hex(&self) -> bool {
        self.kind() == NumberKind::Hex
    }
    fn is_bin(&self) -> bool {
        self.kind() == NumberKind::Bin
    }
    fn is_oct(&self) -> bool {
        self.kind() == NumberKind::Oct
    }
    fn is_dec(&self) -> bool {
        self.kind() == NumberKind::Dec
    }
    fn has_exponent(&self) -> bool {
        match self.kind() {
            NumberKind::Dec => self.0.contains(|c| c == 'e' || c == 'E'),
            _ => false,
        }
    }
    fn is_big_int(&self) -> bool {
        self.kind() == NumberKind::BigInt
    }
}
impl NumberExt for Number<String> {
    fn kind(&self) -> NumberKind {
        if self.0.starts_with("0x") {
            NumberKind::Hex
        } else if self.0.starts_with("0b") {
            NumberKind::Bin
        } else if self.0.starts_with("0o") {
            NumberKind::Oct
        } else if self.0.ends_with("n") {
            NumberKind::BigInt
        } else {
            NumberKind::Dec
        }
    }

    fn is_hex(&self) -> bool {
        self.kind() == NumberKind::Hex
    }
    fn is_bin(&self) -> bool {
        self.kind() == NumberKind::Bin
    }
    fn is_oct(&self) -> bool {
        self.kind() == NumberKind::Oct
    }
    fn is_dec(&self) -> bool {
        self.kind() == NumberKind::Dec
    }
    fn has_exponent(&self) -> bool {
        match self.kind() {
            NumberKind::Dec => self.0.contains(|c| c == 'e' || c == 'E'),
            _ => false,
        }
    }
    fn is_big_int(&self) -> bool {
        self.kind() == NumberKind::BigInt
    }
}

impl<'a> From<&'a str> for Number<&'a str> {
    fn from(s: &'a str) -> Self {
        Number(s)
    }
}

impl<'a> ToString for Number<&'a str> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<'a> PartialEq<str> for Number<&'a str> {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

#[derive(Debug, PartialEq, Clone)]
/// A Regular Expression Literal
///
/// These being with a `/` and the
/// body ends with another `/`
/// optionally a series of one letter
/// flags can be included after the `/`
pub struct RegEx<T> {
    pub body: T,
    pub flags: Option<T>,
}
/// Extension methods for allowing RegEx
/// to work with both &str and String
pub trait RegExExt<T> {
    fn from_parts(body: T, flags: Option<T>) -> RegEx<T>;
}

impl<'a> RegExExt<&'a str> for RegEx<&'a str> {
    fn from_parts(body: &'a str, flags: Option<&'a str>) -> Self {
        let flags = if let Some(flags) = flags {
            if flags == "" {
                None
            } else {
                Some(flags)
            }
        } else {
            None
        };
        RegEx { body, flags }
    }
}
impl RegExExt<String> for RegEx<String> {
    fn from_parts(body: String, flags: Option<String>) -> Self {
        let flags = if let Some(flags) = flags {
            if flags == "" {
                None
            } else {
                Some(flags)
            }
        } else {
            None
        };
        RegEx { body, flags }
    }
}

#[derive(Debug, PartialEq, Clone)]
/// A single or double quoted string
/// literal
pub enum StringLit<T> {
    Single(T),
    Double(T),
}

impl<'a> ToString for RegEx<&'a str> {
    fn to_string(&self) -> String {
        let f = if let Some(ref f) = self.flags {
            f.to_string()
        } else {
            String::new()
        };
        format!("/{}/{}", self.body, f)
    }
}
/// Extension methods for allowing StringLit
/// to work with both &str and String
pub trait StringLitExt<T> {
    fn single(content: T) -> StringLit<T>;
    fn double(content: T) -> StringLit<T>;
    fn is_single(&self) -> bool;
    fn is_double(&self) -> bool;
    fn no_quote(&self) -> T;
}

impl<T> ToString for StringLit<T>
where
    T: ::core::fmt::Display,
{
    fn to_string(&self) -> String {
        match self {
            StringLit::Single(ref s) => format!(r#"'{}'"#, s),
            StringLit::Double(ref s) => format!(r#""{}""#, s),
        }
    }
}

impl<'a> StringLitExt<&'a str> for StringLit<&'a str> {
    fn single(content: &'a str) -> Self {
        StringLit::Single(content)
    }
    fn double(content: &'a str) -> Self {
        StringLit::Double(content)
    }
    fn is_single(&self) -> bool {
        match self {
            StringLit::Single(_) => true,
            _ => false,
        }
    }
    fn is_double(&self) -> bool {
        match self {
            StringLit::Double(_) => true,
            _ => false,
        }
    }
    fn no_quote(&self) -> &'a str {
        match self {
            StringLit::Single(ref inner) => inner,
            StringLit::Double(ref inner) => inner,
        }
    }
}
impl StringLitExt<String> for StringLit<String> {
    fn single(content: String) -> Self {
        StringLit::Single(content)
    }
    fn double(content: String) -> Self {
        StringLit::Double(content)
    }
    fn is_single(&self) -> bool {
        match self {
            StringLit::Single(_) => true,
            _ => false,
        }
    }
    fn is_double(&self) -> bool {
        match self {
            StringLit::Double(_) => true,
            _ => false,
        }
    }
    fn no_quote(&self) -> String {
        match self {
            StringLit::Single(ref inner) => inner.clone(),
            StringLit::Double(ref inner) => inner.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
/// A template string
///
/// These include strings that are wrapped in back ticks (`)
/// which allows for interpolating any js expression between `${`
/// and `}`
pub enum Template<T> {
    NoSub(T),
    Head(T),
    Middle(T),
    Tail(T),
}
/// Extension methods for allowing Template
/// to work with both &str and String
pub trait TemplateExt<T> {
    fn no_sub_template(content: T) -> Template<T>;
    fn template_head(content: T) -> Template<T>;
    fn template_middle(content: T) -> Template<T>;
    fn template_tail(content: T) -> Template<T>;
    fn is_head(&self) -> bool;
    fn is_middle(&self) -> bool;
    fn is_tail(&self) -> bool;
    fn is_no_sub(&self) -> bool;
}

impl<'a> TemplateExt<&'a str> for Template<&'a str> {
    fn no_sub_template(content: &'a str) -> Self {
        Template::NoSub(content)
    }
    fn template_head(content: &'a str) -> Self {
        Template::Head(content)
    }
    fn template_middle(content: &'a str) -> Self {
        Template::Middle(content)
    }
    fn template_tail(content: &'a str) -> Self {
        Template::Tail(content)
    }
    fn is_head(&self) -> bool {
        match self {
            Template::Head(_) => true,
            _ => false,
        }
    }
    fn is_middle(&self) -> bool {
        match self {
            Template::Middle(_) => true,
            _ => false,
        }
    }
    fn is_tail(&self) -> bool {
        match self {
            Template::Tail(_) => true,
            _ => false,
        }
    }
    fn is_no_sub(&self) -> bool {
        match self {
            Template::NoSub(_) => true,
            _ => false,
        }
    }
}
impl TemplateExt<String> for Template<String> {
    fn no_sub_template(content: String) -> Self {
        Template::NoSub(content)
    }
    fn template_head(content: String) -> Self {
        Template::Head(content)
    }
    fn template_middle(content: String) -> Self {
        Template::Middle(content)
    }
    fn template_tail(content: String) -> Self {
        Template::Tail(content)
    }
    fn is_head(&self) -> bool {
        match self {
            Template::Head(_) => true,
            _ => false,
        }
    }
    fn is_middle(&self) -> bool {
        match self {
            Template::Middle(_) => true,
            _ => false,
        }
    }
    fn is_tail(&self) -> bool {
        match self {
            Template::Tail(_) => true,
            _ => false,
        }
    }
    fn is_no_sub(&self) -> bool {
        match self {
            Template::NoSub(_) => true,
            _ => false,
        }
    }
}

impl<T> ToString for Template<T>
where
    T: ::core::fmt::Display,
{
    fn to_string(&self) -> String {
        match self {
            Template::NoSub(ref c) => format!("`{}`", c),
            Template::Head(ref c) => format!("`{}${{", c),
            Template::Middle(ref c) => format!("}}{}${{", c),
            Template::Tail(ref c) => format!("}}{}`", c),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// The tokenized representation of `true` or `false`
pub enum Boolean {
    True,
    False,
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

#[derive(Debug, PartialEq, Clone, Copy)]
/// The 5 kinds of numbers
pub enum NumberKind {
    Dec,
    Hex,
    Bin,
    Oct,
    BigInt,
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// All available punctuation
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
/// The 4 kinds of comments
pub enum CommentKind {
    Single,
    Multi,
    Html,
    Hashbang,
}

#[derive(Debug, Clone)]
/// A JS Keyword
///
/// # Standard
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
pub enum Keyword<T> {
    Await(T),
    Break(T),
    Case(T),
    Catch(T),
    Class(T),
    Const(T),
    Continue(T),
    Debugger(T),
    Default(T),
    Delete(T),
    Do(T),
    Else(T),
    Enum(T),
    Export(T),
    Extends(T),
    Finally(T),
    For(T),
    Function(T),
    If(T),
    Implements(T),
    Import(T),
    In(T),
    InstanceOf(T),
    Interface(T),
    Let(T),
    New(T),
    Package(T),
    Private(T),
    Protected(T),
    Public(T),
    Return(T),
    Static(T),
    Super(T),
    Switch(T),
    This(T),
    Throw(T),
    Try(T),
    TypeOf(T),
    Var(T),
    Void(T),
    While(T),
    With(T),
    Yield(T),
}

impl<T, U> PartialEq<Keyword<T>> for Keyword<U> {
    fn eq(&self, other: &Keyword<T>) -> bool {
        use Keyword::*;
        match (self, other) {
              (Await(_), Await(_))
            | (Break(_), Break(_))
            | (Case(_), Case(_))
            | (Catch(_), Catch(_))
            | (Class(_), Class(_))
            | (Const(_), Const(_))
            | (Continue(_), Continue(_))
            | (Debugger(_), Debugger(_))
            | (Default(_), Default(_))
            | (Delete(_), Delete(_))
            | (Do(_), Do(_))
            | (Else(_), Else(_))
            | (Enum(_), Enum(_))
            | (Export(_), Export(_))
            | (Extends(_), Extends(_))
            | (Finally(_), Finally(_))
            | (For(_), For(_))
            | (Function(_), Function(_))
            | (If(_), If(_))
            | (Implements(_), Implements(_))
            | (Import(_), Import(_))
            | (In(_), In(_))
            | (InstanceOf(_), InstanceOf(_))
            | (Interface(_), Interface(_))
            | (Let(_), Let(_))
            | (New(_), New(_))
            | (Package(_), Package(_))
            | (Private(_), Private(_))
            | (Protected(_), Protected(_))
            | (Public(_), Public(_))
            | (Return(_), Return(_))
            | (Static(_), Static(_))
            | (Super(_), Super(_))
            | (Switch(_), Switch(_))
            | (This(_), This(_))
            | (Throw(_), Throw(_))
            | (Try(_), Try(_))
            | (TypeOf(_), TypeOf(_))
            | (Var(_), Var(_))
            | (Void(_), Void(_))
            | (While(_), While(_))
            | (With(_), With(_))
            | (Yield(_), Yield(_)) => true,
            _ => false,
        }
    }
}

impl Keyword<()> {
    pub fn with_str<'a>(&self, s: &'a str) -> Keyword<&'a str> {
        match self {
            Keyword::Await(_) => Keyword::Await(s),
            Keyword::Break(_) => Keyword::Break(s),
            Keyword::Case(_) => Keyword::Case(s),
            Keyword::Catch(_) => Keyword::Catch(s),
            Keyword::Class(_) => Keyword::Class(s),
            Keyword::Const(_) => Keyword::Const(s),
            Keyword::Continue(_) => Keyword::Continue(s),
            Keyword::Debugger(_) => Keyword::Debugger(s),
            Keyword::Default(_) => Keyword::Default(s),
            Keyword::Delete(_) => Keyword::Delete(s),
            Keyword::Do(_) => Keyword::Do(s),
            Keyword::Else(_) => Keyword::Else(s),
            Keyword::Enum(_) => Keyword::Enum(s),
            Keyword::Export(_) => Keyword::Export(s),
            Keyword::Extends(_) => Keyword::Extends(s),
            Keyword::Finally(_) => Keyword::Finally(s),
            Keyword::For(_) => Keyword::For(s),
            Keyword::Function(_) => Keyword::Function(s),
            Keyword::If(_) => Keyword::If(s),
            Keyword::Implements(_) => Keyword::Implements(s),
            Keyword::Import(_) => Keyword::Import(s),
            Keyword::In(_) => Keyword::In(s),
            Keyword::InstanceOf(_) => Keyword::InstanceOf(s),
            Keyword::Interface(_) => Keyword::Interface(s),
            Keyword::Let(_) => Keyword::Let(s),
            Keyword::New(_) => Keyword::New(s),
            Keyword::Package(_) => Keyword::Package(s),
            Keyword::Private(_) => Keyword::Private(s),
            Keyword::Protected(_) => Keyword::Protected(s),
            Keyword::Public(_) => Keyword::Public(s),
            Keyword::Return(_) => Keyword::Return(s),
            Keyword::Static(_) => Keyword::Static(s),
            Keyword::Super(_) => Keyword::Super(s),
            Keyword::Switch(_) => Keyword::Switch(s),
            Keyword::This(_) => Keyword::This(s),
            Keyword::Throw(_) => Keyword::Throw(s),
            Keyword::Try(_) => Keyword::Try(s),
            Keyword::TypeOf(_) => Keyword::TypeOf(s),
            Keyword::Var(_) => Keyword::Var(s),
            Keyword::Void(_) => Keyword::Void(s),
            Keyword::While(_) => Keyword::While(s),
            Keyword::With(_) => Keyword::With(s),
            Keyword::Yield(_) => Keyword::Yield(s),
        }
    }
}

impl<T> ::std::string::ToString for Keyword<T> {
    /// Convert a keyword into a string
    fn to_string(&self) -> String {
        self.as_str().into()
    }
}

impl<T> Keyword<T> {
    /// Is this keyword one of the future reserved words
    ///
    /// - enum
    /// - export
    /// - implements
    /// - super
    pub fn is_future_reserved(&self) -> bool {
        match self {
            Keyword::Enum(_) => true,
            Keyword::Export(_) => true,
            Keyword::Implements(_) => true,
            Keyword::Super(_) => true,
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
    pub fn is_strict_reserved(&self) -> bool {
        match self {
            Keyword::Implements(_) => true,
            Keyword::Interface(_) => true,
            Keyword::Package(_) => true,
            Keyword::Private(_) => true,
            Keyword::Protected(_) => true,
            Keyword::Public(_) => true,
            Keyword::Static(_) => true,
            Keyword::Yield(_) => true,
            Keyword::Let(_) => true,
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
    pub fn is_reserved(&self) -> bool {
        match self {
            Keyword::Break(_)
            | Keyword::Case(_)
            | Keyword::Catch(_)
            | Keyword::Class(_)
            | Keyword::Continue(_)
            | Keyword::Debugger(_)
            | Keyword::Default(_)
            | Keyword::Delete(_)
            | Keyword::Do(_)
            | Keyword::Else(_)
            | Keyword::Export(_)
            | Keyword::Extends(_)
            | Keyword::Finally(_)
            | Keyword::For(_)
            | Keyword::Function(_)
            | Keyword::If(_)
            | Keyword::Import(_)
            | Keyword::In(_)
            | Keyword::InstanceOf(_)
            | Keyword::New(_)
            | Keyword::Return(_)
            | Keyword::Switch(_)
            | Keyword::Super(_)
            | Keyword::This(_)
            | Keyword::Throw(_)
            | Keyword::Try(_)
            | Keyword::TypeOf(_)
            | Keyword::Var(_)
            | Keyword::Void(_)
            | Keyword::While(_)
            | Keyword::With(_) => true,
            _ => false,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Keyword::Await(_) => "await",
            Keyword::Break(_) => "break",
            Keyword::Case(_) => "case",
            Keyword::Catch(_) => "catch",
            Keyword::Class(_) => "class",
            Keyword::Const(_) => "const",
            Keyword::Continue(_) => "continue",
            Keyword::Debugger(_) => "debugger",
            Keyword::Default(_) => "default",
            Keyword::Import(_) => "import",
            Keyword::Delete(_) => "delete",
            Keyword::Do(_) => "do",
            Keyword::Else(_) => "else",
            Keyword::Enum(_) => "enum",
            Keyword::Export(_) => "export",
            Keyword::Extends(_) => "extends",
            Keyword::Finally(_) => "finally",
            Keyword::For(_) => "for",
            Keyword::Function(_) => "function",
            Keyword::If(_) => "if",
            Keyword::In(_) => "in",
            Keyword::Implements(_) => "implements",
            Keyword::InstanceOf(_) => "instanceof",
            Keyword::Interface(_) => "interface",
            Keyword::Let(_) => "let",
            Keyword::New(_) => "new",
            Keyword::Package(_) => "package",
            Keyword::Private(_) => "private",
            Keyword::Protected(_) => "protected",
            Keyword::Public(_) => "public",
            Keyword::Static(_) => "static",
            Keyword::Return(_) => "return",
            Keyword::Super(_) => "super",
            Keyword::Switch(_) => "switch",
            Keyword::This(_) => "this",
            Keyword::Throw(_) => "throw",
            Keyword::Try(_) => "try",
            Keyword::TypeOf(_) => "typeof",
            Keyword::Var(_) => "var",
            Keyword::Void(_) => "void",
            Keyword::While(_) => "while",
            Keyword::With(_) => "with",
            Keyword::Yield(_) => "yield",
        }
    }

    pub fn to_empty(&self) -> Keyword<()> {
        match self {
            Keyword::Await(_) => Keyword::Await(()),
            Keyword::Break(_) => Keyword::Break(()),
            Keyword::Case(_) => Keyword::Case(()),
            Keyword::Catch(_) => Keyword::Catch(()),
            Keyword::Class(_) => Keyword::Class(()),
            Keyword::Const(_) => Keyword::Const(()),
            Keyword::Continue(_) => Keyword::Continue(()),
            Keyword::Debugger(_) => Keyword::Debugger(()),
            Keyword::Default(_) => Keyword::Default(()),
            Keyword::Import(_) => Keyword::Import(()),
            Keyword::Delete(_) => Keyword::Delete(()),
            Keyword::Do(_) => Keyword::Do(()),
            Keyword::Else(_) => Keyword::Else(()),
            Keyword::Enum(_) => Keyword::Enum(()),
            Keyword::Export(_) => Keyword::Export(()),
            Keyword::Extends(_) => Keyword::Extends(()),
            Keyword::Finally(_) => Keyword::Finally(()),
            Keyword::For(_) => Keyword::For(()),
            Keyword::Function(_) => Keyword::Function(()),
            Keyword::If(_) => Keyword::If(()),
            Keyword::In(_) => Keyword::In(()),
            Keyword::Implements(_) => Keyword::Implements(()),
            Keyword::InstanceOf(_) => Keyword::InstanceOf(()),
            Keyword::Interface(_) => Keyword::Interface(()),
            Keyword::Let(_) => Keyword::Let(()),
            Keyword::New(_) => Keyword::New(()),
            Keyword::Package(_) => Keyword::Package(()),
            Keyword::Private(_) => Keyword::Private(()),
            Keyword::Protected(_) => Keyword::Protected(()),
            Keyword::Public(_) => Keyword::Public(()),
            Keyword::Static(_) => Keyword::Static(()),
            Keyword::Return(_) => Keyword::Return(()),
            Keyword::Super(_) => Keyword::Super(()),
            Keyword::Switch(_) => Keyword::Switch(()),
            Keyword::This(_) => Keyword::This(()),
            Keyword::Throw(_) => Keyword::Throw(()),
            Keyword::Try(_) => Keyword::Try(()),
            Keyword::TypeOf(_) => Keyword::TypeOf(()),
            Keyword::Var(_) => Keyword::Var(()),
            Keyword::Void(_) => Keyword::Void(()),
            Keyword::While(_) => Keyword::While(()),
            Keyword::With(_) => Keyword::With(()),
            Keyword::Yield(_) => Keyword::Yield(()),
        }
    }
}

impl<'a> Keyword<&'a str> {
    pub fn has_unicode_escape(&self) -> bool {
        match self {
            Keyword::Await(s) => s,
            Keyword::Break(s) => s,
            Keyword::Case(s) => s,
            Keyword::Catch(s) => s,
            Keyword::Class(s) => s,
            Keyword::Const(s) => s,
            Keyword::Continue(s) => s,
            Keyword::Debugger(s) => s,
            Keyword::Default(s) => s,
            Keyword::Import(s) => s,
            Keyword::Delete(s) => s,
            Keyword::Do(s) => s,
            Keyword::Else(s) => s,
            Keyword::Enum(s) => s,
            Keyword::Export(s) => s,
            Keyword::Extends(s) => s,
            Keyword::Finally(s) => s,
            Keyword::For(s) => s,
            Keyword::Function(s) => s,
            Keyword::If(s) => s,
            Keyword::In(s) => s,
            Keyword::Implements(s) => s,
            Keyword::InstanceOf(s) => s,
            Keyword::Interface(s) => s,
            Keyword::Let(s) => s,
            Keyword::New(s) => s,
            Keyword::Package(s) => s,
            Keyword::Private(s) => s,
            Keyword::Protected(s) => s,
            Keyword::Public(s) => s,
            Keyword::Static(s) => s,
            Keyword::Return(s) => s,
            Keyword::Super(s) => s,
            Keyword::Switch(s) => s,
            Keyword::This(s) => s,
            Keyword::Throw(s) => s,
            Keyword::Try(s) => s,
            Keyword::TypeOf(s) => s,
            Keyword::Var(s) => s,
            Keyword::Void(s) => s,
            Keyword::While(s) => s,
            Keyword::With(s) => s,
            Keyword::Yield(s) => s,
        }.contains("\\u")
    }
}

impl<'a> TokenExt for Token<&'a str> {
    fn is_boolean(&self) -> bool {
        match self {
            Token::Boolean(_) => true,
            _ => false,
        }
    }
    fn is_boolean_true(&self) -> bool {
        match self {
            Token::Boolean(ref b) => b.into(),
            _ => false,
        }
    }
    fn is_boolean_false(&self) -> bool {
        match self {
            Token::Boolean(ref b) => {
                let b: bool = b.into();
                !b
            }
            _ => false,
        }
    }
    fn is_eof(&self) -> bool {
        match self {
            Token::EoF => true,
            _ => false,
        }
    }
    fn is_ident(&self) -> bool {
        match self {
            Token::Ident(_) => true,
            _ => false,
        }
    }
    fn is_keyword(&self) -> bool {
        match self {
            Token::Keyword(_) => true,
            _ => false,
        }
    }
    fn is_strict_reserved(&self) -> bool {
        match self {
            Token::Keyword(ref k) => k.is_strict_reserved(),
            _ => false,
        }
    }
    fn is_restricted(&self) -> bool {
        match self {
            Token::Ident(ref i) => i == "arguments" || i == "eval",
            _ => false,
        }
    }
    fn is_null(&self) -> bool {
        match self {
            Token::Null => true,
            _ => false,
        }
    }

    fn is_number(&self) -> bool {
        if let Token::Number(ref _n) = self {
            true
        } else {
            false
        }
    }
    fn is_hex_literal(&self) -> bool {
        match self {
            Token::Number(ref n) => n.is_hex(),
            _ => false,
        }
    }
    fn is_bin_literal(&self) -> bool {
        match self {
            Token::Number(ref n) => n.is_bin(),
            _ => false,
        }
    }
    fn is_oct_literal(&self) -> bool {
        match self {
            Token::Number(ref n) => n.is_oct(),
            _ => false,
        }
    }
    fn is_punct(&self) -> bool {
        match self {
            Token::Punct(_) => true,
            _ => false,
        }
    }
    fn is_string(&self) -> bool {
        if let Token::String(ref _s) = self {
            true
        } else {
            false
        }
    }
    fn is_double_quoted_string(&self) -> bool {
        match self {
            Token::String(ref s) => match s {
                StringLit::Double(_) => true,
                _ => false,
            },
            _ => false,
        }
    }
    fn is_single_quoted_string(&self) -> bool {
        match self {
            Token::String(ref s) => match s {
                StringLit::Single(_) => true,
                _ => false,
            },
            _ => false,
        }
    }
    fn is_regex(&self) -> bool {
        match self {
            Token::RegEx(_) => true,
            _ => false,
        }
    }
    fn is_template(&self) -> bool {
        match self {
            Token::Template(_) => true,
            _ => false,
        }
    }
    fn is_template_no_sub(&self) -> bool {
        match self {
            Token::Template(ref s) => s.is_no_sub(),
            _ => false,
        }
    }
    fn is_template_head(&self) -> bool {
        match self {
            Token::Template(ref s) => s.is_head() || s.is_no_sub(),
            _ => false,
        }
    }
    fn is_template_body(&self) -> bool {
        match self {
            Token::Template(ref s) => s.is_middle(),
            _ => false,
        }
    }
    fn is_template_tail(&self) -> bool {
        match self {
            Token::Template(ref s) => s.is_tail() || s.is_no_sub(),
            _ => false,
        }
    }
    fn is_literal(&self) -> bool {
        match self {
            Token::Boolean(_) => true,
            Token::String(_) => true,
            Token::Null => true,
            Token::Number(_) => true,
            Token::RegEx(_) => true,
            Token::Template(_) => true,
            _ => false,
        }
    }
    fn is_comment(&self) -> bool {
        match self {
            Token::Comment(_) => true,
            _ => false,
        }
    }
    fn is_multi_line_comment(&self) -> bool {
        match self {
            Token::Comment(ref t) => t.kind == CommentKind::Multi,
            _ => false,
        }
    }

    fn is_single_line_comment(&self) -> bool {
        match self {
            Token::Comment(ref t) => t.kind == CommentKind::Single,
            _ => false,
        }
    }
    fn matches_boolean(&self, b: Boolean) -> bool {
        match self {
            Token::Boolean(m) => m == &b,
            _ => false,
        }
    }
    fn matches_boolean_str(&self, b: &str) -> bool {
        match self {
            Token::Boolean(ref lit) => match (lit, b) {
                (&Boolean::True, "true") | (&Boolean::False, "false") => true,
                _ => false,
            },
            _ => false,
        }
    }
    fn matches_ident_str(&self, name: &str) -> bool {
        match self {
            Token::Ident(i) => i.matches(name),
            _ => false,
        }
    }
    fn matches_keyword<T>(&self, keyword: Keyword<T>) -> bool {
        match self {
            Token::Keyword(k) => k == &keyword,
            _ => false,
        }
    }
    fn matches_keyword_str(&self, name: &str) -> bool {
        match self {
            Token::Keyword(n) => n.as_str() == name,
            _ => false,
        }
    }
    fn matches_number_str(&self, number: &str) -> bool {
        match self {
            Token::Number(n) => n == number,
            _ => false,
        }
    }
    fn matches_punct(&self, p: Punct) -> bool {
        match self {
            Token::Punct(m) => m == &p,
            _ => false,
        }
    }

    fn matches_punct_str(&self, s: &str) -> bool {
        match self {
            Token::Punct(ref p) => p.matches_str(s),
            _ => false,
        }
    }

    fn matches_comment_str(&self, comment: &str) -> bool {
        match self {
            Token::Comment(ref t) => t.content == comment,
            _ => false,
        }
    }

    fn matches_string_content(&self, content: &str) -> bool {
        match self {
            Token::String(ref lit) => match lit {
                StringLit::Single(s) => content == *s,
                StringLit::Double(s) => content == *s,
            },
            _ => false,
        }
    }
}
