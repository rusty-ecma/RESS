use combine::{
    choice,
    error::ParseError,
    not_followed_by,
    parser::char::{string, char as c_char},
    attempt, Parser, Stream,
    range::recognize,
};
use punct::Punct;
use refs::tokens::RefToken as Token;

pub fn punctuation<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(multi_punct()),
        attempt(single_punct())
    )).map(Token::Punct)
}

fn single_punct<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(normal_punct()),
        attempt(div_punct())
    ))
}

fn normal_punct<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(recognize(c_char('}'))).map(|_| Punct::CloseBrace),
        normal_punct_not_close_brace()
    ))
}

fn normal_punct_not_close_brace<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        open_brace(),
        open_paren(),
        close_paren(),
        period(),
        semi(),
        comma(),
        open_bracket(),
        close_bracket(),
        colon(),
        question(),
        tilde(),
        gt(),
        lt(),
        assign(),
        bang(),
        plus(),
        minus(),
        mul(),
        modulo(),
        bit_and(),
        pipe(),
        xor(),
    ))
}

fn div_punct<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('/')
        .skip(not_followed_by(c_char('*'))))
    .map(|_| Punct::ForwardSlash)
}

fn multi_punct<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        four_char_punct(),
        three_char_punct(),
        two_char_punct(),
    ))
}

fn four_char_punct<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    unsigned_rhs_assign().map(|_| Punct::UnsignedRightShiftAssign)
}

fn three_char_punct<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        spread(),
        attempt(strict_equals()),
        attempt(strict_not_equals()),
        attempt(unsigned_rhs()),
        attempt(lhs_assign()),
        attempt(rhs_assign()),
        attempt(exp_assign()),
    ))
}

fn two_char_punct<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(logical_and()),
        attempt(logical_or()),
        attempt(equal()),
        not_equal(),
        attempt(add_assign()),
        attempt(sub_assign()),
        attempt(mul_assign()),
        div_assign(),
        increment(),
        decrement(),
        attempt(lhs()),
        attempt(rhs()),
        and_assign(),
        or_assign(),
        xor_assign(),
        mod_assign(),
        leq(),
        geq(),
        fat_arrow(),
        exp(),
    ))
}

fn open_brace<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('{')).map(|_| Punct::OpenBrace)
}
fn open_paren<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('(')).map(|_| Punct::OpenParen)
}
fn close_paren<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char(')')).map(|_| Punct::CloseParen)
}
fn period<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('.')).map(|_| Punct::Period)
}
fn semi<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char(';')).map(|_| Punct::SemiColon)
}
fn comma<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char(',')).map(|_| Punct::Comma)
}
fn open_bracket<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('[')).map(|_| Punct::OpenBracket)
}
fn close_bracket<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char(']')).map(|_| Punct::CloseBracket)
}
fn colon<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char(':')).map(|_| Punct::Colon)
}
fn question<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('?')).map(|_| Punct::QuestionMark)
}
fn tilde<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('~')).map(|_| Punct::BitwiseNot)
}
fn gt<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('>')).map(|_| Punct::GreaterThan)
}
fn lt<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('<')).map(|_| Punct::LessThan)
}
fn assign<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('=')).map(|_| Punct::Assign)
}
fn bang<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('!')).map(|_| Punct::Not)
}
fn plus<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('+')).map(|_| Punct::Plus)
}
fn minus<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('-')).map(|_| Punct::Minus)
}
fn mul<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('*')).map(|_| Punct::Asterisk)
}
fn modulo<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('%')).map(|_| Punct::Modulo)
}
fn bit_and<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('&')).map(|_| Punct::And)
}
fn pipe<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('|')).map(|_| Punct::Pipe)
}
fn xor<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(c_char('^')).map(|_| Punct::Caret)
}

fn unsigned_rhs_assign<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string(">>>=")).map(|_| Punct::UnsignedRightShiftAssign)
}
fn spread<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("...")).map(|_| Punct::Spread)
}
fn strict_equals<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("===")).map(|_| Punct::StrictEquals)
}
fn strict_not_equals<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("!==")).map(|_| Punct::StrictNotEquals)
}
fn unsigned_rhs<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string(">>>")).map(|_| Punct::UnsignedRightShift)
}
fn lhs_assign<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("<<=")).map(|_| Punct::LeftShiftAssign)
}
fn rhs_assign<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string(">>=")).map(|_| Punct::RightShiftAssign)
}
fn exp_assign<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("**=")).map(|_| Punct::ExponentAssign)
}
fn logical_and<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("&&")).map(|_| Punct::LogicalAnd)
}
fn logical_or<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("||")).map(|_| Punct::LogicalOr)
}
fn equal<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("==")).map(|_| Punct::Equal)
}
fn not_equal<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("!=")).map(|_| Punct::NotEqual)
}
fn add_assign<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("+=")).map(|_| Punct::AddAssign)
}
fn sub_assign<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("-=")).map(|_| Punct::SubtractAssign)
}
fn mul_assign<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("*=")).map(|_| Punct::MultiplyAssign)
}
fn div_assign<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("/=")).map(|_| Punct::DivideAssign)
}
fn increment<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("++")).map(|_| Punct::Increment)
}
fn decrement<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("--")).map(|_| Punct::Decrement)
}
fn lhs<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("<<")).map(|_| Punct::LeftShift)
}
fn rhs<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string(">>")).map(|_| Punct::RightShift)
}
fn and_assign<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("&=")).map(|_| Punct::BitwiseAndAssign)
}
fn or_assign<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("|=")).map(|_| Punct::BitwiseOrAssign)
}
fn xor_assign<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("^=")).map(|_| Punct::BitwiseXOrAssign)
}
fn mod_assign<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("%=")).map(|_| Punct::ModuloAssign)
}
fn leq<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("<=")).map(|_| Punct::LessThanEqual)
}
fn geq<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string(">=")).map(|_| Punct::GreaterThanEqual)
}
fn fat_arrow<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("=>")).map(|_| Punct::FatArrow)
}
fn exp<I>() -> impl Parser<Input = I, Output = Punct>
where
    I: Stream<Item = char>,
    I: combine::RangeStreamOnce,
    <I as combine::StreamOnce>::Range: combine::stream::Range,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize(string("**")).map(|_| Punct::Exponent)
}