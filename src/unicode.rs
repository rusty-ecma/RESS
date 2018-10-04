use combine::*;
use combine::parser::{
    char::{char as c_char, hex_digit},
    item::satisfy,
    error::unexpected_any
};

use unic_ucd_ident::{is_id_continue, is_id_start};

pub(crate) fn escape_sequence<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        count(4, hex_digit())
    ).map(|hex: String| hex)
}

pub(crate) fn code_point_literal<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        c_char('{'),
        many(hex_digit()).then_partial(|s: &mut String| {
            if let Ok(num) = u32::from_str_radix(&s, 16) {
                if num <= 1114111 {
                    value(s.to_owned()).left()
                } else {
                    unexpected_any("unicode code point must be <= 1114111").right()
                }
            } else {
                unexpected_any("unicode code point must be <= valid number").right()
            }
        }),
        c_char('}'),
    )
    .map(|(_, num, _): (char, String, char)| format!("{{{}}}", num))

}

pub(crate) fn char_literal<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
        (
            c_char('\\'),
            c_char('u'),
            choice((code_point_literal(), escape_sequence()))
        ).map(|(_, _, sequence): (char, char, String)| format!(r#"\u{}"#, sequence))
}

pub(crate) fn id_start<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(is_id_start)
}

pub(crate) fn id_continue<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(is_id_continue)
}