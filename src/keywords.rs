use combine::{
    choice, error::ParseError, not_followed_by,
    parser::{
        char::string,
    },
    try, Parser, Stream,
};
use tokens::{ident_part, Token};
#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Break,
    Case,
    Catch,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Finally,
    For,
    Function,
    If,
    InstanceOf,
    In,
    New,
    Return,
    Switch,
    This,
    Throw,
    Try,
    TypeOf,
    Var,
    Void,
    While,
    With,
    Export,
    Import,
    Super,
    Enum,
    Implements,
    Interface,
    Package,
    Private,
    Protected,
    Public,
    Static,
    Yield,
    Let,
    Eval,
    Arguments,
    Of,
    Const,
}

impl<'a> From<&'a str> for Keyword {
    fn from(s: &'a str) -> Self {
        match s {
            "break" => Keyword::Break,
            "case" => Keyword::Case,
            "catch" => Keyword::Catch,
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
            "eval" => Keyword::Eval,
            "arguments" => Keyword::Arguments,
            "of" => Keyword::Of,
            _ => panic!("Unknown Keyword, `{}`", s)
        }
    }
}

impl From<String> for Keyword {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl ::std::string::ToString for Keyword {
    fn to_string(&self) -> String {
        match self {
            Keyword::Break => "break".into(),
            Keyword::Case => "case".into(),
            Keyword::Catch => "catch".into(),
            Keyword::Continue => "continue".into(),
            Keyword::Debugger => "debugger".into(),
            Keyword::Default => "default".into(),
            Keyword::Delete => "delete".into(),
            Keyword::Do => "do".into(),
            Keyword::Else => "else".into(),
            Keyword::Finally => "finally".into(),
            Keyword::For => "for".into(),
            Keyword::Function => "function".into(),
            Keyword::If => "if".into(),
            Keyword::InstanceOf => "instanceof".into(),
            Keyword::In => "in".into(),
            Keyword::New => "new".into(),
            Keyword::Return => "return".into(),
            Keyword::Switch => "switch".into(),
            Keyword::This => "this".into(),
            Keyword::Throw => "throw".into(),
            Keyword::Try => "try".into(),
            Keyword::TypeOf => "typeof".into(),
            Keyword::Var => "var".into(),
            Keyword::Void => "void".into(),
            Keyword::While => "while".into(),
            Keyword::With => "with".into(),
            Keyword::Export => "export".into(),
            Keyword::Import => "import".into(),
            Keyword::Super => "super".into(),
            Keyword::Enum => "enum".into(),
            Keyword::Implements => "implements".into(),
            Keyword::Interface => "interface".into(),
            Keyword::Package => "package".into(),
            Keyword::Private => "private".into(),
            Keyword::Protected => "protected".into(),
            Keyword::Public => "public".into(),
            Keyword::Static => "static".into(),
            Keyword::Yield => "yield".into(),
            Keyword::Let => "let".into(),
            Keyword::Eval => "eval".into(),
            Keyword::Arguments => "arguments".into(),
            Keyword::Of => "of".into()
        }
    }
}

impl Keyword {
    pub fn is_future_reserved(&self) -> bool {
        match self {
            &Keyword::Export => true,
            &Keyword::Implements => true,
            &Keyword::Super => true,
            &Keyword::Enum => true,
            _ => false,
        }
    }
    pub fn is_strict_reserved(&self) -> bool {
        match self {
            &Keyword::Implements => true,
            &Keyword::Interface => true,
            &Keyword::Package => true,
            &Keyword::Private => true,
            &Keyword::Protected => true,
            &Keyword::Public => true,
            &Keyword::Static => true,
            &Keyword::Yield => true,
            &Keyword::Let => true,
            _ => false
        }
    }
    pub fn is_restricted(&self) -> bool {
        match self {
            &Keyword::Eval => true,
            &Keyword::Arguments => true,
            _ => false,
        }
    }
    pub fn is_reserved(&self) -> bool {
        match self {
            &Keyword::Break => true,
            &Keyword::Case => true,
            &Keyword::Catch => true,
            &Keyword::Continue => true,
            &Keyword::Debugger => true,
            &Keyword::Default => true,
            &Keyword::Delete => true,
            &Keyword::Do => true,
            &Keyword::Else => true,
            &Keyword::Finally => true,
            &Keyword::For => true,
            &Keyword::Function => true,
            &Keyword::If => true,
            &Keyword::InstanceOf => true,
            &Keyword::In => true,
            &Keyword::New => true,
            &Keyword::Return => true,
            &Keyword::Switch => true,
            &Keyword::This => true,
            &Keyword::Throw => true,
            &Keyword::Try => true,
            &Keyword::TypeOf => true,
            &Keyword::Var => true,
            &Keyword::Void => true,
            &Keyword::While => true,
            &Keyword::With => true,
            _ => false
        }
    }
}

pub(crate) fn literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(future_reserved()),
        try(strict_mode_reserved()),
        try(restricted()),
        try(reserved()),
    )).skip(not_followed_by(ident_part()))
        .map(|t| t)
}

