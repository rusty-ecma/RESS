use crate::tokenizer::RawToken;


#[derive(Clone, Debug)]
pub struct LookBehind {
    list: [Option<RawToken>; 3],
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
    pub fn push(&mut self, tok: RawToken) {
        if self.pointer >= 2 {
            self.pointer = 0;
        } else {
            self.pointer += 1;
        }
        self.list[self.pointer as usize] = Some(tok);
    }
    #[inline]
    pub fn last(&self) -> &Option<RawToken> {
        &self.list[self.pointer as usize]
    }
    #[inline]
    pub fn two(&self) -> &Option<RawToken> {
        if self.pointer == 0 {
            &self.list[2]
        } else {
            &self.list[(self.pointer - 1) as usize]
        }
    }
    #[inline]
    pub fn three(&self) -> &Option<RawToken> {
        if self.pointer == 2 {
            &self.list[0]
        } else {
            &self.list[(self.pointer + 1) as usize]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tokens::Punct;
    #[test]
    fn three() {
        let first = RawToken::EoF;
        let second = RawToken::Ident;
        let third = RawToken::Null;
        let mut l = LookBehind::new();
        l.push(first);
        test(&l, Some(first), None, None);
        l.push(second);
        test(&l, Some(second), Some(first), None);
        l.push(third);
        test(&l, Some(third), Some(second), Some(first));
    }

    #[test]
    fn six() {
        let first = RawToken::EoF;
        let second = RawToken::Ident;
        let third = RawToken::Null;
        let fourth = RawToken::Punct(Punct::Ampersand);
        let fifth = RawToken::Punct(Punct::Bang);
        let sixth = RawToken::Punct(Punct::Caret);
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
    }

    fn test(l: &LookBehind, first: Option<RawToken>, second: Option<RawToken>, third: Option<RawToken>) {
        println!("{:?}", l);
        assert_eq!(l.last(), &first);
        assert_eq!(l.two(), &second);
        assert_eq!(l.three(), &third);
    }
}