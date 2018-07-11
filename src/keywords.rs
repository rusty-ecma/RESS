use combine::{
    choice, error::ParseError, not_followed_by,
    parser::{
        char::string,
    },
    try, Parser, Stream,
};
use tokens::{ident_part, TokenData};
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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
    Of
}

impl<'a> From<&'a str> for Token {
    fn from(s: &'a str) -> Token {
        match s {
            "break" => Token::Break,
            "case" => Token::Case,
            "catch" => Token::Catch,
            "continue" => Token::Continue,
            "debugger" => Token::Debugger,
            "default" => Token::Default,
            "delete" => Token::Delete,
            "do" => Token::Do,
            "else" => Token::Else,
            "finally" => Token::Finally,
            "for" => Token::For,
            "function" => Token::Function,
            "if" => Token::If,
            "instanceof" => Token::InstanceOf,
            "in" => Token::In,
            "new" => Token::New,
            "return" => Token::Return,
            "switch" => Token::Switch,
            "this" => Token::This,
            "throw" => Token::Throw,
            "try" => Token::Try,
            "typeof" => Token::TypeOf,
            "var" => Token::Var,
            "void" => Token::Void,
            "while" => Token::While,
            "with" => Token::With,
            "export" => Token::Export,
            "import" => Token::Import,
            "super" => Token::Super,
            "enum" => Token::Enum,
            "implements" => Token::Implements,
            "interface" => Token::Interface,
            "package" => Token::Package,
            "private" => Token::Private,
            "protected" => Token::Protected,
            "public" => Token::Public,
            "static" => Token::Static,
            "yield" => Token::Yield,
            "let" => Token::Let,
            "eval" => Token::Eval,
            "arguments" => Token::Arguments,
            "of" => Token::Of,
            _ => panic!("Unknown Keyword, `{}`", s)
        }
    }
}

impl From<String> for Token {
    fn from(s: String) -> Token {
        Self::from(s.as_str())
    }
}

impl ::std::string::ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Break => "break".into(),
            Token::Case => "case".into(),
            Token::Catch => "catch".into(),
            Token::Continue => "continue".into(),
            Token::Debugger => "debugger".into(),
            Token::Default => "default".into(),
            Token::Delete => "delete".into(),
            Token::Do => "do".into(),
            Token::Else => "else".into(),
            Token::Finally => "finally".into(),
            Token::For => "for".into(),
            Token::Function => "function".into(),
            Token::If => "if".into(),
            Token::InstanceOf => "instanceof".into(),
            Token::In => "in".into(),
            Token::New => "new".into(),
            Token::Return => "return".into(),
            Token::Switch => "switch".into(),
            Token::This => "this".into(),
            Token::Throw => "throw".into(),
            Token::Try => "try".into(),
            Token::TypeOf => "typeof".into(),
            Token::Var => "var".into(),
            Token::Void => "void".into(),
            Token::While => "while".into(),
            Token::With => "with".into(),
            Token::Export => "export".into(),
            Token::Import => "import".into(),
            Token::Super => "super".into(),
            Token::Enum => "enum".into(),
            Token::Implements => "implements".into(),
            Token::Interface => "interface".into(),
            Token::Package => "package".into(),
            Token::Private => "private".into(),
            Token::Protected => "protected".into(),
            Token::Public => "public".into(),
            Token::Static => "static".into(),
            Token::Yield => "yield".into(),
            Token::Let => "let".into(),
            Token::Eval => "eval".into(),
            Token::Arguments => "arguments".into(),
            Token::Of => "of".into()
        }
    }
}

impl Token {
    
}

pub(crate) fn literal<I>() -> impl Parser<Input = I, Output = TokenData>
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

pub(crate) fn reserved<I>() -> impl Parser<Input = I, Output = TokenData>
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
    ]).map(|t| TokenData::Keyword(Token::from(t.to_owned())))
}

pub(crate) fn future_reserved<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(string("export")),
        try(string("import")),
        try(string("super")),
        try(string("enum")),
    )).map(|t| TokenData::Keyword(Token::from(t)))
}

pub(crate) fn strict_mode_reserved<I>() -> impl Parser<Input = I, Output = TokenData>
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
    )).map(|t| TokenData::Keyword(Token::from(t)))
}

pub(crate) fn restricted<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(string("eval")), try(string("arguments")))).map(|t| TokenData::Keyword(Token::from(t)))
}

#[cfg(test)]
mod test {
    use super::*;
    use tokens::token;
    use tokens::TokenData;
    #[test]
    fn future_reserved() {
        let keywords = ["enum", "export", "import", "super"];
        for keyword in keywords.iter() {
            let k = token().parse(keyword.clone()).unwrap();
            assert_eq!(k, (TokenData::keyword(*keyword), ""))
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
            assert_eq!(k, (TokenData::keyword(*keyword), ""));
        }
        match super::strict_mode_reserved().parse("junk") {
            Ok(k) => panic!("parsed junk as {:?}", k),
            Err(_) => (),
        }
    }

    #[test]
    fn restricted_reserved() {
        let k = token().parse("eval").unwrap();
        assert_eq!(k, (TokenData::keyword("eval"), ""));
        let k2 = token().parse("arguments").unwrap();
        assert_eq!(k2, (TokenData::keyword("arguments"), ""))
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
            assert_eq!(k, (TokenData::keyword(key), ""));
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
            assert_eq!(k, (TokenData::keyword(key), ""));
        }
    }
}