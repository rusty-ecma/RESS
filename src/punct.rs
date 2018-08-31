use combine::{
    choice, error::ParseError, not_followed_by, parser::char::{char as c_char, string}, try,
    Parser, Stream,
};
use tokens::Token;
#[derive(Debug, PartialEq, Clone)]
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
}

impl<'a> From<&'a str> for Punct {
    fn from(s: &'a str) -> Self {
        match s {
            "{" => Punct::OpenBrace,
            "}" => Punct::CloseBrace,
            "(" => Punct::OpenParen,
            ")" => Punct::CloseParen,
            "." => Punct::Period,
            ";" => Punct::SemiColon,
            "," => Punct::Comma,
            "[" => Punct::OpenBracket,
            "]" => Punct::CloseBracket,
            ":" => Punct::Colon,
            "?" => Punct::QuestionMark,
            "~" => Punct::BitwiseNot,
            ">" => Punct::GreaterThan,
            "<" => Punct::LessThan,
            "=" => Punct::Assign,
            "!" => Punct::Not,
            "+" => Punct::Plus,
            "-" => Punct::Minus,
            "*" => Punct::Asterisk,
            "%" => Punct::Modulo,
            "|" => Punct::Pipe,
            "&" => Punct::And,
            "^" => Punct::Caret,
            "/" => Punct::ForwardSlash,
            ">>>=" => Punct::UnsignedRightShiftAssign,
            "..." => Punct::Spread,
            "===" => Punct::StrictEquals,
            "!==" => Punct::StrictNotEquals,
            ">>>" => Punct::UnsignedRightShift,
            "<<=" => Punct::LeftShiftAssign,
            ">>=" => Punct::RightShiftAssign,
            "**=" => Punct::ExponentAssign,
            "&&" => Punct::LogicalAnd,
            "||" => Punct::LogicalOr,
            "==" => Punct::Equal,
            "!=" => Punct::NotEqual,
            "+=" => Punct::AddAssign,
            "-=" => Punct::SubtractAssign,
            "*=" => Punct::MultiplyAssign,
            "/=" => Punct::DivideAssign,
            "++" => Punct::Increment,
            "--" => Punct::Decrement,
            "<<" => Punct::LeftShift,
            ">>" => Punct::RightShift,
            "&=" => Punct::BitwiseAndAssign,
            "|=" => Punct::BitwiseOrAssign,
            "^=" => Punct::BitwiseXOrAssign,
            "%=" => Punct::ModuloAssign,
            "=>" => Punct::FatArrow,
            ">=" => Punct::GreaterThanEqual,
            "<=" => Punct::LessThanEqual,
            "**" => Punct::Exponent,
            _ => panic!("Unknown punctuation: {}", s),
        }
    }
}

impl From<String> for Punct {
    fn from(s: String) -> Punct {
        Self::from(s.as_str())
    }
}

impl ::std::string::ToString for Punct {
    fn to_string(&self) -> String {
        match self {
            &Punct::OpenBrace => "{".into(),
            &Punct::CloseBrace => "}".into(),
            &Punct::OpenParen => "(".into(),
            &Punct::CloseParen => ")".into(),
            &Punct::Period => ".".into(),
            &Punct::SemiColon => ";".into(),
            &Punct::Comma => ",".into(),
            &Punct::OpenBracket => "[".into(),
            &Punct::CloseBracket => "]".into(),
            &Punct::Colon => ":".into(),
            &Punct::QuestionMark => "?".into(),
            &Punct::BitwiseNot => "~".into(),
            &Punct::GreaterThan => ">".into(),
            &Punct::LessThan => "<".into(),
            &Punct::Assign => "=".into(),
            &Punct::Not => "!".into(),
            &Punct::Plus => "+".into(),
            &Punct::Minus => "-".into(),
            &Punct::Asterisk => "*".into(),
            &Punct::Modulo => "%".into(),
            &Punct::Pipe => "|".into(),
            &Punct::And => "&".into(),
            &Punct::Caret => "^".into(),
            &Punct::ForwardSlash => "/".into(),
            &Punct::UnsignedRightShiftAssign => ">>>=".into(),
            &Punct::Spread => "...".into(),
            &Punct::StrictEquals => "===".into(),
            &Punct::StrictNotEquals => "!==".into(),
            &Punct::UnsignedRightShift => ">>>".into(),
            &Punct::LeftShiftAssign => "<<=".into(),
            &Punct::RightShiftAssign => ">>=".into(),
            &Punct::ExponentAssign => "**=".into(),
            &Punct::LogicalAnd => "&&".into(),
            &Punct::LogicalOr => "||".into(),
            &Punct::Equal => "==".into(),
            &Punct::NotEqual => "!=".into(),
            &Punct::AddAssign => "+=".into(),
            &Punct::SubtractAssign => "-=".into(),
            &Punct::MultiplyAssign => "*=".into(),
            &Punct::DivideAssign => "/=".into(),
            &Punct::Increment => "++".into(),
            &Punct::Decrement => "--".into(),
            &Punct::LeftShift => "<<".into(),
            &Punct::RightShift => ">>".into(),
            &Punct::BitwiseAndAssign => "&=".into(),
            &Punct::BitwiseOrAssign => "|=".into(),
            &Punct::BitwiseXOrAssign => "^=".into(),
            &Punct::ModuloAssign => "%=".into(),
            &Punct::FatArrow => "=>".into(),
            &Punct::GreaterThanEqual => ">=".into(),
            &Punct::LessThanEqual => "<=".into(),
            &Punct::Exponent => "**".into(),
        }
    }
}
pub(crate) fn punctuation<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(multi_punct()), try(single_punct()))).map(|t: String| Token::Punct(Punct::from(t)))
}

fn single_punct<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(normal_punct()), try(div_punct()))).map(|c| c.to_string())
}

fn normal_punct<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(c_char('}')), try(normal_punct_not_close_brace()))).map(|c: char| c)
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
    c_char('/').skip(not_followed_by(c_char('*'))).map(|c| c)
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
            assert_eq!(t, (Token::punct(p), ""));
        }
        let multi = vec![
            ">>>=", //3 char
            "...",
            "===",
            "!==",
            ">>>",
            "<<=",
            ">>=",
            "**=", //2 char
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
            assert_eq!(t, (Token::punct(p), ""));
        }
        for p in single.iter().chain(multi.iter()) {
            let t = token().parse(p.clone()).unwrap();
            assert_eq!(t, (Token::punct(*p), ""))
        }
    }
}
