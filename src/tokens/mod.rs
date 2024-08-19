mod boolean;
mod comment;
mod ident;
mod keyword;
mod number;
mod regex;
mod string;
mod template;

pub mod prelude {
    pub use super::{
        Boolean, Comment, Ident, Keyword, Number, Punct, RegEx, StringLit, Template,
        TemplateLiteral, Token,
    };
}

use std::fmt::Display;

pub use boolean::Boolean;
pub use comment::{Comment, CommentKind};
pub use ident::Ident;
pub use keyword::Keyword;
pub use number::{Number, NumberKind};
pub use regex::RegEx;
pub use string::{InnerString, StringLit};
pub use template::{Template, TemplateLiteral};

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

impl<T> PartialEq<&str> for Token<T>
where
    T: AsRef<str>,
{
    fn eq(&self, other: &&str) -> bool {
        match self {
            Token::Boolean(b) => b.eq(*other),
            Token::EoF => (*other).eq(""),
            Token::Ident(s) => s.eq(other),
            Token::Keyword(k) => k.as_str().eq(*other),
            Token::Null => (*other).eq("null"),
            Token::Number(n) => n.eq(other),
            Token::Punct(p) => p.eq(*other),
            Token::String(s) => s.as_ref().eq(*other),
            _ => false,
        }
    }
}

impl<T> PartialEq<bool> for Token<T> {
    fn eq(&self, other: &bool) -> bool {
        if let Token::Boolean(b) = self {
            b == other
        } else {
            false
        }
    }
}

impl<T> Token<T> {
    pub fn is_boolean(&self) -> bool {
        matches!(self, Token::Boolean(_))
    }

    pub fn is_boolean_true(&self) -> bool {
        match self {
            Token::Boolean(ref b) => b.into(),
            _ => false,
        }
    }

    pub fn is_boolean_false(&self) -> bool {
        match self {
            Token::Boolean(ref b) => {
                let b: bool = b.into();
                !b
            }
            _ => false,
        }
    }

    pub fn is_eof(&self) -> bool {
        matches!(self, Token::EoF)
    }

    pub fn is_ident(&self) -> bool {
        matches!(self, Token::Ident(_))
    }

    pub fn is_keyword(&self) -> bool {
        matches!(self, Token::Keyword(_))
    }

    pub fn is_strict_reserved(&self) -> bool {
        match self {
            Token::Keyword(ref k) => k.is_strict_reserved(),
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Token::Null)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Token::Number(_))
    }

    pub fn is_punct(&self) -> bool {
        matches!(self, Token::Punct(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Token::String(_))
    }

    pub fn is_double_quoted_string(&self) -> bool {
        matches!(self, Token::String(StringLit::Double(_)))
    }

    pub fn is_single_quoted_string(&self) -> bool {
        matches!(self, Token::String(StringLit::Single(_)))
    }

    pub fn is_regex(&self) -> bool {
        matches!(self, Token::RegEx(_))
    }

    pub fn is_template(&self) -> bool {
        matches!(self, Token::Template(_))
    }

    pub fn is_template_no_sub(&self) -> bool {
        match self {
            Token::Template(ref s) => s.is_no_sub(),
            _ => false,
        }
    }

    pub fn is_template_head(&self) -> bool {
        match self {
            Token::Template(ref s) => s.is_head() || s.is_no_sub(),
            _ => false,
        }
    }

    pub fn is_template_body(&self) -> bool {
        match self {
            Token::Template(ref s) => s.is_middle(),
            _ => false,
        }
    }

    pub fn is_template_tail(&self) -> bool {
        match self {
            Token::Template(ref s) => s.is_tail() || s.is_no_sub(),
            _ => false,
        }
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            Token::Boolean(_)
                | Token::String(_)
                | Token::Null
                | Token::Number(_)
                | Token::RegEx(_)
                | Token::Template(_)
        )
    }

    pub fn is_comment(&self) -> bool {
        matches!(self, Token::Comment(_))
    }

    pub fn is_multi_line_comment(&self) -> bool {
        match self {
            Token::Comment(ref t) => t.kind == CommentKind::Multi,
            _ => false,
        }
    }

    pub fn is_single_line_comment(&self) -> bool {
        match self {
            Token::Comment(ref t) => t.kind == CommentKind::Single,
            _ => false,
        }
    }

    pub fn matches_boolean(&self, b: Boolean) -> bool {
        match self {
            Token::Boolean(m) => m == &b,
            _ => false,
        }
    }
    pub fn matches_boolean_str(&self, b: &str) -> bool {
        match self {
            Token::Boolean(ref lit) => matches!(
                (lit, b),
                (&Boolean::True, "true") | (&Boolean::False, "false")
            ),
            _ => false,
        }
    }

    pub fn matches_keyword<K>(&self, keyword: Keyword<K>) -> bool {
        match self {
            Token::Keyword(k) => k.eq(&keyword),
            _ => false,
        }
    }

    pub fn matches_keyword_str(&self, name: &str) -> bool {
        match self {
            Token::Keyword(n) => n.as_str() == name,
            _ => false,
        }
    }

    pub fn matches_punct(&self, p: Punct) -> bool {
        match self {
            Token::Punct(m) => m == &p,
            _ => false,
        }
    }

    pub fn matches_punct_str(&self, s: &str) -> bool {
        match self {
            Token::Punct(ref p) => p.matches_str(s),
            _ => false,
        }
    }
}

