use std::char;

pub struct JSBuffer<'a> {
    pub buffer: &'a [u8],
    pub idx: usize,
}
/// Re-implementation of
/// the std::str::Chars logic
const CONT_MASK: u8 = 0b0011_1111;
const TAG_CONT_U8: u8 = 0b1000_0000;
impl<'a> JSBuffer<'a> {
    #[inline]
    pub fn next_char(&mut self) -> Option<char> {
        if self.at_end() {
            return None;
        }
        let x = self.next_or_zero();
        if x < 128 {
            return Some(x as char);
        }

        // Multibyte case follows
        // Decode from a byte combination out of: [[[x y] z] w]
        // NOTE: Performance is sensitive to the exact formulation here
        let init = (x & (0x7F >> 2)) as u32;
        let y = self.next_or_zero();
        let mut ch = Self::utf8_acc_cont_byte(init, y);
        if x >= 0xE0 {
            // [[x y z] w] case
            // 5th bit in 0xE0 .. 0xEF is always clear, so `init` is still valid
            let z = self.next_or_zero();
            let y_z = Self::utf8_acc_cont_byte((y & CONT_MASK) as u32, z);
            ch = init << 12 | y_z;
            if x >= 0xF0 {
                // [x y z w] case
                // use only the lower 3 bits of `init`
                let w = self.next_or_zero();
                ch = (init & 7) << 18 | Self::utf8_acc_cont_byte(y_z, w);
            }
        }
        char::from_u32(ch)
    }
    #[inline]
    pub fn prev_char(&mut self) -> Option<char> {
        // Decode UTF-8
        if self.idx == 0 {
            return None;
        }
        let w = self.prev_or_zero();
        if w < 128 {
            return char::from_u32(w as u32);
        }

        // Multibyte case follows
        // Decode from a byte combination out of: [x [y [z w]]]
        let mut ch;
        let z = self.prev_or_zero();
        ch = Self::utf8_first_byte(z, 2);
        if Self::utf8_is_cont_byte(z) {
            let y = self.prev_or_zero();
            ch = Self::utf8_first_byte(y, 3);
            if Self::utf8_is_cont_byte(y) {
                let x = self.prev_or_zero();
                ch = Self::utf8_first_byte(x, 4);
                ch = Self::utf8_acc_cont_byte(ch, y);
            }
            ch = Self::utf8_acc_cont_byte(ch, z);
        }
        ch = Self::utf8_acc_cont_byte(ch, w);

        char::from_u32(ch)
    }
    #[inline]
    fn next_or_zero(&mut self) -> u8 {
        if self.at_end() {
            return 0;
        }
        let old = self.idx;
        self.idx += 1;
        self.buffer[old]
    }
    #[inline]
    fn prev_or_zero(&mut self) -> u8 {
        if self.idx < 1 {
            return 0;
        }
        self.idx = self.idx.saturating_sub(1);
        self.buffer[self.idx]
    }
    #[inline]
    fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 {
        (ch << 6) | (byte & CONT_MASK) as u32
    }
    #[inline]
    fn utf8_first_byte(byte: u8, width: u32) -> u32 {
        (byte & (0x7F >> width)) as u32
    }
    #[inline]
    fn utf8_is_cont_byte(byte: u8) -> bool {
        (byte & !CONT_MASK) == TAG_CONT_U8
    }
}

