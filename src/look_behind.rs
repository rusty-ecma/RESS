use crate::tokenizer::RawKeyword;
use crate::tokens::Punct;
use std::fmt::Debug;
use std::rc::Rc;

/// A 2 element buffer of
/// MetaTokens, this will use a
/// "ring buffer"-esque scheme
/// for automatically overwriting
/// any element after 2
#[derive(Clone)]
pub struct LookBehind {
    list: [Option<MetaToken>; 3],
    pointer: u8,
}

impl Debug for LookBehind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entry(self.one())
            .entry(self.two())
            .entry(self.three())
            .finish()
    }
}

impl LookBehind {
    #[inline]
    pub const fn new() -> Self {
        Self {
            list: [None, None, None],
            pointer: 2, // force the first pointer value to be 0
        }
    }
    #[inline]
    pub fn push(&mut self, token: MetaToken) {
        self.pointer = wrapping_add(self.pointer, 1, 2);
        self.list[self.pointer as usize] = Some(token)
    }
    #[inline]
    pub fn one(&self) -> &Option<MetaToken> {
        &self.list[self.pointer as usize]
    }
    #[inline]
    pub fn two(&self) -> &Option<MetaToken> {
        let idx = wrapping_sub(self.pointer, 1, 2) as usize;
        &self.list[idx]
    }
    #[inline]
    pub fn three(&self) -> &Option<MetaToken> {
        let idx = wrapping_sub(self.pointer, 2, 2) as usize;
        &self.list[idx]
    }
}

#[inline]
pub fn wrapping_sub(lhs: u8, rhs: u8, max: u8) -> u8 {
    if lhs >= rhs {
        lhs - rhs
    } else {
        let diff = rhs - lhs;
        (max + 1) - diff
    }
}
#[inline]
pub fn wrapping_add(lhs: u8, rhs: u8, max: u8) -> u8 {
    let maybe = lhs + rhs;
    if maybe > max {
        let diff = maybe - max;
        diff.saturating_sub(1)
    } else {
        maybe
    }
}

/// Token classes needed for look behind
///
/// All variants will carry their line number
///
#[derive(Debug, Clone, Copy)]
pub enum MetaToken {
    Keyword(RawKeyword, u32),
    Punct(Punct),
    OpenParen(Paren),
    CloseParen(Paren),
    OpenBrace(Brace, u32),
    CloseBrace(Brace),
    Ident,
    Other,
}
#[derive(Debug, Clone, Copy)]
pub struct Paren {
    pub func_expr: bool,
    pub conditional: bool,
}
#[derive(Debug, Clone, Copy)]
pub struct Brace {
    pub is_block: bool,
    pub paren: Option<Paren>,
}

impl MetaToken {
    pub fn line_number(self) -> u32 {
        match self {
            MetaToken::Keyword(_, line) | MetaToken::OpenBrace(_, line) => line,
            _ => 0,
        }
    }
}

impl PartialEq for MetaToken {
    fn eq(&self, other: &MetaToken) -> bool {
        match (self, other) {
            (MetaToken::Keyword(lhs, _), MetaToken::Keyword(rhs, _)) => lhs == rhs,
            (MetaToken::Punct(lhs), MetaToken::Punct(rhs)) => lhs == rhs,
            (MetaToken::Ident, MetaToken::Ident) | (MetaToken::Other, MetaToken::Other) => true,
            _ => false,
        }
    }
}

impl<T> From<(&crate::Token<T>, u32)> for MetaToken {
    fn from((other, line): (&crate::Token<T>, u32)) -> Self {
        match other {
            crate::Token::Keyword(k) => MetaToken::Keyword(k.into(), line),
            crate::Token::Punct(p) => MetaToken::Punct(*p),
            crate::Token::Ident(_) => MetaToken::Ident,
            _ => MetaToken::Other,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OpenBrace {
    pub look_behind: LookBehind,
    pub parent: Option<Rc<OpenBrace>>,
}

#[derive(Debug, Clone)]
pub struct CloseBrace {
    pub open: Rc<OpenBrace>,
}

#[derive(Debug, Clone)]
pub struct CloseParen {
    pub open: LookBehind,
}

impl std::ops::Deref for OpenBrace {
    type Target = LookBehind;
    fn deref(&self) -> &Self::Target {
        &self.look_behind
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tokens::Punct;

    #[test]
    fn wrapping_collection() {
        let first = MetaToken::Other;
        let second = MetaToken::Ident;
        let third = MetaToken::Keyword(RawKeyword::Function, 1);
        let fourth = MetaToken::Punct(Punct::Ampersand);
        let fifth = MetaToken::Punct(Punct::Bang);
        let sixth = MetaToken::Punct(Punct::Caret);
        let seventh = MetaToken::Punct(Punct::Pipe);
        let eighth = MetaToken::Punct(Punct::Tilde);
        let mut l = LookBehind::new();
        l.push(first);
        test(&l, Some(first), None, None);
        l.push(second);
        test(&l, Some(second), Some(first), None);
        l.push(third);
        test(&l, Some(third), Some(second), Some(first));
        l.push(fourth);
        test(&l, Some(fourth), Some(third), Some(second));
        l.push(fifth);
        test(&l, Some(fifth), Some(fourth), Some(third));
        l.push(sixth);
        test(&l, Some(sixth), Some(fifth), Some(fourth));
        l.push(seventh);
        test(&l, Some(seventh), Some(sixth), Some(fifth));
        l.push(eighth);
        test(&l, Some(eighth), Some(seventh), Some(sixth));
    }

    fn test(
        l: &LookBehind,
        first: Option<MetaToken>,
        second: Option<MetaToken>,
        third: Option<MetaToken>,
    ) {
        println!("{:?}", l);
        assert_eq!(l.one(), &first, "one didn't match");
        assert_eq!(l.two(), &second, "two didn't match");
        assert_eq!(l.three(), &third, "three didn't match");
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
