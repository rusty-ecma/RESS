use crate::tokenizer::RawToken;
use crate::tokens::{Keyword, Punct};

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
    pub fn push(&mut self, tok: &RawToken) {
        if self.pointer >= 2 {
            self.pointer = 0;
        } else {
            self.pointer += 1;
        }
        self.list[self.pointer as usize] = Some(tok.into());
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
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MetaToken {
    Keyword(Keyword<()>),
    Punct(Punct),
    Ident,
    Other,
}

fn switch_keyword<T>(key: &Keyword<T>) -> Keyword<()> {
    match key {
        Keyword::Await(_) => Keyword::Await(()),
        Keyword::Break(_) => Keyword::Break(()),
        Keyword::Case(_) => Keyword::Case(()),
        Keyword::Catch(_) => Keyword::Catch(()),
        Keyword::Class(_) => Keyword::Class(()),
        Keyword::Const(_) => Keyword::Const(()),
        Keyword::Continue(_) => Keyword::Continue(()),
        Keyword::Debugger(_) => Keyword::Debugger(()),
        Keyword::Default(_) => Keyword::Default(()),
        Keyword::Delete(_) => Keyword::Delete(()),
        Keyword::Do(_) => Keyword::Do(()),
        Keyword::Else(_) => Keyword::Else(()),
        Keyword::Enum(_) => Keyword::Enum(()),
        Keyword::Export(_) => Keyword::Export(()),
        Keyword::Finally(_) => Keyword::Finally(()),
        Keyword::For(_) => Keyword::For(()),
        Keyword::Function(_) => Keyword::Function(()),
        Keyword::If(_) => Keyword::If(()),
        Keyword::Implements(_) => Keyword::Implements(()),
        Keyword::Import(_) => Keyword::Import(()),
        Keyword::In(_) => Keyword::In(()),
        Keyword::InstanceOf(_) => Keyword::InstanceOf(()),
        Keyword::Interface(_) => Keyword::Interface(()),
        Keyword::Let(_) => Keyword::Let(()),
        Keyword::New(_) => Keyword::New(()),
        Keyword::Package(_) => Keyword::Package(()),
        Keyword::Private(_) => Keyword::Private(()),
        Keyword::Protected(_) => Keyword::Protected(()),
        Keyword::Public(_) => Keyword::Public(()),
        Keyword::Return(_) => Keyword::Return(()),
        Keyword::Static(_) => Keyword::Static(()),
        Keyword::Super(_) => Keyword::Super(()),
        Keyword::Switch(_) => Keyword::Switch(()),
        Keyword::This(_) => Keyword::This(()),
        Keyword::Throw(_) => Keyword::Throw(()),
        Keyword::Try(_) => Keyword::Try(()),
        Keyword::TypeOf(_) => Keyword::TypeOf(()),
        Keyword::Var(_) => Keyword::Var(()),
        Keyword::Void(_) => Keyword::Void(()),
        Keyword::While(_) => Keyword::While(()),
        Keyword::With(_) => Keyword::With(()),
        Keyword::Yield(_) => Keyword::Yield(()),
    }
}

impl From<&RawToken> for MetaToken {
    fn from(other: &RawToken) -> Self {
        match other {
            RawToken::Keyword(k) => MetaToken::Keyword(switch_keyword(k)),
            RawToken::Punct(p) => MetaToken::Punct(*p),
            RawToken::Ident => MetaToken::Ident,
            _ => MetaToken::Other,
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
        let third = RawToken::Keyword(Keyword::Function(()));
        let fourth = RawToken::Punct(Punct::Ampersand);
        let fifth = RawToken::Punct(Punct::Bang);
        let sixth = RawToken::Punct(Punct::Caret);
        let mut l = LookBehind::new();
        l.push(&first);
        test(&l, Some((&first).into()), None, None);
        l.push(&second);
        test(&l, Some((&second).into()), Some((&first).into()), None);
        l.push(&third);
        test(
            &l,
            Some((&third).into()),
            Some((&second).into()),
            Some((&first).into()),
        );
        l.push(&fourth);
        test(
            &l,
            Some((&fourth).into()),
            Some((&third).into()),
            Some((&second).into()),
        );
        l.push(&fifth);
        test(
            &l,
            Some((&fifth).into()),
            Some((&fourth).into()),
            Some((&third).into()),
        );
        l.push(&sixth);
        test(
            &l,
            Some((&sixth).into()),
            Some((&fifth).into()),
            Some((&fourth).into()),
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
