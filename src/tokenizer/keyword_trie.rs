use crate::tokenizer::{Tokenizer, RawKeyword};

impl<'a> Tokenizer<'a> {
    fn a_keywords(&mut self) -> Option<RawKeyword> {
        if self.eat_ch_or_escaped('w') {
            self.aw_keywords()
        } else {
            None
        }
    }
    fn aw_keywords(&mut self) -> Option<RawKeyword> {
        if self.eat_ch_or_escaped('a') {
            self.awa_keywords()
        } else {
            None
        }
    }
    fn awa_keywords(&mut self) -> Option<RawKeyword> {
        if self.eat_ch_or_escaped('i') {
            self.awai_keywords()
        } else {
            None
        }
    }

    fn awai_keywords(&mut self) -> Option<RawKeyword> {
        if self.eat_ch_or_escaped('t') {
            if let Some(c) = self.stream.next_char() {
                if Self::is_id_continue(c) {
                    None
                } else {
                    let _ = self.stream.prev_char();
                    Some(RawKeyword::Await)
                }
            } else {
                Some(RawKeyword::Await)
            }
        } else {
            None
        }
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
        let acc = 0u32;
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