use combine::{
    choice, eof,
    error::ParseError,
    many, not_followed_by,
    parser::char::{char as c_char, string},
    attempt, Parser, Stream,
    range::recognize,
};

use refs::{
    comments::comment,
    keywords::literal as keyword,
    numbers::literal as number,
    punct::punctuation,
    strings::{
        literal as string_lit,
        template_start,
    },
};
use keywords::Keyword;
use punct::Punct;
use unicode;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StringLit {
    Double,
    Single,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Comment {
    SingleLine,
    MultiLine,
    Html,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Number {
    Dec,
    Bin,
    Hex,
    Oct
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Template {
    NoSub,
    Head,
    Body,
    Tail,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RefToken {
    /// `true` of `false`
    Boolean(bool),
    /// The end of the file
    EoF,
    /// An identifier this will be either a variable name
    /// or a function/method name
    Ident,
    /// A word that has been reserved to not be used as an identifier
    Keyword(Keyword),
    /// A `null` literal value
    Null,
    /// A number, this includes integers (`1`), decimals (`0.1`),
    /// hex (`0x8f`), binary (`0b010011010`), and octal (`0o273`)
    Numeric(Number),
    /// A punctuation mark, this includes all mathematical operators
    /// logical operators and general syntax punctuation
    Punct(Punct),
    /// A string literal, either double or single quoted, the associated
    /// value will be the unquoted string
    String(StringLit),
    /// A regular expression literal.
    /// ```js
    /// let regex = /[a-zA-Z]+/g;
    /// ```
    RegEx,
    /// The string parts of a template string
    /// ```
    /// # extern crate ress;
    /// # use ress::{Scanner, Item, Token, Number, Template};
    /// # fn main() {
    /// let js = "`Things and stuff times ${10} equals ${100000000}... i think`";
    /// let mut s = Scanner::new(js);
    /// assert_eq!(s.next().unwrap().token,
    ///             Token::template_head("Things and stuff times "));
    /// assert_eq!(s.next().unwrap().token,
    ///             Token::numeric("10"));
    /// assert_eq!(s.next().unwrap().token,
    ///             Token::template_middle(" equals "));
    /// assert_eq!(s.next().unwrap().token,
    ///             Token::numeric("100000000"));
    /// assert_eq!(s.next().unwrap().token,
    ///             Token::template_tail("... i think"));
    /// # }
    /// ```
    Template(Template),
    /// A comment, the associated value will contain the raw comment
    /// This will capture both inline comments `// I am an inline comment`
    /// and multi-line comments
    /// ```js
    /// /*multi lines
    /// * comments
    /// */
    /// ```
    Comment(Comment),
}


pub fn token<I>() -> impl Parser<Input = I, Output = RefToken>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'static str>,
{
    choice((
        token_not_eof(),
        end_of_input(),
    ))
}

pub(crate) fn token_not_eof<I>() -> impl Parser<Input = I, Output = RefToken>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>,
    <I as combine::StreamOnce>::Range: std::convert::From<&'static str>,
{
    choice((
        comment(),
        boolean_literal(),
        attempt(keyword()),
        attempt(ident()),
        attempt(null_literal()),
        attempt(number()),
        attempt(string_lit()),
        attempt(punctuation()),
        attempt(template_start()),
    ))
}

pub fn boolean_literal<I>() -> impl Parser<Input = I, Output = RefToken>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(true_literal()),
        attempt(false_literal())
    )).map(RefToken::Boolean)
}


fn true_literal<I>() -> impl Parser<Input = I, Output = bool>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(
        string("true")
        .skip(not_followed_by(super::super::tokens::raw_ident_part()))
    ).map(|_| true)
}

pub(crate) fn raw_ident_part<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(
        choice((
            super::super::unicode::id_continue(),
            c_char('$'),
            c_char('\\').skip(c_char('u')),
            c_char('\u{200C}'),
            c_char('\u{200D}'),
        ))
    )
}

fn false_literal<I>() -> impl Parser<Input = I, Output = bool>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(
        string("false")
        .skip(not_followed_by(super::super::tokens::raw_ident_part()))
    ).map(|_| false)
}

pub fn null_literal<I>() -> impl Parser<Input = I, Output = RefToken>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(
        string("null")
            .skip(not_followed_by(super::super::tokens::raw_ident_part()))
    ).map(|_| RefToken::Null)
}

pub fn ident<I>() -> impl Parser<Input = I, Output = RefToken>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    std::string::String: std::iter::Extend<<I as combine::StreamOnce>::Range>,
{
    recognize((
        ident_start(),
        many::<String, _>(ident_part())
    )).map(|_| RefToken::Ident)
}




pub(crate) fn ident_part<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(
        choice((
            attempt(ident_start()),
            attempt(raw_ident_part()),
        ))
    )
}

fn ident_start<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(
        choice((
            attempt(unicode::id_start().map(|c: char| c.to_string())),
            attempt(c_char('$').map(|c: char| c.to_string())),
            attempt(c_char('_').map(|c: char| c.to_string())),
            attempt(unicode::char_literal()),
        ))
    )
}

pub(crate) fn end_of_input<I>() -> impl Parser<Input = I, Output = RefToken>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(
        eof()
    ).map(|_| RefToken::EoF)
}