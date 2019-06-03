use combine::{
    attempt, choice,
    error::ParseError,
    not_followed_by,
    parser::char::{char as c_char, string},
    Parser, Stream,
};
use tokens::Token;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Punct {
    And,
    Assign,
    Asterisk,
    BitwiseNot,
    Caret,
    CloseBrace,
    CloseBracket,
    CloseParen,
    Colon,
    Comma,
    ForwardSlash,
    GreaterThan,
    LessThan,
    Minus,
    Modulo,
    Not,
    OpenBrace,
    OpenBracket,
    OpenParen,
    Period,
    Pipe,
    Plus,
    QuestionMark,
    SemiColon,
    Spread,
    UnsignedRightShiftAssign,
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
    Private,
}

impl<'a> ::std::convert::TryFrom<&'a str> for Punct {
    type Error = String;
    fn try_from(s: &'a str) -> Result<Punct, Self::Error> {
        match s {
            "{" => Ok(Punct::OpenBrace),
            "}" => Ok(Punct::CloseBrace),
            "(" => Ok(Punct::OpenParen),
            ")" => Ok(Punct::CloseParen),
            "." => Ok(Punct::Period),
            ";" => Ok(Punct::SemiColon),
            "," => Ok(Punct::Comma),
            "[" => Ok(Punct::OpenBracket),
            "]" => Ok(Punct::CloseBracket),
            ":" => Ok(Punct::Colon),
            "?" => Ok(Punct::QuestionMark),
            "~" => Ok(Punct::BitwiseNot),
            ">" => Ok(Punct::GreaterThan),
            "<" => Ok(Punct::LessThan),
            "=" => Ok(Punct::Assign),
            "!" => Ok(Punct::Not),
            "+" => Ok(Punct::Plus),
            "-" => Ok(Punct::Minus),
            "*" => Ok(Punct::Asterisk),
            "%" => Ok(Punct::Modulo),
            "|" => Ok(Punct::Pipe),
            "&" => Ok(Punct::And),
            "^" => Ok(Punct::Caret),
            "/" => Ok(Punct::ForwardSlash),
            ">>>=" => Ok(Punct::UnsignedRightShiftAssign),
            "..." => Ok(Punct::Spread),
            "===" => Ok(Punct::StrictEquals),
            "!==" => Ok(Punct::StrictNotEquals),
            ">>>" => Ok(Punct::UnsignedRightShift),
            "<<=" => Ok(Punct::LeftShiftAssign),
            ">>=" => Ok(Punct::RightShiftAssign),
            "**=" => Ok(Punct::ExponentAssign),
            "&&" => Ok(Punct::LogicalAnd),
            "||" => Ok(Punct::LogicalOr),
            "==" => Ok(Punct::Equal),
            "!=" => Ok(Punct::NotEqual),
            "+=" => Ok(Punct::AddAssign),
            "-=" => Ok(Punct::SubtractAssign),
            "*=" => Ok(Punct::MultiplyAssign),
            "/=" => Ok(Punct::DivideAssign),
            "++" => Ok(Punct::Increment),
            "--" => Ok(Punct::Decrement),
            "<<" => Ok(Punct::LeftShift),
            ">>" => Ok(Punct::RightShift),
            "&=" => Ok(Punct::BitwiseAndAssign),
            "|=" => Ok(Punct::BitwiseOrAssign),
            "^=" => Ok(Punct::BitwiseXOrAssign),
            "%=" => Ok(Punct::ModuloAssign),
            "=>" => Ok(Punct::FatArrow),
            ">=" => Ok(Punct::GreaterThanEqual),
            "<=" => Ok(Punct::LessThanEqual),
            "**" => Ok(Punct::Exponent),
            "#" => Ok(Punct::Private),
            _ => Err(format!("{} is not a known punct", s))
        }
    }
}

