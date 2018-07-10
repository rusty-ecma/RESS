
use combine::{
    choice, error::ParseError, not_followed_by,
    parser::{
        char::{char as c_char, string},
    },
    try, Parser, Stream,
};
use tokens::TokenData;
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Period,
    SemiColon,
    Comma,
    OpenBracket,
    CloseBracket,
    Colon,
    QuestionMark,
    BitwiseNot,
    GreaterThan,
    LessThan,
    Assign,
    Not,
    Plus,
    Minus,
    Asterisk,
    Modulo,
    Pipe,
    And,
    Caret,
    ForwardSlash,
    UnsignedRightShiftAssign,
    Spread,
    StrictEquals,
    StrictNotEquals,
    UnsignedRightShift,
    LeftShiftAssign,
    RightShiftAssign,
    ExponentAssign,
    LogicalAnd,
    LogicalOr,
    Equal,
    NotEqual,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    Increment,
    Decrement,
    LeftShift,
    RightShift,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXOrAssign,
    ModuloAssign,
    FatArrow,
    GreaterThanEqual,
    LessThanEqual,
    Exponent,
}

impl<'a> From<&'a str> for Token {
    fn from(s: &'a str) -> Token {
        match s {
            "{" => Token::OpenBrace,
            "}" => Token::CloseBrace,
            "(" => Token::OpenParen,
            ")" => Token::CloseParen,
            "." => Token::Period,
            ";" => Token::SemiColon,
            "," => Token::Comma,
            "[" => Token::OpenBracket,
            "]" => Token::CloseBracket,
            ":" => Token::Colon,
            "?" => Token::QuestionMark,
            "~" => Token::BitwiseNot,
            ">" => Token::GreaterThan,
            "<" => Token::LessThan,
            "=" => Token::Assign,
            "!" => Token::Not,
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Asterisk,
            "%" => Token::Modulo,
            "|" => Token::Pipe,
            "&" => Token::And,
            "^" => Token::Caret,
            "/" => Token::ForwardSlash,
            ">>>=" => Token::UnsignedRightShiftAssign,
            "..." => Token::Spread,
            "===" => Token::StrictEquals,
            "!==" => Token::StrictNotEquals,
            ">>>" => Token::UnsignedRightShift,
            "<<=" => Token::LeftShiftAssign,
            ">>=" => Token::RightShiftAssign,
            "**=" => Token::ExponentAssign,
            "&&" => Token::LogicalAnd,
            "||" => Token::LogicalOr,
            "==" => Token::Equal,
            "!=" => Token::NotEqual,
            "+=" => Token::AddAssign,
            "-=" => Token::SubtractAssign,
            "*=" => Token::MultiplyAssign,
            "/=" => Token::DivideAssign,
            "++" => Token::Increment,
            "--" => Token::Decrement,
            "<<" => Token::LeftShift,
            ">>" => Token::RightShift,
            "&=" => Token::BitwiseAndAssign,
            "|=" => Token::BitwiseOrAssign,
            "^=" => Token::BitwiseXOrAssign,
            "%=" => Token::ModuloAssign,
            "=>" => Token::FatArrow,
            ">=" => Token::GreaterThanEqual,
            "<=" => Token::LessThanEqual,
            "**" => Token::Exponent,
            _ => panic!("Unknown punctuation: {}", s)
        }
    }
}

impl From<String> for Token {
    fn from(s: String) -> Token {
        Self::from(s.as_str())
    }
}

pub(crate) fn punctuation<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(multi_punct()), try(single_punct()))).map(|t: String| TokenData::Punct(Token::from(t)))
}
pub(crate) fn punctuation_not_close_brace<I>() -> impl Parser<Input = I, Output = TokenData>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(multi_punct()), try(single_punct_not_close_brace()))).map(|t: String| TokenData::Punct(Token::from(t)))
}

fn single_punct<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(normal_punct()), try(div_punct()))).map(|c| c.to_string())
}

fn single_punct_not_close_brace<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(normal_punct_not_close_brace()), try(div_punct()))).map(|c| c.to_string())
}



fn normal_punct<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(c_char('}')),
        try(normal_punct_not_close_brace()),
    )).map(|c: char| c)
}


fn normal_punct_not_close_brace<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice([
        try(c_char('{')),
        try(c_char('(')),
        try(c_char(')')),
        try(c_char('.')),
        try(c_char(';')),
        try(c_char(',')),
        try(c_char('[')),
        try(c_char(']')),
        try(c_char(':')),
        try(c_char('?')),
        try(c_char('~')),
        try(c_char('>')),
        try(c_char('<')),
        try(c_char('=')),
        try(c_char('!')),
        try(c_char('+')),
        try(c_char('-')),
        try(c_char('*')),
        try(c_char('%')),
        try(c_char('&')),
        try(c_char('|')),
        try(c_char('^')),
    ]).map(|c: char| c)
}

fn div_punct<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    c_char('/')
        .skip(not_followed_by(c_char('*')))
        .map(|c| c)
}

fn multi_punct<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice([
        //4 char
        try(string(">>>=")),
        //3 char
        try(string("...")),
        try(string("===")),
        try(string("!==")),
        try(string(">>>")),
        try(string("<<=")),
        try(string(">>=")),
        try(string("**=")),
        //2 char
        try(string("&&")),
        try(string("||")),
        try(string("==")),
        try(string("!=")),
        try(string("+=")),
        try(string("-=")),
        try(string("*=")),
        try(string("/=")),
        try(string("++")),
        try(string("--")),
        try(string("<<")),
        try(string(">>")),
        try(string("&=")),
        try(string("|=")),
        try(string("^=")),
        try(string("%=")),
        try(string("<=")),
        try(string(">=")),
        try(string("=>")),
        try(string("**")),
    ]).map(|t| t.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use tokens::token;
    #[test]
    fn punct() {
        let single = vec![
            "{", "}", "(", ")", ".", ";", ",", "[", "]", ":", "?", "~", ">", "<", "=", "!", "+",
            "-", "/", "*", "%", "&", "|", "^",
        ];
        for p in single.clone() {
            let t = token().parse(p.clone()).unwrap();
            assert_eq!(t, (TokenData::punct(p), ""));
        }
        let multi = vec![
            ">>>=",
            //3 char
            "...",
            "===",
            "!==",
            ">>>",
            "<<=",
            ">>=",
            "**=",
            //2 char
            "&&",
            "||",
            "==",
            "!=",
            "+=",
            "-=",
            "*=",
            "/=",
            "++",
            "--",
            "<<",
            ">>",
            "&=",
            "|=",
            "^=",
            "%=",
            "<=",
            ">=",
            "=>",
            "**",
        ];
        for p in multi.clone() {
            let t = token().parse(p.clone()).unwrap();
            assert_eq!(t, (TokenData::punct(p), ""));
        }
        for p in single.iter().chain(multi.iter()) {
            let t = token().parse(p.clone()).unwrap();
            assert_eq!(t, (TokenData::punct(*p), ""))
        }
    }
}