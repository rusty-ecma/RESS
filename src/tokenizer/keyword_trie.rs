use crate::tokenizer::{Tokenizer, RawToken, RawKeyword};

impl<'a> Tokenizer<'a> {
    fn a_keywords(&mut self) -> Option<RawToken> {
        self.suffix_for_token("wait", RawToken::Keyword(RawKeyword::Await))
    }
    fn b_keywords(&mut self) -> Option<RawToken> {
        self.suffix_for_token("reak", RawToken::Keyword(RawKeyword::Break))
    }

    fn c_keywords(&mut self) -> Option<RawToken> {
        if self.eat_ch_or_escaped('a') {
            if self.eat_ch_or_escaped('s') {
                self.suffix_for_token("e", RawToken::Keyword(RawKeyword::Case))
            } else if self.eat_ch_or_escaped('t') {
                self.suffix_for_token("ch", RawToken::Keyword(RawKeyword::Catch))
            } else {
                None
            }
        } else if self.eat_ch_or_escaped('l') {
            self.suffix_for_token("ass", RawToken::Keyword(RawKeyword::Class))
        } else if self.eat_ch_or_escaped('o') && self.eat_ch_or_escaped('n') {
            if self.eat_ch_or_escaped('s') {
                self.suffix_for_token("t", RawToken::Keyword(RawKeyword::Const))
            } else if self.eat_ch_or_escaped('t') {
                self.suffix_for_token("inue", RawToken::Keyword(RawKeyword::Continue))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn d_keyword(&mut self) -> Option<RawToken> {
        unimplemented!()
    }

    fn e_keyword(&mut self) -> Option<RawToken> {
        unimplemented!()
    }

    fn f_keyword(&mut self) -> Option<RawToken> {
        unimplemented!()
    }

    fn i_keyword(&mut self) -> Option<RawToken> {
        unimplemented!()
    }

    fn l_keyword(&mut self) -> Option<RawToken> {
        unimplemented!()
    }

    fn n_keyword(&mut self) -> Option<RawToken> {
        unimplemented!()
    }

    fn p_keyword(&mut self) -> Option<RawToken> {
        unimplemented!()
    }

    fn r_keyword(&mut self) -> Option<RawToken> {
        unimplemented!()
    }

    fn s_keyword(&mut self) -> Option<RawToken> {
        unimplemented!()
    }

    fn t_keyword(&mut self) -> Option<RawToken> {
        unimplemented!()
    }

    fn v_keyword(&mut self) -> Option<RawToken> {
        unimplemented!()
    }

    fn w_keyword(&mut self) -> Option<RawToken> {
        unimplemented!()
    }

    fn y_keyword(&mut self) -> Option<RawToken> {
        unimplemented!()
    }
    
    fn suffix_for_token(&mut self, suffix: &str, tok: RawToken) -> Option<RawToken> {
        if self.eat_chs_or_escaped(suffix) {
            if self.at_ident_end() {
                Some(tok)
            } else {
                None
            }
        } else {
            None
        }
    }


    fn at_ident_end(&mut self) -> bool {
        if let Some(c) = self.stream.next_char() {
            if Self::is_id_continue(c) {
                false
            } else {
                let _ = self.stream.prev_char();
                true
            }
        } else {
            true
        }
    }

    fn eat_chs_or_escaped(&mut self, chars: &str) -> bool {
        for c in chars.chars() {
            if !self.eat_ch_or_escaped(c) {
                return false
            }
        }
        true
    }

    fn eat_ch_or_escaped(&mut self, ch: char) -> bool {
        if self.look_ahead_byte_matches(ch) {
            self.stream.skip(1);
            true
        } else if self.look_ahead_matches("\\u") {
            self.stream.skip(2);
            self.eat_escaped(ch)
        } else {
            false
        }
    }

    fn eat_escaped(&mut self, ch: char) -> bool {
        if self.look_ahead_byte_matches('{') {
            self.stream.skip(1);
            let escaped = self.eat_escaped_code_point();
            ch == escaped
        } else {
            let escaped = self.eat_escaped_hex4();
            ch == escaped
        }
    }
    fn eat_escaped_code_point(&mut self) -> char {
        let mut acc = 0u32;
        while let Some(ch) = self.stream.next_char() {
            if ch == '}' {
                if let Some(ch) = std::char::from_u32(acc) {
                    return ch;
                } else {
                    panic!("invalid escaped character")
                }
            }
            if let Some(n) = ch.to_digit(16) {
                acc = Self::shift_radix(acc, n, 16);
            } else {
                panic!("invalid hex in escaped codepoint")
            }
        }
        panic!("invalid escaped codepoint")
    }
    fn eat_escaped_hex4(&mut self) -> char {
        let mut acc = 0u32;
        for _ in 0..4 {
            if let Some(ch) = self.stream.next_char() {
                if let Some(n) = ch.to_digit(16) {
                    acc = Self::shift_radix(acc, n, 16);
                } else {
                    panic!("invalid hex in unicode escaped hex4")
                }
            } else {
                panic!("unexpected end of escaped unicode hex4")
            }
        }
        unimplemented!()
    }
    fn shift_radix(acc: u32, next: u32, radix: u32) -> u32 {
        (acc * radix) + next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn keyword_await() {
        
    }
}