impl<'a> JSBuffer<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer: buffer,
            idx: 0,
        }
    }

    pub fn at_end(&self) -> bool {
        self.idx >= self.buffer.len()
    }
    // /// Check if the buffer has [count] chars
    // /// before it ends
    // pub fn has(&self, count: usize) -> bool {
    //     self.buffer.len() > self.idx.saturating_add(count)
    // }
    /// Check if the next few bytes match the provided bytes
    pub fn look_ahead_matches(&self, s: &[u8]) -> bool {
        let end = self.idx.saturating_add(s.len());
        end <= self.buffer.len() && &self.buffer[self.idx..end] == s
    }
    /// Skip the number of characters provided
    /// note: these are full unicode characters, not just bytes
    pub fn skip(&mut self, count: usize) {
        for _ in 0..count {
            let _ = self.next_char();
        }
    }
    // /// Skip characters until the provided bytes are found
    // /// or the end of the buffer. If no match is found
    // /// this will return false and reset the index to
    // /// the original value
    // pub fn skip_until(&mut self, s: &[u8]) -> bool {
    //     let current_idx = self.idx;
    //     let mut end = self.idx.saturating_add(s.len());
    //     let mut at_end = self.at_end();
    //     while !at_end
    //     && &self.buffer[self.idx..end] != s {
    //         self.idx = self.idx.saturating_add(1);
    //         end = end.saturating_add(1);
    //         at_end = self.at_end()
    //     }
    //     if at_end {
    //         self.idx = current_idx;
    //     }
    //     !at_end
    // }
    /// check if current char is a valid
    /// js whitespace character
    pub fn at_whitespace(&mut self) -> bool {
        if self.at_end() {
            return false;
        }
        self.buffer[self.idx] == 9
            || self.buffer[self.idx] == 10
            || self.buffer[self.idx] == 11
            || self.buffer[self.idx] == 12
            || self.buffer[self.idx] == 13
            || self.buffer[self.idx] == 32
            || {
                let c = if let Some(c) = self.next_char() {
                    let _ = self.prev_char();
                    c
                } else {
                    return false;
                };
                c == '\u{00A0}'
                    || c == '\u{FEFF}'
                    || c == '\u{2028}'
                    || c == '\u{2029}'
                    || match unic_ucd::category::GeneralCategory::of(c) {
                        unic_ucd::category::GeneralCategory::SpaceSeparator => true,
                        _ => false,
                    }
            }
    }
    #[inline]
    pub fn at_new_line(&mut self) -> bool {
        let c = if let Some(c) = self.next_char() {
            let _ = self.prev_char();
            c
        } else {
            return false;
        };
        self.look_ahead_matches(b"\r\n")
            || c == '\n'
            || c == '\r'
            || c == '\u{00A0}'
            || c == '\u{FEFF}'
    }
    #[inline]
    pub fn at_decimal(&self) -> bool {
        self.buffer[self.idx] > 47 && self.buffer[self.idx] < 58
    }
    #[inline]
    pub fn at_octal(&self) -> bool {
        self.buffer[self.idx] > 47 && self.buffer[self.idx] < 56
    }
}

impl<'a> From<&'a str> for JSBuffer<'a> {
    fn from(s: &'a str) -> JSBuffer {
        Self::new(s.as_bytes())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check() {
        let s = "🦜🦡🐁kł둘";
        let mut b = JSBuffer {
            buffer: s.as_bytes(),
            idx: 0,
        };
        assert!(b.next_char().unwrap() == '🦜');
        assert!(b.next_char().unwrap() == '🦡');
        assert!(b.next_char().unwrap() == '🐁');
        assert!(b.next_char().unwrap() == 'k');
        assert!(b.next_char().unwrap() == 'ł');
        assert!(b.next_char().unwrap() == '둘');

        assert!(b.prev_char().unwrap() == '둘');
        assert!(b.prev_char().unwrap() == 'ł');
        assert!(b.prev_char().unwrap() == 'k');
        assert!(b.prev_char().unwrap() == '🐁');
        assert!(b.prev_char().unwrap() == '🦡');
        assert!(b.prev_char().unwrap() == '🦜');
    }

    #[test]
    fn at_end() {
        let js = "'things and stuff'";
        let mut buf = JSBuffer::from(js);
        let mut i = 0;
        for c in js.chars() {
            assert!(c == buf.next_char().unwrap());
            i += 1;
            if i < js.len() - 1 {
                assert!(!buf.at_end());
            }
        }
        assert!(buf.at_end());
    }

    // #[test]
    // fn skip_until() {
    //     let js = "'things and stuff'";
    //     let mut buf = JSBuffer::from(js);
    //     assert_eq!(buf.next_char().unwrap(), '\'');
    //     buf.skip_until(&['\'' as u8]);
    //     assert_eq!(buf.next_char().unwrap(), '\'');
    //     assert!(buf.at_end());
    // }

    #[test]
    fn look_ahead_matches() {
        let js = r#""things and stuff""#;
        let mut buf = JSBuffer::from(js);
        for i in 0..js.len() {
            let c = &js[i..i + 1];
            assert!(buf.look_ahead_matches(c.as_bytes()));
            let _ = buf.next_char();
        }
    }
}