pub(crate) fn reserved<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice([
        try(string("break")),
        try(string("case")),
        try(string("catch")),
        try(string("continue")),
        try(string("debugger")),
        try(string("default")),
        try(string("delete")),
        try(string("do")),
        try(string("else")),
        try(string("finally")),
        try(string("for")),
        try(string("function")),
        try(string("if")),
        try(string("instanceof")),
        try(string("in")),
        try(string("new")),
        try(string("return")),
        try(string("switch")),
        try(string("this")),
        try(string("throw")),
        try(string("try")),
        try(string("typeof")),
        try(string("var")),
        try(string("void")),
        try(string("while")),
        try(string("with")),
    ]).map(|t| Token::Keyword(Keyword::from(t.to_owned())))
}

pub(crate) fn future_reserved<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(string("export")),
        try(string("import")),
        try(string("super")),
        try(string("enum")),
    )).map(|t| Token::Keyword(Keyword::from(t)))
}

pub(crate) fn strict_mode_reserved<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(string("implements")),
        try(string("interface")),
        try(string("package")),
        try(string("private")),
        try(string("protected")),
        try(string("public")),
        try(string("static")),
        try(string("yield")),
        try(string("let")),
    )).map(|t| Token::Keyword(Keyword::from(t)))
}

pub(crate) fn restricted<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(string("eval")), try(string("arguments")))).map(|t| Token::Keyword(Keyword::from(t)))
}

#[cfg(test)]
mod test {
    use super::*;
    use tokens::token;
    use tokens::Token;
    #[test]
    fn future_reserved() {
        let keywords = ["enum", "export", "import", "super"];
        for keyword in keywords.iter() {
            let k = token().parse(keyword.clone()).unwrap();
            assert_eq!(k, (Token::keyword(*keyword), ""))
        }
        match super::future_reserved().parse("junk") {
            Ok(k) => panic!("parsed junk as {:?}", k),
            Err(_) => (),
        }
    }

    #[test]
    fn strict_mode_reserved() {
        let keywords = [
            "implements",
            "interface",
            "package",
            "private",
            "protected",
            "public",
            "static",
            "yield",
            "let",
        ];
        for keyword in keywords.iter() {
            let k = token().parse(keyword.clone())
                .unwrap();
            assert_eq!(k, (Token::keyword(*keyword), ""));
        }
        match super::strict_mode_reserved().parse("junk") {
            Ok(k) => panic!("parsed junk as {:?}", k),
            Err(_) => (),
        }
    }

    #[test]
    fn restricted_reserved() {
        let k = token().parse("eval").unwrap();
        assert_eq!(k, (Token::keyword("eval"), ""));
        let k2 = token().parse("arguments").unwrap();
        assert_eq!(k2, (Token::keyword("arguments"), ""))
    }

    #[test]
    fn reserved_keywords() {
        let keys = vec![
            "break",
            "case",
            "catch",
            "continue",
            "debugger",
            "default",
            "delete",
            "do",
            "else",
            "finally",
            "for",
            "function",
            "if",
            "instanceof",
            "in",
            "new",
            "return",
            "switch",
            "this",
            "throw",
            "try",
            "typeof",
            "var",
            "void",
            "while",
            "with",
        ];
        for key in keys {
            let k = token().parse(key.clone()).unwrap();
            assert_eq!(k, (Token::keyword(key), ""));
        }
    }

    #[test]
    fn keywords_test() {
        let keys = vec![
            "enum",
            "export",
            "import",
            "super",
            "implements",
            "interface",
            "package",
            "private",
            "protected",
            "public",
            "static",
            "yield",
            "let",
            "eval",
            "break",
            "case",
            "catch",
            "continue",
            "debugger",
            "default",
            "delete",
            "do",
            "else",
            "finally",
            "for",
            "function",
            "if",
            "instanceof",
            "in",
            "new",
            "return",
            "switch",
            "this",
            "throw",
            "try",
            "typeof",
            "var",
            "void",
            "while",
            "with",
        ];
        for key in keys {
            let k = token().parse(key.clone()).unwrap();
            assert_eq!(k, (Token::keyword(key), ""));
        }
    }
}