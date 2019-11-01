use crate::tokenizer::{RawToken, RawKeyword};
use crate::tokens::Punct;

#[derive(Clone, Debug)]
pub struct LookBehind {
    list: [Option<MetaToken>; 2],
    pointer: u8,
}

impl LookBehind {
    #[inline]
    pub const fn new() -> Self {
        Self {
            list: [None, None],
            pointer: 1, // force the first pointer value to be 0
        }
    }
    #[inline]
    pub fn push(&mut self, tok: &RawToken, line: u32) {
        self.push_close((tok, line).into());
    }
    #[inline]
    pub fn push_close(&mut self, token: MetaToken) {
        self.pointer = wrapping_add(self.pointer, 1, 1);
        self.list[self.pointer as usize] = Some(token)
    }
    #[inline]
    pub fn one(&self) -> &Option<MetaToken> {
        &self.list[self.pointer as usize]
    }
    #[inline]
    pub fn two(&self) -> &Option<MetaToken> {
        let idx = wrapping_sub(self.pointer, 1, 1) as usize;
        &self.list[idx]
    }
}

#[inline]
pub fn wrapping_sub(lhs: u8, rhs: u8, max: u8) -> u8 {
    if lhs >= rhs {
        lhs - rhs
    } else {
        let diff = rhs - lhs;
        let maybe = (max + 1) - diff;
        maybe
    }
}
#[inline]
pub fn wrapping_add(lhs: u8, rhs: u8, max: u8) -> u8 {
    let maybe = lhs + rhs;
    if maybe > max {
        let diff = maybe - max;
        0 + (diff.saturating_sub(1))
    } else {
        maybe
    }
}

/// Token classes needed for look behind
/// this enum will carry it's line number
#[derive(Debug, Clone)]
pub enum MetaToken {
    Keyword(RawKeyword, u32),
    Punct(Punct, u32),
    CloseParen(Box<LookBehind>, u32),
    CloseBrace(Box<LookBehind>, u32),
    Ident(u32),
    Other(u32),
}

impl MetaToken {
    pub fn line_number(&self) -> u32 {
        match self {
            MetaToken::Keyword(_, line)
            | MetaToken::Punct(_, line)
            | MetaToken::CloseParen(_, line)
            | MetaToken::CloseBrace(_, line)
            | MetaToken::Ident(line)
            | MetaToken::Other(line) => *line
        }
    }
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
        let seventh = RawToken::Punct(Punct::Pipe);
        let eighth = RawToken::Punct(Punct::Tilde);
        let mut l = LookBehind::new();
        l.push(&first, 1);
        test(&l, Some((&first, 1).into()), None);
        l.push(&second, 1);
        test(&l, Some((&second, 1).into()), Some((&first, 1).into()));
        l.push(&third, 1);
        test(
            &l,
            Some((&third, 1).into()),
            Some((&second, 1).into()),
        );
        l.push(&fourth, 1);
        test(
            &l,
            Some((&fourth, 1).into()),
            Some((&third, 1).into()),
        );
        l.push(&fifth, 1);
        test(
            &l,
            Some((&fifth, 1).into()),
            Some((&fourth, 1).into()),
        );
        l.push(&sixth, 1);
        test(
            &l,
            Some((&sixth, 1).into()),
            Some((&fifth, 1).into()),
        );
        l.push(&seventh, 1);
        test(
            &l,
            Some((&seventh, 1).into()),
            Some((&sixth, 1).into()),
        );
        l.push(&eighth, 1);
        test(
            &l,
            Some((&eighth, 1).into()),
            Some((&seventh, 1).into()),
        );
    }

    fn test(
        l: &LookBehind,
        first: Option<MetaToken>,
        second: Option<MetaToken>,
    ) {
        println!("{:?}", l);
        assert_eq!(l.one(), &first, "one didn't match");
        assert_eq!(l.two(), &second, "two didn't match");
    }

    #[test]
    fn wrapping() {
        assert_eq!(wrapping_sub(4, 1, 4), 3);
        assert_eq!(wrapping_sub(1, 1, 4), 0);
        assert_eq!(wrapping_sub(0, 1, 4), 4);
        assert_eq!(wrapping_add(0, 1, 4), 1);
        assert_eq!(wrapping_add(4, 1, 4), 0);
        assert_eq!(wrapping_add(0, 6, 4), 1)
        
    }
}
