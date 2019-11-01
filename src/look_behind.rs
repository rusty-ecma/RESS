use crate::tokenizer::{RawToken, RawKeyword};
use crate::tokens::Punct;

#[derive(Clone, Debug)]
pub struct LookBehind {
    list: [Option<MetaToken>; 3],
    pointer: u8,
}

impl LookBehind {
    #[inline]
    pub const fn new() -> Self {
        Self {
            list: [None, None, None],
            pointer: 0,
        }
    }
    #[inline]
    pub fn push(&mut self, tok: &RawToken, line: u32) {
        if self.pointer >= 2 {
            self.pointer = 0;
        } else {
            self.pointer += 1;
        }
        self.list[self.pointer as usize] = Some((tok, line).into());
    }
    #[inline]
    pub fn last(&self) -> &Option<MetaToken> {
        &self.list[self.pointer as usize]
    }
    #[inline]
    pub fn two(&self) -> &Option<MetaToken> {
        if self.pointer == 0 {
            &self.list[2]
        } else {
            &self.list[(self.pointer - 1) as usize]
        }
    }
    #[inline]
    pub fn three(&self) -> &Option<MetaToken> {
        if self.pointer == 2 {
            &self.list[0]
        } else {
            &self.list[(self.pointer + 1) as usize]
        }
    }
}
/// Token classes needed for look behind
/// this enum will carry it's line number
#[derive(Debug, Clone, Copy)]
pub enum MetaToken {
    Keyword(RawKeyword, u32),
    Punct(Punct, u32),
    Ident(u32),
    Other(u32),
}

impl PartialEq for MetaToken {
    fn eq(&self, other: &MetaToken) -> bool {
        match (self, other) {
            (MetaToken::Keyword(lhs, _), MetaToken::Keyword(rhs, _)) => lhs == rhs,
            (MetaToken::Punct(lhs, _), MetaToken::Punct(rhs, _)) => lhs == rhs,
            (MetaToken::Ident(_), MetaToken::Ident(_)) 
            | (MetaToken::Other(_), MetaToken::Other(_)) => true,
            _ => false
        }
    }
}

impl From<(&RawToken, u32)> for MetaToken {
    fn from((other, line): (&RawToken, u32)) -> Self {
        match other {
            RawToken::Keyword(k) => MetaToken::Keyword(*k, line),
            RawToken::Punct(p) => MetaToken::Punct(*p, line),
            RawToken::Ident => MetaToken::Ident(line),
            _ => MetaToken::Other(line),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tokens::Punct;

    #[test]
    fn six() {
        let first = RawToken::EoF;
        let second = RawToken::Ident;
        let third = RawToken::Keyword(RawKeyword::Function);
        let fourth = RawToken::Punct(Punct::Ampersand);
        let fifth = RawToken::Punct(Punct::Bang);
        let sixth = RawToken::Punct(Punct::Caret);
        let mut l = LookBehind::new();
        l.push(&first, 1);
        test(&l, Some((&first, 1).into()), None, None);
        l.push(&second, 1);
        test(&l, Some((&second, 1).into()), Some((&first, 1).into()), None);
        l.push(&third, 1);
        test(
            &l,
            Some((&third, 1).into()),
            Some((&second, 1).into()),
            Some((&first, 1).into()),
        );
        l.push(&fourth, 1);
        test(
            &l,
            Some((&fourth, 1).into()),
            Some((&third, 1).into()),
            Some((&second, 1).into()),
        );
        l.push(&fifth, 1);
        test(
            &l,
            Some((&fifth, 1).into()),
            Some((&fourth, 1).into()),
            Some((&third, 1).into()),
        );
        l.push(&sixth, 1);
        test(
            &l,
            Some((&sixth, 1).into()),
            Some((&fifth, 1).into()),
            Some((&fourth, 1).into()),
        );
    }

    fn test(
        l: &LookBehind,
        first: Option<MetaToken>,
        second: Option<MetaToken>,
        third: Option<MetaToken>,
    ) {
        println!("{:?}", l);
        assert_eq!(l.last(), &first);
        assert_eq!(l.two(), &second);
        assert_eq!(l.three(), &third);
    }
}
