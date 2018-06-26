extern crate combine;
use combine::{
    many, many1, Parser, sep_by, choice, optional,
    parser::char::{letter, spaces,char, string},
    error::ParseError,
    stream::Stream,
};

fn keyword<I>() -> impl Parser<Input = I, Output = String> 
where I: Stream<Item = char>,
I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    
    choice((
        tokens::false_token(),
        tokens::true_token(),
        tokens::null_token(),
        // string("break"),
        // string("continue"),
        // string("debugger"),
        // string("in"),
        // string("instanceof"),
        // string("delete"),
        // string("function"),
        // string("new"),
        // string("var"),
        // string("return"),
    )).map(|x| x.to_owned())
}

mod tokens {
    use combine::parser::char::string;
    use combine::Parser;
    use combine::error::ParseError;
    use combine::Stream;
    fn own(s: &str) -> String {
        s.to_owned()
    }
    pub fn false_token<I>() -> impl Parser<Input = I, Output = String> 
    where I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        string("false").map(own)
    }
    pub fn true_token<I>() -> impl Parser<Input = I, Output = String> 
    where I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        string("true").map(own)
    }
    pub fn null_token<I>() -> impl Parser<Input = I, Output = String> 
    where I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        string("null").map(own)
    }
}

pub fn word<I>() -> impl Parser<Input = I, Output = String> 
where I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>, 
{
    many1(letter())
}

pub fn words<I>() -> impl Parser<Input = I, Output = Vec<String>> 
where I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>, 
{
    sep_by(word(), spaces())
}

pub fn arguments<I>() -> impl Parser<Input = I, Output = Vec<String>> 
where I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>, 
{
    (
        char('('),
        many(argument()),
        char(')')

    ).map(|(_,a,_)| a)
}

fn argument<I>() -> impl Parser<Input = I, Output = String>
where I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>, 
{
    (
        optional(char(',')),
        word()
    ).map(|(_, w)| w)
}


// pub fn ident_start<I>() -> impl Parser<Input = I, Output = char>
// where I: Stream<Item = char>,
//     I::Error: ParseError<I::Item, I::Range, I::Position>,
// {
//     choice(
//         (
//             letter(),
//             char(|c| c == '$' || c == '_' || c == '\u{005C}')
//         )
//     )
// }

// fn ident<I>() -> impl Parser<Input = I, Output = String> 
// where I: Stream<Item = char>,
//     I::Error: ParseError<I::Item, I::Range, I::Position>,
// {
//     (
//         ident_start(), 
//         try(
//             many1(
//                 letter()
//             )
//         ).map(|(start, remaining): (char, String)| {
//             format("{}{}",start, remaining)
//         })
//     )
// }

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let r = super::keyword().parse("true").unwrap();
        println!("{:#?}", r);
    }
}