impl<T> Token<T>
where
    T: AsRef<str>,
{
    pub fn is_restricted(&self) -> bool {
        match self {
            Token::Ident(ref i) => i.as_ref() == "arguments" || i.as_ref() == "eval",
            _ => false,
        }
    }

    pub fn is_hex_literal(&self) -> bool {
        match self {
            Token::Number(ref n) => n.is_hex(),
            _ => false,
        }
    }

    pub fn is_bin_literal(&self) -> bool {
        match self {
            Token::Number(ref n) => n.is_bin(),
            _ => false,
        }
    }

    pub fn is_oct_literal(&self) -> bool {
        match self {
            Token::Number(ref n) => n.is_oct(),
            _ => false,
        }
    }

    pub fn matches_ident_str(&self, name: &str) -> bool {
        match self {
            Token::Ident(i) => i.eq(name),
            _ => false,
        }
    }

    pub fn matches_number_str(&self, number: &str) -> bool {
        match self {
            Token::Number(n) => n.eq(number),
            _ => false,
        }
    }

    pub fn matches_comment_str(&self, comment: &str) -> bool {
        match self {
            Token::Comment(t) => t.content.as_ref() == comment,
            _ => false,
        }
    }

    pub fn matches_string_content(&self, content: &str) -> bool {
        match self {
            Token::String(ref lit) => match lit {
                StringLit::Single(s) => content == s.content.as_ref(),
                StringLit::Double(s) => content == s.content.as_ref(),
            },
            _ => false,
        }
    }
}

impl<T> ToString for Token<T>
where
    T: Display,
{
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    DoubleAmpersandEqual,
    DoubleAsterisk,
    DoubleAsteriskEqual,
    DoubleEqual,
    DoubleGreaterThan,
    DoubleGreaterThanEqual,
    DoubleLessThan,
    DoubleLessThanEqual,
    DoublePipe,
    DoublePipeEqual,
    DoublePlus,
    DoubleQuestionMark,
    DoubleQuestionMarkEqual,
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
    QuestionMarkDot,
    SemiColon,
    Tilde,
    TripleEqual,
    TripleGreaterThanEqual,
    TripleGreaterThan,
}

impl PartialEq<str> for Punct {
    fn eq(&self, other: &str) -> bool {
        self.matches_str(other)
    }
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
            Punct::DoubleAmpersandEqual => s == "&&=",
            Punct::DoublePipeEqual => s == "||=",
            Punct::DoubleQuestionMark => s == "??",
            Punct::DoubleQuestionMarkEqual => s == "??=",
            Punct::QuestionMarkDot => s == "?.",
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
            Punct::DoubleAmpersandEqual => "&&=",
            Punct::DoublePipeEqual => "||=",
            Punct::DoubleQuestionMark => "??",
            Punct::DoubleQuestionMarkEqual => "??=",
            Punct::QuestionMarkDot => "?.",
        }
        .into()
    }
}

