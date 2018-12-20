use combine::{
    choice, error::ParseError,
    not_followed_by,
    parser::char::string,
    attempt,
    Parser,
    Stream,
    range::recognize,
};

use refs::tokens::{
    RefToken as Token,
    raw_ident_part,
};
// use tokens::raw_ident_part;
use keywords::Keyword;

/// generate a parser that will return an instance of Token::Keyword on success
pub fn literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>,
{
    choice((
        future_reserved(),
        strict_mode_reserved(),
        reserved(),
    )).skip(not_followed_by(raw_ident_part()))
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
{
    choice((
        reserved_a_to_e(),
        reserved_f_to_r(),
        reserved_s_to_z(),
    ))
}

pub(crate) fn reserved_a_to_e<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
        choice((
            attempt(_await()),
            attempt(_break()),
            attempt(_case()),
            attempt(_catch()),
            attempt(_class()),
            attempt(_const()),
            attempt(_continue()),
            attempt(_debugger()),
            attempt(_default()),
            attempt(_delete()),
            attempt(_do()),
            attempt(_else()),
        ))
}

pub(crate) fn reserved_f_to_r<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(_finally()),
        attempt(_for()),
        attempt(_function()),
        attempt(_if()),
        attempt(_instanceof()),
        attempt(_in()),
        attempt(_new()),
        attempt(_return()),
    ))
}

pub(crate) fn reserved_s_to_z<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(switch()),
        attempt(this()),
        attempt(_throw()),
        attempt(_try()),
        attempt(_typeof()),
        attempt(var()),
        attempt(_void()),
        attempt(_while()),
        attempt(with()),
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
{
    choice((
        attempt(_implements()),
        attempt(_interface()),
        attempt(_package()),
        attempt(_private()),
        attempt(_protected()),
        attempt(_public()),
        attempt(_static()),
        attempt(_yield()),
        attempt(_let()),
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
{
    choice((
        attempt(export()),
        attempt(import()),
        attempt(_super()),
        attempt(_enum()),
    ))
}

fn _await<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("await")).map(|_| Keyword::Await)
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
    recognize(
        string("export")
    ).map(|_| Keyword::Export)
}

pub(crate) fn import<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(
        string("import")
    ).map(|_| Keyword::Import)
}

pub(crate) fn _super<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(
        string("super")
    ).map(|_| Keyword::Super)
}

pub(crate) fn _enum<I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(
        string("enum")
    ).map(|_| Keyword::Enum)
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