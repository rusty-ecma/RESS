use combine::{
    choice,
    error::ParseError,
    many, many1, optional,
    parser::char::{char as c_char, digit, hex_digit, oct_digit},
    attempt, Parser, Stream,
    range::recognize,
};

use refs::tokens::{RefToken as Token, Number};

pub fn literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>
{
    choice((
        attempt(non_decimal()),
        attempt(decimal_literal()),
    )).map(Token::Numeric)
}

fn decimal_literal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>
{
    choice((
        attempt(full_decimal_literal()),
        attempt(no_leading_decimal())
    )).map(|_| Number::Dec)
}

fn full_decimal_literal<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>
{
    recognize((
        //any number of digits
        many1::<String, _>(digit()),
        //optionally followed by a . and any number of digits
        optional((c_char('.'), many::<String, _>(digit()))),
        //optionally followed by e|E and any number of digits
        optional(exponent()),
    ))
}

fn exponent<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>
{
    recognize((
        choice([c_char('e'), c_char('E')]),
        optional(choice([c_char('-'), c_char('+')])),
        many1::<String, _>(digit()),
    ))
}

fn no_leading_decimal<I>() -> impl Parser<Input = I, Output = I::Range>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>
{
    recognize((
        c_char('.'),
        many1::<String, _>(digit()),
        optional(exponent())
    ))
}

pub fn non_decimal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>
{
    choice((
        attempt(hex_literal()),
        attempt(octal_literal()),
        attempt(bin_literal())
    ))
}

fn hex_literal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>
{
    recognize((
        c_char('0'),
        choice([c_char('x'), c_char('X')]),
        many1::<String, _>(hex_digit()),
    )).map(|_| Number::Hex)
}

fn bin_literal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>
{
    recognize((
        c_char('0'),
        choice([c_char('b'), c_char('B')]),
        many1::<String, _>(choice([c_char('1'), c_char('0')])),
    )).map(|_| Number::Bin)
}

fn octal_literal<I>() -> impl Parser<Input = I, Output = Number>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    combine::error::Info<char, <I as combine::StreamOnce>::Range>: std::convert::From<<I as combine::StreamOnce>::Range>
{
    recognize((
        c_char('0'),
        choice([c_char('o'), c_char('O')]),
        many1::<String, _>(oct_digit()),
    )).map(|_| Number::Oct)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn numbers_ref() {
        let numbers = [
            "1",
            "123.345",
            "123.345e11",
            ".99E23",
            "0x77afd",
            "0o7777",
            "0b010101001",
        ];

        for num in numbers.iter() {
            let result = literal().easy_parse(*num).unwrap();
            assert!(result.1.len() == 0);
        }
    }
}