// impl<'a> From<&'a str> for Punct {
//     fn from(s: &'a str) -> Self {
//         match s {
//             "{" => Punct::OpenBrace,
//             "}" => Punct::CloseBrace,
//             "(" => Punct::OpenParen,
//             ")" => Punct::CloseParen,
//             "." => Punct::Period,
//             ";" => Punct::SemiColon,
//             "," => Punct::Comma,
//             "[" => Punct::OpenBracket,
//             "]" => Punct::CloseBracket,
//             ":" => Punct::Colon,
//             "?" => Punct::QuestionMark,
//             "~" => Punct::BitwiseNot,
//             ">" => Punct::GreaterThan,
//             "<" => Punct::LessThan,
//             "=" => Punct::Assign,
//             "!" => Punct::Not,
//             "+" => Punct::Plus,
//             "-" => Punct::Minus,
//             "*" => Punct::Asterisk,
//             "%" => Punct::Modulo,
//             "|" => Punct::Pipe,
//             "&" => Punct::And,
//             "^" => Punct::Caret,
//             "/" => Punct::ForwardSlash,
//             ">>>=" => Punct::UnsignedRightShiftAssign,
//             "..." => Punct::Spread,
//             "===" => Punct::StrictEquals,
//             "!==" => Punct::StrictNotEquals,
//             ">>>" => Punct::UnsignedRightShift,
//             "<<=" => Punct::LeftShiftAssign,
//             ">>=" => Punct::RightShiftAssign,
//             "**=" => Punct::ExponentAssign,
//             "&&" => Punct::LogicalAnd,
//             "||" => Punct::LogicalOr,
//             "==" => Punct::Equal,
//             "!=" => Punct::NotEqual,
//             "+=" => Punct::AddAssign,
//             "-=" => Punct::SubtractAssign,
//             "*=" => Punct::MultiplyAssign,
//             "/=" => Punct::DivideAssign,
//             "++" => Punct::Increment,
//             "--" => Punct::Decrement,
//             "<<" => Punct::LeftShift,
//             ">>" => Punct::RightShift,
//             "&=" => Punct::BitwiseAndAssign,
//             "|=" => Punct::BitwiseOrAssign,
//             "^=" => Punct::BitwiseXOrAssign,
//             "%=" => Punct::ModuloAssign,
//             "=>" => Punct::FatArrow,
//             ">=" => Punct::GreaterThanEqual,
//             "<=" => Punct::LessThanEqual,
//             "**" => Punct::Exponent,
//             "#" => Punct::Private,
//             _ => panic!("Unknown punctuation: {}", s),
//         }
//     }
// }

// impl From<String> for Punct {
//     fn from(s: String) -> Punct {
//         Self::try_from(s.as_str()).unwrap()
//     }
// }

impl ::std::string::ToString for Punct {
    fn to_string(&self) -> String {
        match self {
            Punct::OpenBrace => "{".into(),
            Punct::CloseBrace => "}".into(),
            Punct::OpenParen => "(".into(),
            Punct::CloseParen => ")".into(),
            Punct::Period => ".".into(),
            Punct::SemiColon => ";".into(),
            Punct::Comma => ",".into(),
            Punct::OpenBracket => "[".into(),
            Punct::CloseBracket => "]".into(),
            Punct::Colon => ":".into(),
            Punct::QuestionMark => "?".into(),
            Punct::BitwiseNot => "~".into(),
            Punct::GreaterThan => ">".into(),
            Punct::LessThan => "<".into(),
            Punct::Assign => "=".into(),
            Punct::Not => "!".into(),
            Punct::Plus => "+".into(),
            Punct::Minus => "-".into(),
            Punct::Asterisk => "*".into(),
            Punct::Modulo => "%".into(),
            Punct::Pipe => "|".into(),
            Punct::And => "&".into(),
            Punct::Caret => "^".into(),
            Punct::ForwardSlash => "/".into(),
            Punct::UnsignedRightShiftAssign => ">>>=".into(),
            Punct::Spread => "...".into(),
            Punct::StrictEquals => "===".into(),
            Punct::StrictNotEquals => "!==".into(),
            Punct::UnsignedRightShift => ">>>".into(),
            Punct::LeftShiftAssign => "<<=".into(),
            Punct::RightShiftAssign => ">>=".into(),
            Punct::ExponentAssign => "**=".into(),
            Punct::LogicalAnd => "&&".into(),
            Punct::LogicalOr => "||".into(),
            Punct::Equal => "==".into(),
            Punct::NotEqual => "!=".into(),
            Punct::AddAssign => "+=".into(),
            Punct::SubtractAssign => "-=".into(),
            Punct::MultiplyAssign => "*=".into(),
            Punct::DivideAssign => "/=".into(),
            Punct::Increment => "++".into(),
            Punct::Decrement => "--".into(),
            Punct::LeftShift => "<<".into(),
            Punct::RightShift => ">>".into(),
            Punct::BitwiseAndAssign => "&=".into(),
            Punct::BitwiseOrAssign => "|=".into(),
            Punct::BitwiseXOrAssign => "^=".into(),
            Punct::ModuloAssign => "%=".into(),
            Punct::FatArrow => "=>".into(),
            Punct::GreaterThanEqual => ">=".into(),
            Punct::LessThanEqual => "<=".into(),
            Punct::Exponent => "**".into(),
            Punct::Private => "#".into(),
        }
    }
}
pub fn punctuation<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    use std::convert::TryFrom;
    choice((attempt(multi_punct()), attempt(single_punct())))
        .map(|t: String| Token::Punct(Punct::try_from(t.as_str()).unwrap()))
}

