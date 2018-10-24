use combine::{
    choice, error::ParseError, not_followed_by, parser::char::string, attempt, Parser, Stream,
};
use tokens::{raw_ident_part, Token};
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
        }.into()
    }
}

impl Keyword {
    /// Is this keyword one of the future reserved words
    ///
    /// - enum
    /// - export
    /// - implements
    /// - super
    pub fn is_future_reserved(&self) -> bool {
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
    pub fn is_strict_reserved(&self) -> bool {
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
    pub fn is_reserved(&self) -> bool {
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
}
/// generate a parser that will return an instance of Token::Keyword on success
pub(crate) fn literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(future_reserved()),
        attempt(strict_mode_reserved()),
        attempt(reserved()),
    )).skip(not_followed_by(raw_ident_part()))
    .map(|t| t)
}
/// generate a parser that will return a Token::Keyword with in finds
/// one of the reserved keywords
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
pub(crate) fn reserved<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(reserved_a_to_d()),
        attempt(reserved_e_to_r()),
        attempt(reserved_s_to_z()),
    )).map(|k| Token::Keyword(k))
}

pub(crate) fn reserved_a_to_d<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(string("await").map(|_| Keyword::Await)),
        attempt(string("break").map(|_| Keyword::Break)),
        attempt(string("case").map(|_| Keyword::Case)),
        attempt(string("catch").map(|_| Keyword::Catch)),
        attempt(string("class").map(|_| Keyword::Class)),
        attempt(string("const").map(|_| Keyword::Const)),
        attempt(string("continue").map(|_| Keyword::Continue)),
        attempt(string("debugger").map(|_| Keyword::Debugger)),
        attempt(string("default").map(|_| Keyword::Default)), 
        attempt(string("delete").map(|_| Keyword::Delete)),
        attempt(string("do").map(|_| Keyword::Do)),
    ))
}

pub(crate) fn reserved_e_to_r<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{

    choice((
        attempt(string("else").map(|_| Keyword::Else)),
        attempt(string("finally").map(|_| Keyword::Finally)),
        attempt(string("for").map(|_| Keyword::For)),
        attempt(string("function").map(|_| Keyword::Function)),
        attempt(string("if").map(|_| Keyword::If)),
        attempt(string("instanceof").map(|_| Keyword::InstanceOf)),
        attempt(string("in").map(|_| Keyword::In)),
        attempt(string("new").map(|_| Keyword::New)),
        attempt(string("return").map(|_| Keyword::Return)),
    ))
}
pub(crate) fn reserved_s_to_z<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(string("switch").map(|_| Keyword::Switch)),
        attempt(string("this").map(|_| Keyword::This)),
        attempt(string("throw").map(|_| Keyword::Throw)),
        attempt(string("try").map(|_| Keyword::Try)),
        attempt(string("typeof").map(|_| Keyword::TypeOf)),
        attempt(string("var").map(|_| Keyword::Var)),
        attempt(string("void").map(|_| Keyword::Void)),
        attempt(string("while").map(|_| Keyword::While)),
        attempt(string("with").map(|_| Keyword::With)),
    ))
}
/// Generate a parser that will return an instance of Token::Keyword when one of the
/// future reserved words are found
///
/// ## Keywords
/// - export
/// - import
/// - super
/// - enum
pub(crate) fn future_reserved<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(string("export").map(|_| Keyword::Export)),
        attempt(string("import").map(|_| Keyword::Import)),
        attempt(string("super").map(|_| Keyword::Super)),
        attempt(string("enum").map(|_| Keyword::Enum)),
    )).map(|k| Token::Keyword(k))
}

/// Generate a parser that will return an instance of Token::Keyword when a
/// strict mode reserved word is found
///
/// ##Keywords
/// - implements
/// - interface
/// - package
/// - private
/// - protected
/// - public
/// - static
/// - yield
/// - let
pub(crate) fn strict_mode_reserved<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(string("implements").map(|_| Keyword::Implements)),
        attempt(string("interface").map(|_| Keyword::Interface)),
        attempt(string("package").map(|_| Keyword::Package)),
        attempt(string("private").map(|_| Keyword::Private)),
        attempt(string("protected").map(|_| Keyword::Protected)),
        attempt(string("public").map(|_| Keyword::Public)),
        attempt(string("static").map(|_| Keyword::Static)),
        attempt(string("yield").map(|_| Keyword::Yield)),
        attempt(string("let").map(|_| Keyword::Let)),
    )).map(|k| Token::Keyword(k))
}

#[cfg(test)]
mod test {
    use super::*;
    use tokens::{token, Token};
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
            let k = token().parse(keyword.clone()).unwrap();
            assert_eq!(k, (Token::keyword(*keyword), ""));
        }
        match super::strict_mode_reserved().parse("junk") {
            Ok(k) => panic!("parsed junk as {:?}", k),
            Err(_) => (),
        }
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

    proptest! {
        #[test]
        fn keyword_prop(s in r#"await|break|case|catch|class|const|continue|debugger|default|import|delete|do|else|enum|export|finally|for|function|if|in|implements|instanceof|interface|let|new|package|private|protected|public|static|return|super|switch|this|throw|try|typeof|var|void|while|with|yield"#) {
            let r = token().easy_parse(s.as_str()).unwrap();
            assert!(r.0.is_keyword() && r.0.matches_keyword_str(&s));
        }
    }
}
