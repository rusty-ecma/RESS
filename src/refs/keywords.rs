use combine::{
    attempt, choice,
    error::ParseError,
    not_followed_by,
    parser::char::string,
    range::{range, recognize},
    Parser, Stream,
};

use refs::tokens::{raw_ident_part, RefToken as Token};
// use tokens::raw_ident_part;
use keywords::Keyword;

/// generate a parser that will return an instance of Token::Keyword on success
pub fn literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>:
        std::convert::From<<I as combine::StreamOnce>::Range>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'static str>,
{
    choice((future_reserved(), strict_mode_reserved(), reserved()))
        .skip(not_followed_by(raw_ident_part()))
        .map(Token::Keyword)
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
pub(crate) fn reserved<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'static str>,
{
    choice((reserved_a_to_d(), reserved_e_to_r(), reserved_s_to_z()))
}

pub(crate) fn reserved_a_to_d<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'static str>,
{
    choice((
        attempt(range("await".into()).map(|_| Keyword::Await)),
        attempt(range("break".into()).map(|_| Keyword::Break)),
        attempt(range("case".into()).map(|_| Keyword::Case)),
        attempt(range("catch".into()).map(|_| Keyword::Catch)),
        attempt(range("class".into()).map(|_| Keyword::Class)),
        attempt(range("const".into()).map(|_| Keyword::Const)),
        attempt(range("continue".into()).map(|_| Keyword::Continue)),
        attempt(range("debugger".into()).map(|_| Keyword::Debugger)),
        attempt(range("default".into()).map(|_| Keyword::Default)),
        attempt(range("delete".into()).map(|_| Keyword::Delete)),
        attempt(range("do".into()).map(|_| Keyword::Do)),
    ))
}

pub(crate) fn reserved_e_to_r<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'static str>,
{
    choice((
        attempt(range("else".into()).map(|_| Keyword::Else)),
        attempt(range("finally".into()).map(|_| Keyword::Finally)),
        attempt(range("for".into()).map(|_| Keyword::For)),
        attempt(range("function".into()).map(|_| Keyword::Function)),
        attempt(range("if".into()).map(|_| Keyword::If)),
        attempt(range("instanceof".into()).map(|_| Keyword::InstanceOf)),
        attempt(range("in".into()).map(|_| Keyword::In)),
        attempt(range("new".into()).map(|_| Keyword::New)),
        attempt(range("return".into()).map(|_| Keyword::Return)),
    ))
}

pub(crate) fn reserved_s_to_z<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'static str>,
{
    choice((
        attempt(range("switch".into()).map(|_| Keyword::Switch)),
        attempt(range("this".into()).map(|_| Keyword::This)),
        attempt(range("throw".into()).map(|_| Keyword::Throw)),
        attempt(range("try".into()).map(|_| Keyword::Try)),
        attempt(range("typeof".into()).map(|_| Keyword::TypeOf)),
        attempt(range("var".into()).map(|_| Keyword::Var)),
        attempt(range("void".into()).map(|_| Keyword::Void)),
        attempt(range("while".into()).map(|_| Keyword::While)),
        attempt(range("with".into()).map(|_| Keyword::With)),
    ))
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
pub(crate) fn strict_mode_reserved<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'static str>,
{
    choice((
        attempt(range("implements".into()).map(|_| Keyword::Implements)),
        attempt(range("interface".into()).map(|_| Keyword::Interface)),
        attempt(range("package".into()).map(|_| Keyword::Package)),
        attempt(range("private".into()).map(|_| Keyword::Private)),
        attempt(range("protected".into()).map(|_| Keyword::Protected)),
        attempt(range("public".into()).map(|_| Keyword::Public)),
        attempt(range("static".into()).map(|_| Keyword::Static)),
        attempt(range("yield".into()).map(|_| Keyword::Yield)),
        attempt(range("let".into()).map(|_| Keyword::Let)),
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
pub(crate) fn future_reserved<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'static str>,
{
    choice((
        attempt(range("export".into()).map(|_| Keyword::Export)),
        attempt(range("import".into()).map(|_| Keyword::Import)),
        attempt(range("super".into()).map(|_| Keyword::Super)),
        attempt(range("enum".into()).map(|_| Keyword::Enum)),
    ))
}

fn _await<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'static str>,
{
    range("await".into()).map(|_| Keyword::Await)
}
fn _break<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("break")).map(|_| Keyword::Break)
}
fn _case<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("case")).map(|_| Keyword::Case)
}
fn _catch<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("catch")).map(|_| Keyword::Catch)
}
fn _class<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("class")).map(|_| Keyword::Class)
}
fn _const<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("const")).map(|_| Keyword::Const)
}
fn _continue<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("continue")).map(|_| Keyword::Continue)
}
fn _debugger<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("debugger")).map(|_| Keyword::Debugger)
}
fn _default<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("default")).map(|_| Keyword::Default)
}
fn _delete<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("delete")).map(|_| Keyword::Delete)
}
fn _do<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("do")).map(|_| Keyword::Do)
}

fn _else<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("else")).map(|_| Keyword::Else)
}
fn _finally<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("finally")).map(|_| Keyword::Finally)
}
fn _for<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("for")).map(|_| Keyword::For)
}
fn _function<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("function")).map(|_| Keyword::Function)
}
fn _if<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("if")).map(|_| Keyword::If)
}
fn _instanceof<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("instanceof")).map(|_| Keyword::InstanceOf)
}
fn _in<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("in")).map(|_| Keyword::In)
}
fn _new<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("new")).map(|_| Keyword::New)
}
fn _return<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("return")).map(|_| Keyword::Return)
}

fn switch<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("switch")).map(|_| Keyword::Switch)
}

fn this<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("this")).map(|_| Keyword::This)
}
fn _throw<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("throw")).map(|_| Keyword::Throw)
}
fn _try<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("try")).map(|_| Keyword::Try)
}
fn _typeof<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("typeof")).map(|_| Keyword::TypeOf)
}
fn var<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("var")).map(|_| Keyword::Var)
}
fn _void<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("void")).map(|_| Keyword::Void)
}
fn _while<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("while")).map(|_| Keyword::While)
}
fn with<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("with")).map(|_| Keyword::With)
}

pub(crate) fn export<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("export")).map(|_| Keyword::Export)
}

pub(crate) fn import<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("import")).map(|_| Keyword::Import)
}

pub(crate) fn _super<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("super")).map(|_| Keyword::Super)
}

pub(crate) fn _enum<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("enum")).map(|_| Keyword::Enum)
}

fn _implements<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("implements")).map(|_| Keyword::Implements)
}
fn _interface<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("interface")).map(|_| Keyword::Interface)
}
fn _package<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("package")).map(|_| Keyword::Package)
}
fn _private<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("private")).map(|_| Keyword::Private)
}
fn _protected<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("protected")).map(|_| Keyword::Protected)
}
fn _public<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("public")).map(|_| Keyword::Public)
}
fn _static<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("static")).map(|_| Keyword::Static)
}
fn _yield<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("yield")).map(|_| Keyword::Yield)
}
fn _let<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("let")).map(|_| Keyword::Let)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ref_keyword() {
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
            "enum",
            "export",
            "import",
            "super",
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
        for key in keywords.iter() {
            let result = match literal().easy_parse(*key) {
                Ok(pair) => pair,
                Err(e) => panic!("failed parsing {}\n{}", key, e),
            };
            if let Token::Keyword(k) = result.0 {
                assert_eq!(k.to_string(), *key);
            }
            assert_eq!(result.1.len(), 0);
        }
    }
}