fn single_punct<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((attempt(normal_punct()), attempt(div_punct()))).map(|c| c.to_string())
}

fn normal_punct<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(c_char('}')),
        attempt(normal_punct_not_close_brace()),
    ))
    .map(|c: char| c)
}

fn normal_punct_not_close_brace<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice([
        attempt(c_char('{')),
        attempt(c_char('(')),
        attempt(c_char(')')),
        attempt(c_char('.')),
        attempt(c_char(';')),
        attempt(c_char(',')),
        attempt(c_char('[')),
        attempt(c_char(']')),
        attempt(c_char(':')),
        attempt(c_char('?')),
        attempt(c_char('~')),
        attempt(c_char('>')),
        attempt(c_char('<')),
        attempt(c_char('=')),
        attempt(c_char('!')),
        attempt(c_char('+')),
        attempt(c_char('-')),
        attempt(c_char('*')),
        attempt(c_char('%')),
        attempt(c_char('&')),
        attempt(c_char('|')),
        attempt(c_char('^')),
        attempt(c_char('#')),
    ])
    .map(|c: char| c)
}

fn div_punct<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    c_char('/').skip(not_followed_by(c_char('*'))).map(|c| c)
}

fn multi_punct<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice([
        //4 char
        attempt(string(">>>=")),
        //3 char
        attempt(string("...")),
        attempt(string("===")),
        attempt(string("!==")),
        attempt(string(">>>")),
        attempt(string("<<=")),
        attempt(string(">>=")),
        attempt(string("**=")),
        //2 char
        attempt(string("&&")),
        attempt(string("||")),
        attempt(string("==")),
        attempt(string("!=")),
        attempt(string("+=")),
        attempt(string("-=")),
        attempt(string("*=")),
        attempt(string("/=")),
        attempt(string("++")),
        attempt(string("--")),
        attempt(string("<<")),
        attempt(string(">>")),
        attempt(string("&=")),
        attempt(string("|=")),
        attempt(string("^=")),
        attempt(string("%=")),
        attempt(string("<=")),
        attempt(string(">=")),
        attempt(string("=>")),
        attempt(string("**")),
    ])
    .map(|t| t.to_string())
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
            assert_eq!(t, (Token::punct(p), ""));
        }
        let multi = vec![
            ">>>=", //3 char
            "...", "===", "!==", ">>>", "<<=", ">>=", "**=", //2 char
            "&&", "||", "==", "!=", "+=", "-=", "*=", "/=", "++", "--", "<<", ">>", "&=", "|=",
            "^=", "%=", "<=", ">=", "=>", "**",
        ];
        for p in multi.clone() {
            let t = token().parse(p.clone()).unwrap();
            assert_eq!(t, (Token::punct(p), ""));
        }
        for p in single.iter().chain(multi.iter()) {
            let t = token().parse(p.clone()).unwrap();
            assert_eq!(t, (Token::punct(*p), ""))
        }
    }
}
