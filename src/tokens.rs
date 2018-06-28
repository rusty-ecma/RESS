use combine::{
    choice,
    Parser,
    Stream,
    eof,
    try,
    parser::char::{
        string,
    },
    error::ParseError,
};

#[derive(Debug, PartialEq)]
pub enum Token {
    Boolean(bool),
    EoF,
    Ident(String),
    Keyword(String),
    Null,
    Numeric(f32),
    Punct(String),
    String(String),
    RegEx(String),
    Template(String),
}

pub fn boolean_literal<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        string("true"),
        string("false"),
    )).map(|t| Token::Boolean(t == "true"))
}

pub fn end_of_input<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    eof().map(|_| Token::EoF)
}

// pub fn ident<I>() -> impl Parser<Input = I, Output = Token>
//     where  I: Stream<Item = char>,
//         I::Error: ParseError<I::Item, I::Range, I::Position>,
// {
//     unimplemented!()
// }

pub fn keyword<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice([
        future_reserved(),
        // strict_mode_reserved(),

    ]).map(|k| k)
}

pub fn future_reserved<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(string("export")),
        string("import"),
        string("super"),
        string("enum"),
    ))
        .map(|t| Token::Keyword(t.to_string()))
}

pub fn strict_mode_reserved<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
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
    )).map(|t| Token::Keyword(t.to_owned()))
}

pub fn restricted<I>() -> impl Parser<Input = I, Output = Token>
    where  I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        string("eval"),
        string("arguments")
    )).map(|t| Token::Keyword(t.to_owned()))
}

pub mod keywords {
    use super::*;

    macro_rules! keyword {
        ($t:ident, $e:expr) =>(
        fn $t<I>() -> impl Parser<Input = I, Output = Token>
            where  I: Stream<Item = char>,
            I::Error: ParseError<I::Item, I::Range, I::Position>,
        {
            string($e).map(|t| Token::Keyword(t.to_owned()))
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn bool() {
        let t = super::boolean_literal().parse("true").unwrap();
        let f = super::boolean_literal().parse("false").unwrap();
        assert_eq!(t.0, Token::Boolean(true));
        assert_eq!(f.0, Token::Boolean(false));
    }

    #[test]
    fn eof() {
        let e = super::end_of_input().parse("").unwrap();
        assert_eq!(e.0, Token::EoF);
    }

    #[test]
    fn future_reserved() {
        let keywords = [
            "enum",
            "export",
            "import",
            "super",
        ];
        for keyword in keywords.iter() {
            super::future_reserved().parse(keyword.clone()).unwrap();
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
            let k = super::strict_mode_reserved().parse(keyword.clone()).unwrap();
            assert_eq!(k.0, Token::Keyword(keyword.to_string()));
        }
        match super::strict_mode_reserved().parse("junk") {
            Ok(k) => panic!("parsed junk as {:?}", k),
            Err(_) => (),
        }
    }

    #[test]
    fn restricted_reserved() {
        let k = super::restricted().parse("eval").unwrap();
        assert_eq!(k.0, Token::Keyword("eval".into()));
        let k2 = super::restricted().parse("arguments").unwrap();
        assert_eq!(k2.0, Token::Keyword("arguments".into()))
    }
}


