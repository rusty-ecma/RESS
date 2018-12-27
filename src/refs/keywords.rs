use combine::{
    attempt, choice, error::ParseError, not_followed_by, range::range, Parser, RangeStream, Stream,
};

use keywords::Keyword;
use refs::tokens::RefToken as Token;
use tokens::raw_ident_part;

/// generate a parser that will return an instance of Token::Keyword on success
pub fn literal<'a, I>() -> impl Parser<Input = I, Output = Token>
where
    I: RangeStream<Item = char, Range = &'a str>,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
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
pub(crate) fn reserved<'a, I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    choice((reserved_a_to_d(), reserved_e_to_r(), reserved_s_to_z()))
}

pub(crate) fn reserved_a_to_d<'a, I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
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

pub(crate) fn reserved_e_to_r<'a, I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
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

pub(crate) fn reserved_s_to_z<'a, I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
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
pub(crate) fn strict_mode_reserved<'a, I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
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
pub(crate) fn future_reserved<'a, I>() -> impl Parser<Input = I, Output = Keyword>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'a str>,
{
    choice((
        attempt(range("export".into()).map(|_| Keyword::Export)),
        attempt(range("import".into()).map(|_| Keyword::Import)),
        attempt(range("super".into()).map(|_| Keyword::Super)),
        attempt(range("enum".into()).map(|_| Keyword::Enum)),
    ))
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
            let s = key.to_string();
            let s = s.as_str();
            let result = match literal().easy_parse(s) {
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