impl<'a> Token<&'a str> {
    pub fn is_div_punct(&self) -> bool {
        matches!(
            self,
            Token::Punct(Punct::ForwardSlashEqual) | Token::Punct(Punct::ForwardSlash)
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn booleans() {
        let t = Token::<&str>::Boolean(Boolean::True);
        let f = Token::<&str>::Boolean(Boolean::False);
        assert!(t.is_boolean());
        assert!(f.is_boolean());
        assert!(t.is_boolean_true());
        assert!(!f.is_boolean_true());
        assert_eq!(t, true);
        assert_eq!(t, "true");
        assert_eq!(f, false);
        assert_eq!(f, "false");
    }

    #[test]
    fn comments() {
        let c1 = Token::Comment(Comment::new_single_line("comment"));
        assert!(c1.is_comment());
        assert!(c1.is_single_line_comment());
        assert!(!c1.is_multi_line_comment());
        let c2 = Token::Comment(Comment::new_multi_line("comment\ncomment"));
        assert!(c2.is_comment());
        assert!(!c2.is_single_line_comment());
        assert!(c2.is_multi_line_comment());
        assert!(!Token::<&str>::Null.is_comment());
        assert!(!Token::<&str>::Null.is_multi_line_comment());
        assert!(!Token::<&str>::Null.is_single_line_comment());
    }

    #[test]
    fn idents() {
        let i = Token::Ident(Ident::from("asdf"));
        let ni = Token::<&str>::Null;
        assert!(i.is_ident());
        assert!(i.matches_ident_str("asdf"));
        assert!(i == "asdf");
        assert!(!ni.is_ident());
        assert!(!ni.matches_ident_str("asdf"));
    }

    #[test]
    fn keywords() {
        check_keyword("await", Token::Keyword(Keyword::Await("await")));
        check_keyword("break", Token::Keyword(Keyword::Break("break")));
        check_keyword("case", Token::Keyword(Keyword::Case("case")));
        check_keyword("catch", Token::Keyword(Keyword::Catch("catch")));
        check_keyword("class", Token::Keyword(Keyword::Class("class")));
        check_keyword("const", Token::Keyword(Keyword::Const("const")));
        check_keyword("continue", Token::Keyword(Keyword::Continue("continue")));
        check_keyword("debugger", Token::Keyword(Keyword::Debugger("debugger")));
        check_keyword("default", Token::Keyword(Keyword::Default("default")));
        check_keyword("import", Token::Keyword(Keyword::Import("import")));
        check_keyword("delete", Token::Keyword(Keyword::Delete("delete")));
        check_keyword("do", Token::Keyword(Keyword::Do("do")));
        check_keyword("else", Token::Keyword(Keyword::Else("else")));
        check_keyword("enum", Token::Keyword(Keyword::Enum("enum")));
        check_keyword("export", Token::Keyword(Keyword::Export("export")));
        check_keyword("extends", Token::Keyword(Keyword::Extends("extends")));
        check_keyword("finally", Token::Keyword(Keyword::Finally("finally")));
        check_keyword("for", Token::Keyword(Keyword::For("for")));
        check_keyword("function", Token::Keyword(Keyword::Function("function")));
        check_keyword("if", Token::Keyword(Keyword::If("if")));
        check_keyword("in", Token::Keyword(Keyword::In("in")));
        check_keyword(
            "implements",
            Token::Keyword(Keyword::Implements("implements")),
        );
        check_keyword(
            "instanceof",
            Token::Keyword(Keyword::InstanceOf("instanceof")),
        );
        check_keyword("interface", Token::Keyword(Keyword::Interface("interface")));
        check_keyword("let", Token::Keyword(Keyword::Let("let")));
        check_keyword("new", Token::Keyword(Keyword::New("new")));
        check_keyword("package", Token::Keyword(Keyword::Package("package")));
        check_keyword("private", Token::Keyword(Keyword::Private("private")));
        check_keyword("protected", Token::Keyword(Keyword::Protected("protected")));
        check_keyword("public", Token::Keyword(Keyword::Public("public")));
        check_keyword("return", Token::Keyword(Keyword::Return("return")));
        check_keyword("static", Token::Keyword(Keyword::Static("static")));
        check_keyword("super", Token::Keyword(Keyword::Super("super")));
        check_keyword("switch", Token::Keyword(Keyword::Switch("switch")));
        check_keyword("this", Token::Keyword(Keyword::This("this")));
        check_keyword("throw", Token::Keyword(Keyword::Throw("throw")));
        check_keyword("try", Token::Keyword(Keyword::Try("try")));
        check_keyword("typeof", Token::Keyword(Keyword::TypeOf("typeof")));
        check_keyword("var", Token::Keyword(Keyword::Var("var")));
        check_keyword("void", Token::Keyword(Keyword::Void("void")));
        check_keyword("while", Token::Keyword(Keyword::While("while")));
        check_keyword("with", Token::Keyword(Keyword::With("with")));
        check_keyword("yield", Token::Keyword(Keyword::Yield("yield")));
        assert!(!Token::<&str>::Null.matches_keyword(Keyword::Yield("yield")));
        assert!(!Token::<&str>::Null.matches_keyword_str("yield"));
    }

    fn check_keyword(s: &str, tok: Token<&str>) {
        assert!(tok.is_keyword());
        assert!(tok.matches_keyword(Keyword::new(s)), "{:?} vs {:?}", s, tok);
        assert!(tok.matches_keyword_str(s));
        assert_eq!(tok, s);
    }

    #[test]
    fn is_strict_reserved() {
        assert!(Token::Keyword(Keyword::Implements(())).is_strict_reserved());
        assert!(!Token::Keyword(Keyword::Await(())).is_strict_reserved());
        assert!(!Token::<&str>::Null.is_strict_reserved());
    }

    #[test]
    fn is_restricted() {
        assert!(Token::Ident(Ident::from("arguments")).is_restricted());
        assert!(Token::Ident(Ident::from("eval")).is_restricted());
        assert!(!Token::Ident(Ident::from("ident")).is_restricted());
        assert!(!Token::<&str>::Null.is_restricted());
    }

    #[test]
    fn numbers() {
        let int = "1234";
        let tok = Token::Number(Number::from(int));
        assert!(tok.is_number());
        assert!(tok.matches_number_str(int));
        assert_eq!(tok, int);
        assert!(!tok.is_oct_literal());
        assert!(!tok.is_bin_literal());
        assert!(!tok.is_hex_literal());
        let flt = "1.334";
        let tok = Token::Number(Number::from(flt));
        assert!(tok.is_number());
        assert!(tok.matches_number_str(flt));
        assert_eq!(tok, flt);
        assert!(!tok.is_oct_literal());
        assert!(!tok.is_bin_literal());
        assert!(!tok.is_hex_literal());
        let hex = "0x3";
        let tok = Token::Number(Number::from(hex));
        assert!(tok.is_number());
        assert!(tok.matches_number_str(hex));
        assert_eq!(tok, hex);
        assert!(!tok.is_oct_literal());
        assert!(!tok.is_bin_literal());
        assert!(tok.is_hex_literal());
        let hex2 = "0X3";
        let tok = Token::Number(Number::from(hex2));
        assert!(tok.is_number());
        assert!(tok.matches_number_str(hex2));
        assert_eq!(tok, hex2);
        assert!(!tok.is_oct_literal());
        assert!(!tok.is_bin_literal());
        assert!(tok.is_hex_literal());
        let oct = "0o4";
        let tok = Token::Number(Number::from(oct));
        assert!(tok.is_number());
        assert!(tok.matches_number_str(oct));
        assert_eq!(tok, oct);
        assert!(tok.is_oct_literal());
        assert!(!tok.is_bin_literal());
        assert!(!tok.is_hex_literal());
        let oct2 = "0O3";
        let tok = Token::Number(Number::from(oct2));
        assert!(tok.is_number());
        assert!(tok.matches_number_str(oct2));
        assert_eq!(tok, oct2);
        assert!(tok.is_oct_literal());
        assert!(!tok.is_bin_literal());
        assert!(!tok.is_hex_literal());
        let bin = "0b0";
        let tok = Token::Number(Number::from(bin));
        assert!(tok.is_number());
        assert!(tok.matches_number_str(bin));
        assert_eq!(tok, bin);
        assert!(!tok.is_oct_literal());
        assert!(tok.is_bin_literal());
        assert!(!tok.is_hex_literal());
        let bin2 = "0B1";
        let tok = Token::Number(Number::from(bin2));
        assert!(tok.is_number());
        assert!(tok.matches_number_str(bin2));
        assert_eq!(tok, bin2);
        assert!(!tok.is_oct_literal());
        assert!(tok.is_bin_literal());
        assert!(!tok.is_hex_literal());
        let exp = "1.22e2";
        let tok = Token::Number(Number::from(exp));
        assert!(tok.is_number());
        assert!(tok.matches_number_str(exp));
        assert_eq!(tok, exp);
        assert!(!tok.is_oct_literal());
        assert!(!tok.is_bin_literal());
        assert!(!tok.is_hex_literal());
        let exp2 = "1.3E8";
        let tok = Token::Number(Number::from(exp2));
        assert!(tok.is_number());
        assert!(tok.matches_number_str(exp2));
        assert_eq!(tok, exp2);
        assert!(!tok.is_oct_literal());
        assert!(!tok.is_bin_literal());
        assert!(!tok.is_hex_literal());
        assert!(!Token::<&str>::Null.is_hex_literal());
        assert!(!Token::<&str>::Null.is_bin_literal());
        assert!(!Token::<&str>::Null.is_oct_literal());
        assert!(!Token::<&str>::Null.matches_number_str("0.0"));
    }

    #[test]
    fn regexes() {
        let r = Token::RegEx(RegEx::from_parts("asdf", None));
        assert!(r.is_regex());
        assert_ne!(r, "");
        assert_ne!(r, "/asdf/");
    }

    #[test]
    fn strings() {
        let s1 = Token::String(StringLit::single("content", false));
        assert!(s1.is_string());
        assert!(s1.is_single_quoted_string());
        assert!(!s1.is_double_quoted_string());
        assert_eq!(s1, "content");
        let s2 = Token::String(StringLit::double("content", false));
        assert!(s2.is_string());
        assert!(!s2.is_single_quoted_string());
        assert!(s2.is_double_quoted_string());
        assert_eq!(s2, "content");
    }

    #[test]
    fn templates() {
        let t = Token::Template(Template::no_sub_template("asdf", false, false, false));
        assert!(t.is_template());
        assert!(t.is_template_head());
        assert!(!t.is_template_body());
        assert!(t.is_template_tail());
        assert!(t.is_template_no_sub());
        let t = Token::Template(Template::template_head("asdf", false, false, false));
        assert!(t.is_template());
        assert!(t.is_template_head());
        assert!(!t.is_template_body());
        assert!(!t.is_template_tail());
        assert!(!t.is_template_no_sub());
        let t = Token::Template(Template::template_middle("asdf", false, false, false));
        assert!(t.is_template());
        assert!(!t.is_template_head());
        assert!(t.is_template_body());
        assert!(!t.is_template_tail());
        assert!(!t.is_template_no_sub());
        let t = Token::Template(Template::template_tail("asdf", false, false, false));
        assert!(t.is_template());
        assert!(!t.is_template_head());
        assert!(!t.is_template_body());
        assert!(t.is_template_tail());
        assert!(!t.is_template_no_sub());
        assert_ne!(t, "");
        assert_ne!(t, "}asdf`");
        assert!(!Token::<&str>::Null.is_template());
        assert!(!Token::<&str>::Null.is_template_body());
        assert!(!Token::<&str>::Null.is_template_no_sub());
        assert!(!Token::<&str>::Null.is_template_head());
        assert!(!Token::<&str>::Null.is_template_tail());
    }

    #[test]
    fn partial_eq() {
        assert_eq!(Token::<&str>::Boolean(Boolean::True), "true");
        assert_eq!(Token::<&str>::EoF, "");
        assert_eq!(Token::Ident(Ident::from("ident")), "ident");
        assert_eq!(Token::Keyword(Keyword::new("await")), "await");
        assert_eq!(Token::<&str>::Null, "null");
        assert_eq!(Token::Number(Number::from("0.0")), "0.0");
        assert_eq!(Token::<&str>::Punct(Punct::Ampersand), "&");
        assert_eq!(Token::String(StringLit::double("string", false)), "string");
        assert!(Token::<&str>::Boolean(Boolean::True).eq(&true));
        assert!(Token::<&str>::Boolean(Boolean::False).eq(&false));
        assert_ne!(Token::<&str>::Null, false);
    }

    #[test]
    fn is_boolean() {
        assert!(Token::<&str>::Boolean(Boolean::True).is_boolean());
        assert!(!Token::<&str>::Null.is_boolean());
        assert!(Token::<&str>::Boolean(Boolean::True).is_boolean_true());
        assert!(!Token::<&str>::Boolean(Boolean::True).is_boolean_false());
        assert!(Token::<&str>::Boolean(Boolean::False).is_boolean_false());
        assert!(!Token::<&str>::Boolean(Boolean::False).is_boolean_true());
        assert!(!Token::<&str>::Null.is_boolean_true());
        assert!(!Token::<&str>::Null.is_boolean_false());
    }

    #[test]
    fn is_null() {
        assert!(Token::<&str>::Null.is_null());
        assert!(!Token::<&str>::EoF.is_null());
    }

    #[test]
    fn is_punct() {
        assert!(!Token::<&str>::Null.is_punct());
        assert!(!Token::<&str>::Null.is_div_punct());
        assert!(Token::<&str>::Punct(Punct::Ampersand).is_punct());
        assert!(!Token::<&str>::Punct(Punct::Ampersand).is_div_punct());
        assert!(Token::<&str>::Punct(Punct::ForwardSlash).is_div_punct());
        assert!(Token::<&str>::Punct(Punct::ForwardSlashEqual).is_div_punct());
    }

    #[test]
    fn is_literal() {
        assert!(Token::String(StringLit::double("content", false)).is_literal());
        assert!(Token::<&str>::Boolean(Boolean::True).is_literal());
        assert!(Token::Number(Number::from("0.0")).is_literal());
        assert!(Token::RegEx(RegEx::from_parts("regex", None)).is_literal());
        assert!(Token::Template(Template::NoSub(TemplateLiteral::new(
            "", false, false, false
        )))
        .is_literal());
    }

    #[test]
    fn matches_bool() {
        let t = Boolean::True;
        let f = Boolean::False;
        let tt = Token::<&str>::Boolean(t);
        let ft = Token::<&str>::Boolean(f);
        assert!(tt.matches_boolean(t.clone()));
        assert!(ft.matches_boolean(f.clone()));
        assert!(!tt.matches_boolean(f.clone()));
        assert!(!ft.matches_boolean(t.clone()));
        assert!(!Token::<&str>::Null.matches_boolean(t));
        assert!(tt.matches_boolean_str("true"));
        assert!(!tt.matches_boolean_str(""));
        assert!(ft.matches_boolean_str("false"));
        assert!(!Token::<&str>::Null.matches_boolean_str("true"));
    }

    #[test]
    fn token_to_string() {
        assert_eq!(Token::<&str>::Boolean(Boolean::True).to_string(), "true");
        assert_eq!(
            Token::<&str>::Comment(Comment::new_single_line("comment")).to_string(),
            "//comment"
        );
        assert_eq!(Token::<&str>::EoF.to_string(), "");
        assert_eq!(Token::<&str>::Ident(Ident::from("ident")), "ident");
        assert_eq!(
            Token::<&str>::Keyword(Keyword::Await("await")).to_string(),
            "await"
        );
        assert_eq!(Token::<&str>::Null.to_string(), "null");
        assert_eq!(
            Token::<&str>::Number(Number::from("0.0")).to_string(),
            "0.0"
        );
        assert_eq!(Token::<&str>::Punct(Punct::Ampersand).to_string(), "&");
        assert_eq!(
            Token::<&str>::RegEx(RegEx::from_parts("regex", None)).to_string(),
            "/regex/"
        );
        assert_eq!(
            Token::<&str>::String(StringLit::double("string", false)).to_string(),
            r#""string""#
        );
        assert_eq!(
            Token::<&str>::Template(Template::no_sub_template(
                "template no sub",
                false,
                false,
                false
            ))
            .to_string(),
            "`template no sub`"
        );
    }

    #[test]
    fn string_matches() {
        let content = "string";
        assert!(Token::String(StringLit::double(content, false)).matches_string_content(content));
        assert!(Token::String(StringLit::single(content, false)).matches_string_content(content));
        assert!(!Token::<&str>::Null.matches_string_content(content));
    }

    #[test]
    fn comment_matches() {
        let content = "comment";
        assert!(Token::Comment(Comment::new_single_line(content)).matches_comment_str(content));
        assert!(!Token::<&str>::Null.matches_comment_str(content));
    }

    #[test]
    fn punct_matches_str() {
        assert!(Punct::Ampersand.matches_str("&"));
        assert!(Punct::AmpersandEqual.matches_str("&="));
        assert!(Punct::Asterisk.matches_str("*"));
        assert!(Punct::AsteriskEqual.matches_str("*="));
        assert!(Punct::AtMark.matches_str("@"));
        assert!(Punct::Bang.matches_str("!"));
        assert!(Punct::BangDoubleEqual.matches_str("!=="));
        assert!(Punct::BangEqual.matches_str("!="));
        assert!(Punct::Caret.matches_str("^"));
        assert!(Punct::CaretEqual.matches_str("^="));
        assert!(Punct::CloseBrace.matches_str("}"));
        assert!(Punct::CloseBracket.matches_str("]"));
        assert!(Punct::CloseParen.matches_str(")"));
        assert!(Punct::Colon.matches_str(":"));
        assert!(Punct::Comma.matches_str(","));
        assert!(Punct::Dash.matches_str("-"));
        assert!(Punct::DoubleDash.matches_str("--"));
        assert!(Punct::DashEqual.matches_str("-="));
        assert!(Punct::DoubleAmpersand.matches_str("&&"));
        assert!(Punct::DoubleAmpersandEqual.matches_str("&&="));
        assert!(Punct::DoubleAsterisk.matches_str("**"));
        assert!(Punct::DoubleAsteriskEqual.matches_str("**="));
        assert!(Punct::DoubleEqual.matches_str("=="));
        assert!(Punct::DoubleGreaterThan.matches_str(">>"));
        assert!(Punct::DoubleGreaterThanEqual.matches_str(">>="));
        assert!(Punct::DoubleLessThan.matches_str("<<"));
        assert!(Punct::DoubleLessThanEqual.matches_str("<<="));
        assert!(Punct::DoublePipe.matches_str("||"));
        assert!(Punct::DoublePipeEqual.matches_str("||="));
        assert!(Punct::DoublePlus.matches_str("++"));
        assert!(Punct::DoubleQuestionMark.matches_str("??"));
        assert!(Punct::DoubleQuestionMarkEqual.matches_str("??="));
        assert!(Punct::Ellipsis.matches_str("..."));
        assert!(Punct::Equal.matches_str("="));
        assert!(Punct::EqualGreaterThan.matches_str("=>"));
        assert!(Punct::ForwardSlash.matches_str("/"));
        assert!(Punct::ForwardSlashEqual.matches_str("/="));
        assert!(Punct::GreaterThan.matches_str(">"));
        assert!(Punct::GreaterThanEqual.matches_str(">="));
        assert!(Punct::Hash.matches_str("#"));
        assert!(Punct::LessThan.matches_str("<"));
        assert!(Punct::LessThanEqual.matches_str("<="));
        assert!(Punct::OpenBrace.matches_str("{"));
        assert!(Punct::OpenBracket.matches_str("["));
        assert!(Punct::OpenParen.matches_str("("));
        assert!(Punct::Percent.matches_str("%"));
        assert!(Punct::PercentEqual.matches_str("%="));
        assert!(Punct::Period.matches_str("."));
        assert!(Punct::Pipe.matches_str("|"));
        assert!(Punct::PipeEqual.matches_str("|="));
        assert!(Punct::Plus.matches_str("+"));
        assert!(Punct::PlusEqual.matches_str("+="));
        assert!(Punct::QuestionMark.matches_str("?"));
        assert!(Punct::QuestionMarkDot.matches_str("?."));
        assert!(Punct::SemiColon.matches_str(";"));
        assert!(Punct::Tilde.matches_str("~"));
        assert!(Punct::TripleEqual.matches_str("==="));
        assert!(Punct::TripleGreaterThanEqual.matches_str(">>>="));
        assert!(Punct::TripleGreaterThan.matches_str(">>>"));
    }

    #[test]
    fn punct_to_string() {
        assert_eq!(Punct::Ampersand.to_string(), "&");
        assert_eq!(Punct::AmpersandEqual.to_string(), "&=");
        assert_eq!(Punct::Asterisk.to_string(), "*");
        assert_eq!(Punct::AsteriskEqual.to_string(), "*=");
        assert_eq!(Punct::AtMark.to_string(), "@");
        assert_eq!(Punct::Bang.to_string(), "!");
        assert_eq!(Punct::BangDoubleEqual.to_string(), "!==");
        assert_eq!(Punct::BangEqual.to_string(), "!=");
        assert_eq!(Punct::Caret.to_string(), "^");
        assert_eq!(Punct::CaretEqual.to_string(), "^=");
        assert_eq!(Punct::CloseBrace.to_string(), "}");
        assert_eq!(Punct::CloseBracket.to_string(), "]");
        assert_eq!(Punct::CloseParen.to_string(), ")");
        assert_eq!(Punct::Colon.to_string(), ":");
        assert_eq!(Punct::Comma.to_string(), ",");
        assert_eq!(Punct::Dash.to_string(), "-");
        assert_eq!(Punct::DoubleDash.to_string(), "--");
        assert_eq!(Punct::DashEqual.to_string(), "-=");
        assert_eq!(Punct::DoubleAmpersand.to_string(), "&&");
        assert_eq!(Punct::DoubleAmpersandEqual.to_string(), "&&=");
        assert_eq!(Punct::DoubleAsterisk.to_string(), "**");
        assert_eq!(Punct::DoubleAsteriskEqual.to_string(), "**=");
        assert_eq!(Punct::DoubleEqual.to_string(), "==");
        assert_eq!(Punct::DoubleGreaterThan.to_string(), ">>");
        assert_eq!(Punct::DoubleGreaterThanEqual.to_string(), ">>=");
        assert_eq!(Punct::DoubleLessThan.to_string(), "<<");
        assert_eq!(Punct::DoubleLessThanEqual.to_string(), "<<=");
        assert_eq!(Punct::DoublePipe.to_string(), "||");
        assert_eq!(Punct::DoublePipeEqual.to_string(), "||=");
        assert_eq!(Punct::DoublePlus.to_string(), "++");
        assert_eq!(Punct::DoubleQuestionMark.to_string(), "??");
        assert_eq!(Punct::DoubleQuestionMarkEqual.to_string(), "??=");
        assert_eq!(Punct::Ellipsis.to_string(), "...");
        assert_eq!(Punct::Equal.to_string(), "=");
        assert_eq!(Punct::EqualGreaterThan.to_string(), "=>");
        assert_eq!(Punct::ForwardSlash.to_string(), "/");
        assert_eq!(Punct::ForwardSlashEqual.to_string(), "/=");
        assert_eq!(Punct::GreaterThan.to_string(), ">");
        assert_eq!(Punct::GreaterThanEqual.to_string(), ">=");
        assert_eq!(Punct::Hash.to_string(), "#");
        assert_eq!(Punct::LessThan.to_string(), "<");
        assert_eq!(Punct::LessThanEqual.to_string(), "<=");
        assert_eq!(Punct::OpenBrace.to_string(), "{");
        assert_eq!(Punct::OpenBracket.to_string(), "[");
        assert_eq!(Punct::OpenParen.to_string(), "(");
        assert_eq!(Punct::Percent.to_string(), "%");
        assert_eq!(Punct::PercentEqual.to_string(), "%=");
        assert_eq!(Punct::Period.to_string(), ".");
        assert_eq!(Punct::Pipe.to_string(), "|");
        assert_eq!(Punct::PipeEqual.to_string(), "|=");
        assert_eq!(Punct::Plus.to_string(), "+");
        assert_eq!(Punct::PlusEqual.to_string(), "+=");
        assert_eq!(Punct::QuestionMark.to_string(), "?");
        assert_eq!(Punct::QuestionMarkDot.to_string(), "?.");
        assert_eq!(Punct::SemiColon.to_string(), ";");
        assert_eq!(Punct::Tilde.to_string(), "~");
        assert_eq!(Punct::TripleEqual.to_string(), "===");
        assert_eq!(Punct::TripleGreaterThanEqual.to_string(), ">>>=");
        assert_eq!(Punct::TripleGreaterThan.to_string(), ">>>");
    }
}
