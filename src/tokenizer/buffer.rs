use std::char;
#[derive(Clone)]
pub struct JSBuffer<'a> {
    pub buffer: &'a [u8],
    pub idx: usize,
    pub len: usize,
}
const CONT_MASK: u8 = 0b0011_1111;
const TAG_CONT_U8: u8 = 0b1000_0000;
/// Re-implementation of
/// the std::str::Chars logic
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
        if x < 0xE0 {
            return char::from_u32(ch);
        }
        // [[x y z] w] case
        // 5th bit in 0xE0 .. 0xEF is always clear, so `init` is still valid
        let z = self.next_or_zero();
        let y_z = Self::utf8_acc_cont_byte((y & CONT_MASK) as u32, z);
        ch = init << 12 | y_z;
        if x < 0xF0 {
            return char::from_u32(ch);
        }
        // [x y z w] case
        // use only the lower 3 bits of `init`
        let w = self.next_or_zero();
        ch = (init & 7) << 18 | Self::utf8_acc_cont_byte(y_z, w);
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
            0
        } else {
            let old = self.idx;
            self.idx += 1;
            self.buffer[old]
        }
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
    #[allow(clippy::all)]
    fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 {
        (ch << 6) | (byte & CONT_MASK) as u32
    }
    #[inline]
    #[allow(clippy::all)]
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
            buffer,
            idx: 0,
            len: buffer.len(),
        }
    }
    /// Check if the buffer is at or past the
    /// end of the bytes provided
    #[inline]
    pub fn at_end(&self) -> bool {
        self.idx >= self.len
    }

    /// Check if the next few bytes match the provided bytes
    #[inline]
    pub fn look_ahead_matches(&self, s: &[u8]) -> bool {
        let len = s.len();
        let end = self.idx + len;
        if end > self.len {
            return false;
        }
        end <= self.len && &self.buffer[self.idx..end] == s
    }
    /// Check if the next byte matches a single byte provided
    #[inline]
    pub fn look_ahead_byte_matches(&self, b: u8) -> bool {
        if self.at_end() {
            false
        } else {
            self.buffer[self.idx] == b
        }
    }

    /// Skip the number of characters provided returning the number of bytes skipped
    /// note: these are full unicode characters, not just bytes
    #[inline]
    pub fn skip(&mut self, count: usize) -> usize {
        let mut ret = 0;
        for _ in 0..count {
            let len = self.next_char().map(|c| c.len_utf8()).unwrap_or(0);
            ret += len;
        }
        ret
    }
    /// Skip a single byte
    /// note: this can cause the buffer to become unaligned
    /// be sure to always know the character you are skipping
    /// is 1 byte wide or use `skip` instead when unsure
    #[inline]
    pub fn skip_bytes(&mut self, count: usize) {
        self.idx += count;
    }

    /// check if current char is a valid
    /// js whitespace character
    pub fn at_whitespace(&mut self) -> bool {
        if self.at_end() {
            return false;
        }
        self.buffer[self.idx] == 9 //\t
            || self.buffer[self.idx] == 10 // \n
            || self.buffer[self.idx] == 11 // \u{000b}
            || self.buffer[self.idx] == 12 // \f
            || self.buffer[self.idx] == 13 // \r
            || self.buffer[self.idx] == 32 // ' '
            || (self.buffer[self.idx] == 194 && self.idx + 1 < self.len && self.buffer[self.idx+1] == 160)
            || (self.buffer[self.idx] >= 226 && self.buffer[self.idx] <= 239 && self.len > self.idx + 2 && {
                match &self.buffer[self.idx..self.idx+3] {
                    [239, 187, 191] //"\u{feff}",
                    | [226, 128, 168] //"\u{2028}",
                    | [226, 128, 169] //"\u{2029}",
                    | [226, 128, 128] //"\u{2000}",
                    | [226, 128, 129] //"\u{2001}",
                    | [226, 128, 130] //"\u{2002}",
                    | [226, 128, 131] //"\u{2003}",
                    | [226, 128, 132] //"\u{2004}",
                    | [226, 128, 133] //"\u{2005}",
                    | [226, 128, 134] //"\u{2006}",
                    | [226, 128, 135] //"\u{2007}",
                    | [226, 128, 136] //"\u{2008}",
                    | [226, 128, 137] //"\u{2009}",
                    | [226, 128, 138] //"\u{200a}",
                    | [226, 128, 175] //"\u{202f}",
                    | [226, 129, 159] //"\u{205f}",
                    | [227, 128, 128] => true,  //"\u{3000}",
                    _ => false,
                }
            } )
    }
    /// Check of the look ahead character is
    /// a valid js new line character
    #[inline]
    pub fn at_new_line(&mut self) -> bool {
        if self.at_end() {
            return false;
        }
        let byte = self.buffer[self.idx];
        if byte < 10 {
            false
        } else if byte == 10 {
            true
        } else if byte < 13 {
            false
        } else if byte == 13 {
            true
        } else if byte < 226 {
            false
        } else if byte == 226 {
            self.look_ahead_matches("\u{2028}".as_bytes())
                || self.look_ahead_matches("\u{2029}".as_bytes())
        } else {
            false
        }
    }
    /// check if the look ahead character is `0` or `1`
    #[inline]
    pub fn at_binary(&self) -> bool {
        if self.at_end() {
            return false;
        }
        self.buffer[self.idx] >= b'0' && self.buffer[self.idx] <= b'1'
    }
    /// check if the look ahead character is a number
    /// between `0` and `9`, inclusive
    #[inline]
    pub fn at_decimal(&self) -> bool {
        if self.at_end() {
            return false;
        }
        self.buffer[self.idx] >= b'0' && self.buffer[self.idx] <= b'9'
    }
    /// check if the look ahead character is a number
    /// between `0` and `7`, inclusive
    #[inline]
    pub fn at_octal(&self) -> bool {
        if self.at_end() {
            return false;
        }
        self.buffer[self.idx] >= b'0' && self.buffer[self.idx] <= b'7'
    }
    /// check if the look ahead character is a number
    /// between `0` and `9` or `a` and `f` or `A` and `F`, inclusive
    #[inline]
    pub fn at_hex(&self) -> bool {
        if self.at_end() {
            return false;
        }
        (self.buffer[self.idx] >= b'0' && self.buffer[self.idx] <= b'9')
            || (self.buffer[self.idx] >= b'a' && self.buffer[self.idx] <= b'f')
            || (self.buffer[self.idx] >= b'A' && self.buffer[self.idx] <= b'F')
    }
    /// Peek forward 1 char with out updating the
    /// `idx` to this new position.
    ///
    /// note: this will still cost the same amount
    /// of work as `next_char` but cleans up the
    /// book keeping for you
    #[inline]
    pub fn peek_char(&mut self) -> Option<char> {
        let ch = self.next_char()?;
        self.skip_back_bytes(ch.len_utf8());
        Some(ch)
    }
    /// Skip backwards a number of bytes
    /// note: this can cause the buffer to become unaligned
    /// be sure to always know the character you are skipping
    /// is [count] bytes wide or use `skip` instead when unsure
    /// the right width is skipped
    #[inline]
    pub fn skip_back_bytes(&mut self, count: usize) {
        self.idx -= count;
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
    fn ascii_chars() {
        let mut bytes = Vec::new();
        for i in 0..=255u8 {
            if i.is_ascii() {
                bytes.push(i);
            }
        }
        let mut buf = JSBuffer::new(&bytes);
        for &byte in &bytes {
            let ch = buf.next_char().unwrap();
            assert_eq!(ch, byte as char);
        }
    }
    #[test]
    fn non_ascii_chars() {
        let mut s = String::new();
        eprintln!("collecting u32 chars");
        for (i, v) in (0x7FF..=0x10FFFF).enumerate() {
            if let Some(ch) = char::from_u32(v) {
                s.push(ch);
            }
            if i % 100 == 0 {
                eprintln!("{}", (v as f32 / (0x10FFFF - 0x7FF) as f32) * 100.0);
            }
        }
        eprintln!("creating buffer");
        let mut buf = JSBuffer::new(s.as_bytes());
        for (i, c1) in s.char_indices() {
            let c2 = buf.next_char().unwrap();
            assert_eq!(
                c1, c2,
                "failed at character {}:\n{} vs {}\n{:08b}\n{:08b}",
                i, c1 as u32, c2 as u32, c1 as u32, c2 as u32
            );
        }
    }
    #[test]
    fn at_whitespace() {
        let whitespaces = &[
            9,  // \t
            10, // \n
            11, // \u{000b}
            12, // \f
            13, // \r
            32, // ' '
            194, 160, //\u{00A0}
            239, 187, 191, // \u{FEFF}
            226, 128, 168, // \u{2028}
            226, 128, 169, // \u{2029}
            226, 128, 128, // \u{2000}
            226, 128, 129, // \u{2001}
            226, 128, 130, // \u{2002}
            226, 128, 131, // \u{2003}
            226, 128, 132, // \u{2004}
            226, 128, 133, // \u{2005}
            226, 128, 134, // \u{2006}
            226, 128, 135, // \u{2007}
            226, 128, 136, // \u{2008}
            226, 128, 137, // \u{2009}
            226, 128, 138, // \u{200A}
            226, 128, 175, // \u{202F}
            226, 129, 159, // \u{205F}
            227, 128, 128, // \u{3000}
        ];
        let mut buf = JSBuffer::new(whitespaces);
        while !buf.at_end() {
            assert!(
                buf.at_whitespace(),
                "buffer was not at whitespace {}",
                buf.idx
            );
            buf.skip(1);
        }
    }
    #[test]
    fn at_oct_number() {
        let s = "012345678";
        let mut buf = JSBuffer::from(s);
        for _ in 0..8 {
            assert!(buf.at_octal());
            let _ = buf.next_char();
        }
        assert!(!buf.at_octal());
    }
    #[test]
    fn at_dec_number() {
        let s = "0123456789a";

        let mut buf = JSBuffer::from(s);
        for _ in 0..10 {
            assert!(buf.at_decimal());
            let _ = buf.next_char();
        }
        assert!(!buf.at_decimal());
    }
    #[test]
    fn check() {
        let s = "🦜🦡🐁kł둘";
        let mut b = JSBuffer::from(s);
        assert!(b.next_char().unwrap() == '🦜');
        assert!(b.next_char().unwrap() == '🦡');
        assert!(b.next_char().unwrap() == '🐁');
        assert!(b.next_char().unwrap() == 'k');
        assert!(b.next_char().unwrap() == 'ł');
        assert!(b.next_char().unwrap() == '둘');
        assert!(b.next_char().is_none());
        assert!(b.prev_char().unwrap() == '둘');
        assert!(b.prev_char().unwrap() == 'ł');
        assert!(b.prev_char().unwrap() == 'k');
        assert!(b.prev_char().unwrap() == '🐁');
        assert!(b.prev_char().unwrap() == '🦡');
        assert!(b.prev_char().unwrap() == '🦜');
        assert!(b.prev_char().is_none());
    }

    #[test]
    fn at_end() {
        let js = "'things and stuff'";
        let mut buf = JSBuffer::from(js);
        for (i, c) in js.char_indices() {
            assert!(c == buf.next_char().unwrap());
            if i < js.len() - 1 {
                assert!(!buf.at_end());
            }
        }
        assert!(buf.at_end());
    }

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
