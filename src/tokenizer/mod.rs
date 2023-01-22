use crate::tokens::{CommentKind, NumberKind, Punct};
use crate::{is_line_term, OpenCurlyKind};
mod buffer;

mod tokens;
mod unicode;
pub use self::tokens::{RawKeyword, RawToken, StringKind, TemplateKind};
use crate::error::RawError;
pub(crate) type Res<T> = Result<T, RawError>;
pub use buffer::JSBuffer;
use log::trace;
mod keyword_trie;

/// A Raw version of the Scanner's `Item`
/// simply providing the start and end of the
/// span and the type of token that span
/// represents
#[derive(Debug, PartialEq, Eq)]
pub struct RawItem {
    pub ty: tokens::RawToken,
    pub start: usize,
    pub end: usize,
}

/// This structure will perform the low level
/// tokenization before the `Scanner` provides
/// additional context
pub struct Tokenizer<'a> {
    pub(super) stream: buffer::JSBuffer<'a>,
    pub(super) current_start: usize,
    pub(super) curly_stack: Vec<OpenCurlyKind>,
}

impl<'a> Tokenizer<'a> {
    /// Create a new tokenizer using
    /// the provided string reference
    /// to create a `JsBuffer`
    pub fn new(stream: &'a str) -> Self {
        Tokenizer {
            current_start: 0,
            stream: stream.into(),
            curly_stack: Vec::with_capacity(2),
        }
    }
    /// Get the next raw token from the js text
    pub fn next(&mut self, allow_html_comment_close: bool) -> Res<RawItem> {
        trace!("next {} {}", self.stream.idx, self.stream.len);
        self.current_start = self.stream.idx;
        let next_char = match self.stream.next_char() {
            Some(ch) => ch,
            None => {
                return Ok(RawItem {
                    start: self.stream.idx,
                    end: self.stream.idx,
                    ty: RawToken::EoF,
                })
            }
        };

        if next_char == '"' || next_char == '\'' {
            return self.string(next_char);
        }
        if next_char == '(' || next_char == ')' || next_char == ';' {
            return self.punct(next_char, allow_html_comment_close);
        }
        if next_char.is_ascii_digit() {
            return self.number(next_char);
        }
        if next_char == '`' {
            return self.template(next_char);
        }
        if next_char == '}' && self.curly_stack.last() == Some(&OpenCurlyKind::Template) {
            self.curly_stack.pop();
            return self.template(next_char);
        }
        if Self::is_id_start(next_char) {
            return self.ident(next_char);
        }
        self.punct(next_char, allow_html_comment_close)
    }
    /// get the next regex token from the js text, providing
    /// the length of the already consumed token (this will be either 1 or 2)
    ///
    /// note: this should only be used after first getting `/` or `/=`
    /// from the `next` method.
    pub fn next_regex(&mut self, start_len: usize) -> Res<RawItem> {
        trace!("next_regex {} {}", self.stream.idx, self.stream.len);
        self.current_start = self.stream.idx;
        let mut end_of_body = false;
        let mut body_idx = 0;
        if self.look_ahead_matches("\\/") {
            self.stream.skip_bytes(2);
        }
        let mut in_class = false;
        while let Some(c) = self.stream.next_char() {
            if end_of_body {
                if c == '\\' {
                    if self.look_ahead_byte_matches('u') {
                        // unicode escape
                        self.stream.skip_bytes(1);
                        if let Some(next) = self.stream.next_char() {
                            if next == '{' {
                                self.escaped_with_code_point()?;
                            } else {
                                self.escaped_with_hex4(next)?;
                            }
                        }
                    }
                } else if !Self::is_id_continue(c) {
                    let _ = self.stream.prev_char();
                    return self.gen_regex(start_len, body_idx);
                }
            } else if c == '\\' {
                if self.stream.at_new_line() {
                    return Err(RawError {
                        idx: self.stream.idx,
                        msg: "new line in regex literal".to_string(),
                    });
                } else {
                    self.stream.skip_bytes(1);
                }
            } else if is_line_term(c) {
                return Err(RawError {
                    idx: self.stream.idx,
                    msg: "new line in regex literal".to_string(),
                });
            } else if in_class {
                // we ignore the /
                if c == ']' {
                    in_class = false;
                }
            } else if c == '/' {
                end_of_body = true;
                body_idx = self.stream.idx;
            } else if c == '[' {
                in_class = true;
            }
        }
        if end_of_body {
            return self.gen_regex(start_len, body_idx);
        }
        Err(RawError {
            msg: format!(
                "unterminated regex at {}",
                String::from_utf8_lossy(&self.stream.buffer[self.current_start..self.stream.idx])
            ),
            idx: self.current_start,
        })
    }
    /// Parse an identifier, including
    /// any possible keywords
    fn ident(&mut self, start: char) -> Res<RawItem> {
        trace!(
            "ident {} ({}, {})",
            start,
            self.current_start,
            self.stream.idx
        );

        let start = if start == '\\' {
            let c = self.escaped_ident_part()?;
            if !Self::is_id_start(c) {
                debug!("bad char: {:?}", c);
                return Err(RawError {
                    msg: "invalid escaped identifier start".to_string(),
                    idx: self.current_start,
                });
            }
            c
        } else {
            start
        };
        if let Some(tok) = self.keyword(start)? {
            return self.gen_token(tok);
        }
        while let Some(c) = self.stream.next_char() {
            if c == '\\' {
                let c = self.escaped_ident_part()?;
                if !Self::is_id_continue(c) {
                    return Err(RawError {
                        msg: format!("invalid escaped identifier character: {}", c),
                        idx: self.current_start,
                    });
                }
            }
            if !Self::is_id_continue(c) && c != '\u{200C}' && c != '\u{200D}' {
                // if we have moved past the last valid identifier, go back 1
                let _ = self.stream.prev_char();
                break;
            }
        }

        self.gen_token(RawToken::Ident)
    }

    /// picking up after the \ in a unicode escape
    /// ex: `\u{61}` or `\u0061`
    #[inline]
    fn escaped_ident_part(&mut self) -> Res<char> {
        trace!("escaped_ident_part");
        if let Some('u') = self.stream.next_char() {
            let x = if let Some(c) = self.stream.next_char() {
                if c == '{' {
                    let (x, _) = self.escaped_with_code_point()?;
                    x
                } else {
                    self.escaped_with_hex4(c)?
                }
            } else {
                return Err(RawError {
                    msg: "invalid unicode escape sequence in identifier".to_string(),
                    idx: self.current_start,
                });
            };
            if let Some(c) = std::char::from_u32(x) {
                Ok(c)
            } else {
                Err(RawError {
                    msg: "invalid unicode escape sequence in identifier".to_string(),
                    idx: self.current_start,
                })
            }
        } else {
            Err(RawError {
                msg: "invalid unicode escape sequence in identifier".to_string(),
                idx: self.current_start,
            })
        }
    }

    /// Consume an escaped code point returning a tuple of the u32
    /// represented in the string as well as the length of the code
    /// point only (the {} will not be included in the count)
    ///
    /// ```js
    /// '\u{888}' \\ returns (2184, 3)
    /// ```
    #[inline]
    pub(crate) fn escaped_with_code_point(&mut self) -> Res<(u32, usize)> {
        trace!("escaped_with_code_point");
        let mut code = 0;
        let mut last_char: char = '{';
        let mut len: usize = 0;
        while let Some(c) = self.stream.next_char() {
            len = len.saturating_add(1);
            last_char = c;
            if c == '}' {
                break;
            }
            if let Some(n) = c.to_digit(16) {
                code = (code * 16) + n;
            } else {
                return Err(RawError {
                    msg: "escaped unicode code point contains a non-hex digit".to_string(),
                    idx: self.stream.idx,
                });
            }
        }

        if code > 0x10_FFFF {
            Err(RawError {
                msg: "escaped unicode codepoint too large".to_string(),
                idx: self.stream.idx,
            })
        } else if last_char != '}' {
            Err(RawError {
                msg: "escaped unicode code points must end in }".to_string(),
                idx: self.current_start,
            })
        } else {
            Ok((code, len))
        }
    }
    /// picking up after the `\u` in a unicode escape with 4 hex characters
    /// ex: `\u0061`
    #[inline]
    fn escaped_with_hex4(&mut self, start: char) -> Res<u32> {
        trace!("escaped_with_hex4");
        let mut code = if let Some(n) = start.to_digit(16) {
            n
        } else {
            return Err(RawError {
                msg: "escaped unicode char code is not a hex digit".to_string(),
                idx: self.stream.idx,
            });
        };
        for _ in 0..3 {
            if let Some(c) = self.stream.next_char() {
                if let Some(n) = c.to_digit(16) {
                    code = (code * 16) + n;
                } else {
                    return Err(RawError {
                        msg: "escaped unicode code point is not a hex digit".to_string(),
                        idx: self.stream.idx,
                    });
                }
            } else {
                return Err(RawError {
                    msg: "escaped unicode sequence does not have 4 characters".to_string(),
                    idx: self.current_start,
                });
            }
        }
        Ok(code)
    }
    /// Parse a string literal, the provided `quote` should be `'` or `"`
    /// to signal where the end of the string might be
    fn string(&mut self, quote: char) -> Res<RawItem> {
        trace!(
            "string {} ({}, {})",
            quote,
            self.current_start,
            self.stream.idx
        );
        let mut escaped = false;
        // we already skipped the quote char
        // so 1 is appropriate
        let mut last_len = 1usize;
        let mut new_line_count = 0usize;
        let mut found_octal_escape = false;
        while let Some(c) = self.stream.next_char() {
            if c == '\\' {
                escaped = !escaped;
                last_len = last_len.saturating_add(1);
            } else if c == '\r' {
                if !escaped {
                    // back up one to avoid splitting a unicode
                    // sequence
                    let _ = self.stream.prev_char();
                    return Err(RawError {
                        msg: "unescaped new line in string literal".to_string(),
                        idx: self.stream.idx,
                    });
                }
                if self.look_ahead_byte_matches('\n') {
                    self.stream.skip_bytes(1);
                }
                escaped = false;
                new_line_count = new_line_count.saturating_add(1);
                last_len = 0;
            } else if Self::is_new_line_not_cr(c) {
                if !escaped {
                    // back up one to avoid splitting a unicode
                    // sequence
                    let _ = self.stream.prev_char();
                    return Err(RawError {
                        msg: "unescaped new line in string literal".to_string(),
                        idx: self.stream.idx,
                    });
                }
                new_line_count = new_line_count.saturating_add(1);
                last_len = 0;
                escaped = false;
            } else if c == quote {
                last_len = last_len.saturating_add(1);
                if !escaped {
                    let kind = if quote == '"' {
                        StringKind::Double
                    } else {
                        StringKind::Single
                    };
                    return self.gen_token(RawToken::String {
                        kind,
                        new_line_count,
                        last_len,
                        found_octal_escape,
                    });
                }
                escaped = false;
            } else {
                let len = if escaped && c == 'u' {
                    if let Some(next) = self.stream.next_char() {
                        if next == '{' {
                            self.escaped_with_code_point()?;
                            8
                        } else {
                            self.escaped_with_hex4(next)?;
                            5
                        }
                    } else {
                        return Err(RawError {
                            idx: self.stream.idx,
                            msg: "Invalid escape in string literal".to_string(),
                        });
                    }
                } else if escaped && c.is_digit(8) {
                    if c != '0' || self.stream.at_decimal() {
                        found_octal_escape = true;
                    }
                    1
                } else {
                    1
                };
                last_len = last_len.saturating_add(len);
                escaped = false;
            }
        }
        // back up one to avoid splitting a unicode
        // sequence
        let _ = self.stream.prev_char();
        Err(RawError {
            msg: "unterminated string literal".to_string(),
            idx: self.stream.idx,
        })
    }
    /// Parse a punctuation mark or sequence the `c` provided is the
    /// first character in the possible sequence
    ///
    /// note: some of these may actually resolve to a another token, for example
    /// `.0` will resolve to a number
    fn punct(&mut self, c: char, allow_html_comment_close: bool) -> Res<RawItem> {
        trace!("punct {} ({}, {})", c, self.current_start, self.stream.idx);
        match c {
            '(' => self.gen_punct(Punct::OpenParen),
            ')' => self.gen_punct(Punct::CloseParen),
            ';' => self.gen_punct(Punct::SemiColon),
            ',' => self.gen_punct(Punct::Comma),
            '[' => self.gen_punct(Punct::OpenBracket),
            ']' => self.gen_punct(Punct::CloseBracket),
            ':' => self.gen_punct(Punct::Colon),
            '?' => self.gen_punct(Punct::QuestionMark),
            '#' => self.hash(),
            '~' => self.gen_punct(Punct::Tilde),
            '{' => self.open_curly(OpenCurlyKind::Block, Punct::OpenBrace),
            '}' => self.close_curly(Punct::CloseBrace),
            '@' => self.gen_punct(Punct::AtMark),
            '.' => self.period(),
            '>' => self.greater_than(),
            '<' => self.less_than(),
            '=' => self.equals(),
            '!' => self.bang(),
            '*' => self.asterisk(),
            '&' => self.ampersand(),
            '|' => self.pipe(),
            '+' => self.plus(),
            '-' => self.minus(allow_html_comment_close),
            '/' => self.forward_slash(allow_html_comment_close),
            '%' => self.percent(),
            '^' => self.caret(),
            _ => Err(RawError {
                msg: format!("unknown punct {:?}", c),
                idx: self.current_start,
            }),
        }
    }
    /// An open curly doesn't have a possible sequence but there
    /// is some book keeping for detecting a possible template
    /// start. The provided `curly` argument will be pushed
    /// onto the `curly_stack` for inspection later
    #[inline]
    fn open_curly(&mut self, curly: OpenCurlyKind, punct: Punct) -> Res<RawItem> {
        trace!(
            "open_curly {:?} ({}, {})",
            curly,
            self.current_start,
            self.stream.idx
        );
        self.curly_stack.push(curly);
        self.gen_punct(punct)
    }
    /// An close curly doesn't have a possible sequence but we
    /// need to pop the top off of the `curly_stack` to make
    /// sure we correctly detect a possible template start
    #[inline]
    fn close_curly(&mut self, punct: Punct) -> Res<RawItem> {
        trace!(
            "close_curly {:?} ({}, {})",
            punct,
            self.current_start,
            self.stream.idx
        );
        let _ = self.curly_stack.pop();
        self.gen_punct(punct)
    }
    /// A period could result in a spread/rest operator (`...`),
    /// a number (`.0`) or just be a period
    #[inline]
    fn period(&mut self) -> Res<RawItem> {
        trace!("period ({}, {})", self.current_start, self.stream.idx);
        if self.look_ahead_matches("..") {
            self.stream.skip_bytes(2);
            self.gen_punct(Punct::Ellipsis)
        } else if self.stream.at_decimal() {
            self.dec_number(true, '.')
        } else {
            self.gen_punct(Punct::Period)
        }
    }
    /// A `>` could be `>>>=`, `>>>`, `>>=` , `>>` or `>=`
    #[inline]
    fn greater_than(&mut self) -> Res<RawItem> {
        trace!("greater_than ({}, {})", self.current_start, self.stream.idx);
        if self.look_ahead_matches(">>=") {
            self.stream.skip_bytes(3);
            self.gen_punct(Punct::TripleGreaterThanEqual)
        } else if self.look_ahead_matches(">>") {
            self.stream.skip_bytes(2);
            self.gen_punct(Punct::TripleGreaterThan)
        } else if self.look_ahead_matches(">=") {
            self.stream.skip_bytes(2);
            self.gen_punct(Punct::DoubleGreaterThanEqual)
        } else if self.look_ahead_byte_matches('>') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::DoubleGreaterThan)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::GreaterThanEqual)
        } else {
            self.gen_punct(Punct::GreaterThan)
        }
    }
    /// A < could be `<<=`, `<=`, `<<`, or `<!--`
    #[inline]
    fn less_than(&mut self) -> Res<RawItem> {
        trace!("less_than ({}, {})", self.current_start, self.stream.idx);
        if self.look_ahead_matches("<=") {
            self.stream.skip_bytes(2);
            self.gen_punct(Punct::DoubleLessThanEqual)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::LessThanEqual)
        } else if self.look_ahead_byte_matches('<') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::DoubleLessThan)
        } else if self.look_ahead_matches("!--") {
            self.stream.skip_bytes(3);
            self.html_comment()
        } else {
            self.gen_punct(Punct::LessThan)
        }
    }
    /// An `=` could be `===`, `==` or `=>`
    #[inline]
    fn equals(&mut self) -> Res<RawItem> {
        trace!("equals ({}, {})", self.current_start, self.stream.idx);
        if self.look_ahead_matches("==") {
            self.stream.skip_bytes(2);
            self.gen_punct(Punct::TripleEqual)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::DoubleEqual)
        } else if self.look_ahead_byte_matches('>') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::EqualGreaterThan)
        } else {
            self.gen_punct(Punct::Equal)
        }
    }
    /// a `!` could be `!==`, or `!=`
    #[inline]
    fn bang(&mut self) -> Res<RawItem> {
        trace!("bang ({}, {})", self.current_start, self.stream.idx);
        if self.look_ahead_matches("==") {
            self.stream.skip_bytes(2);
            self.gen_punct(Punct::BangDoubleEqual)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::BangEqual)
        } else {
            self.gen_punct(Punct::Bang)
        }
    }
    /// a `*` could be `**=`, `**`, or `*=`
    #[inline]
    fn asterisk(&mut self) -> Res<RawItem> {
        trace!("asterisk ({}, {})", self.current_start, self.stream.idx);
        if self.look_ahead_matches("*=") {
            self.stream.skip_bytes(2);
            self.gen_punct(Punct::DoubleAsteriskEqual)
        } else if self.look_ahead_byte_matches('*') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::DoubleAsterisk)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::AsteriskEqual)
        } else {
            self.gen_punct(Punct::Asterisk)
        }
    }
    /// a `&` could be `&&` or `&=`
    #[inline]
    fn ampersand(&mut self) -> Res<RawItem> {
        trace!("ampersand ({}, {})", self.current_start, self.stream.idx);
        if self.look_ahead_byte_matches('&') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::DoubleAmpersand)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::AmpersandEqual)
        } else {
            self.gen_punct(Punct::Ampersand)
        }
    }
    /// a `|` could be `||` or `|=`
    #[inline]
    fn pipe(&mut self) -> Res<RawItem> {
        trace!("pipe ({}, {})", self.current_start, self.stream.idx);
        if self.look_ahead_byte_matches('|') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::DoublePipe)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::PipeEqual)
        } else {
            self.gen_punct(Punct::Pipe)
        }
    }
    /// a `+` could be `++` or `+=`
    #[inline]
    fn plus(&mut self) -> Res<RawItem> {
        trace!("plus ({}, {})", self.current_start, self.stream.idx);
        if self.look_ahead_byte_matches('+') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::DoublePlus)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::PlusEqual)
        } else {
            self.gen_punct(Punct::Plus)
        }
    }
    /// a `-` could be `--` or `-=`
    #[inline]
    fn minus(&mut self, allow_html_comment_close: bool) -> Res<RawItem> {
        trace!("minus ({}, {})", self.current_start, self.stream.idx);
        if self.look_ahead_byte_matches('-') {
            self.stream.skip_bytes(1);
            if allow_html_comment_close && self.look_ahead_byte_matches('>') {
                self.single_comment(CommentKind::Html)
            } else {
                self.gen_punct(Punct::DoubleDash)
            }
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::DashEqual)
        } else {
            self.gen_punct(Punct::Dash)
        }
    }
    /// a `/` could be `/=` or the start of a multiline comment `/*`
    /// or the start of a single line comment `//`
    #[inline]
    fn forward_slash(&mut self, allow_html_comment_close: bool) -> Res<RawItem> {
        trace!(
            "forward_slash ({}, {})",
            self.current_start,
            self.stream.idx
        );
        if self.look_ahead_byte_matches('=') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::ForwardSlashEqual)
        } else if self.look_ahead_byte_matches('*') {
            self.stream.skip_bytes(1);
            self.multi_comment(allow_html_comment_close)
        } else if self.look_ahead_byte_matches('/') {
            self.single_comment(CommentKind::Single)
        } else {
            self.gen_punct(Punct::ForwardSlash)
        }
    }
    /// A `%` could also be `%=`
    #[inline]
    fn percent(&mut self) -> Res<RawItem> {
        trace!("percent ({}, {})", self.current_start, self.stream.idx);
        if self.look_ahead_byte_matches('=') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::PercentEqual)
        } else {
            self.gen_punct(Punct::Percent)
        }
    }
    // a `^` could also be `^=`
    #[inline]
    fn caret(&mut self) -> Res<RawItem> {
        trace!("caret ({}, {})", self.current_start, self.stream.idx);
        if self.look_ahead_byte_matches('=') {
            self.stream.skip_bytes(1);
            self.gen_punct(Punct::CaretEqual)
        } else {
            self.gen_punct(Punct::Caret)
        }
    }
    /// a `#` could also be the start of a hash bang comment `#!`
    #[inline]
    fn hash(&mut self) -> Res<RawItem> {
        trace!("hash ({}, {})", self.current_start, self.stream.idx);
        // hashbang comment can only appear at the start
        if self.current_start == 0 && self.look_ahead_byte_matches('!') {
            while !self.at_new_line() {
                if self.stream.next_char().is_none() {
                    break;
                }
            }
            self.gen_comment(CommentKind::Hashbang, 0, 0, self.local_index())
        } else {
            self.gen_punct(Punct::Hash)
        }
    }
    /// parse a number, this can include decimal or float literals
    /// like `0.01e1` or `10` as well as binary, octal or hex
    /// literals like `0b1`, `0o7`, or `0xf` and BigInt literals
    /// like `1n`
    #[inline]
    fn number(&mut self, start: char) -> Res<RawItem> {
        trace!(
            "number {} ({}, {})",
            start,
            self.current_start,
            self.stream.idx
        );
        if start != '.' {
            if let Some(next) = self.stream.next_char() {
                if start == '0' {
                    if next.eq_ignore_ascii_case(&'x') {
                        self.hex_number()
                    } else if next.eq_ignore_ascii_case(&'o') {
                        self.oct_number()
                    } else if next.eq_ignore_ascii_case(&'b') {
                        self.bin_number()
                    } else if next == 'n' {
                        self.gen_number(NumberKind::BigInt)
                    } else if next.is_ascii_digit() {
                        self.dec_number(false, next)
                    } else if next == '.' {
                        self.dec_number(true, next)
                    } else {
                        let _ = self.stream.prev_char();
                        self.dec_number(false, next)
                    }
                } else if next == '.' {
                    self.dec_number(true, next)
                } else if next == 'n' {
                    self.gen_number(NumberKind::BigInt)
                } else {
                    let _ = self.stream.prev_char();
                    self.dec_number(start == '.', start)
                }
            } else {
                self.gen_number(NumberKind::Dec)
            }
        } else {
            self.punct(start, true)
        }
    }

    /// parse the string portion of a template literal
    /// the start will either be a back tick or
    /// `${`
    #[inline]
    fn template(&mut self, start: char) -> Res<RawItem> {
        trace!(
            "template {} ({}, {})",
            start,
            self.current_start,
            self.stream.idx
        );
        let mut line_count = 0usize;
        let mut last_len = 1usize; // we already skipped the start char
        let mut found_octal_escape = false;
        let mut found_invalid_unicode = false;
        let mut found_invalid_hex = false;
        while let Some(c) = self.stream.next_char() {
            last_len = last_len.saturating_add(1);
            if c == '\\' {
                if self.look_ahead_matches("${") {
                    last_len = last_len.saturating_add(2);
                    self.stream.skip_bytes(2);
                } else if self.look_ahead_byte_matches('`') || self.look_ahead_byte_matches('\\') {
                    last_len = last_len.saturating_add(1);
                    self.stream.skip_bytes(1);
                } else if self.look_ahead_byte_matches('0') {
                    last_len = last_len.saturating_add(1);
                    self.stream.skip_bytes(1);
                    if self.stream.at_octal() {
                        found_octal_escape = true;
                    }
                } else if self.stream.at_octal() {
                    found_octal_escape = true;
                } else if self.look_ahead_byte_matches('u') {
                    self.stream.skip_bytes(1);
                    last_len = last_len.saturating_add(1);
                    if let Some(ch) = self.stream.next_char() {
                        last_len = last_len.saturating_add(1);
                        if ch == '{' {
                            let mut acc = 0u32;
                            while self.stream.at_hex() {
                                last_len = last_len.saturating_add(1);
                                if let Some(n) = self.stream.next_char() {
                                    acc = (acc * 16) + n.to_digit(16).unwrap();
                                }
                            }
                            while !self.look_ahead_byte_matches('}') {
                                found_invalid_unicode = true;
                                if self.look_ahead_byte_matches('`') {
                                    break;
                                }
                                self.stream.skip_bytes(1);
                                last_len = last_len.saturating_add(1);
                            }

                            if acc > 0x10_FFFF {
                                found_invalid_unicode = true;
                            }
                        } else if ch.is_ascii_hexdigit() {
                            for _ in 0..3 {
                                if self.stream.at_hex() {
                                    self.stream.skip_bytes(1);
                                } else {
                                    found_invalid_unicode = true;
                                    break;
                                }
                            }
                        } else {
                            found_invalid_unicode = true;
                        };
                    } else {
                        return Err(RawError {
                            idx: self.stream.idx,
                            msg: "Invalid escape sequence in template literal".to_string(),
                        });
                    }
                } else if self.look_ahead_byte_matches('x') {
                    self.stream.skip_bytes(1);
                    last_len = last_len.saturating_add(1);
                    for _ in 0..2 {
                        if self.stream.at_hex() {
                            let _ = self.stream.next_char();
                        } else {
                            found_invalid_hex = true;
                        }
                    }
                }
            } else if c == '\r' {
                if self.look_ahead_byte_matches('\n') {
                    self.stream.skip_bytes(1);
                }
                line_count = line_count.saturating_add(1);
                last_len = 0;
            } else if Self::is_new_line_not_cr(c) {
                line_count = line_count.saturating_add(1);
                last_len = 0;
            } else if c == '$' {
                if self.look_ahead_byte_matches('{') {
                    self.stream.skip_bytes(1);
                    last_len = last_len.saturating_add(1);
                    self.curly_stack.push(OpenCurlyKind::Template);
                    if start == '`' {
                        return self.gen_template(
                            TemplateKind::Head,
                            line_count,
                            last_len,
                            found_octal_escape,
                            found_invalid_unicode,
                            found_invalid_hex,
                        );
                    } else {
                        return self.gen_template(
                            TemplateKind::Body,
                            line_count,
                            last_len,
                            found_octal_escape,
                            found_invalid_unicode,
                            found_invalid_hex,
                        );
                    }
                }
            } else if c == '`' {
                if start == '`' {
                    return self.gen_template(
                        TemplateKind::NoSub,
                        line_count,
                        last_len,
                        found_octal_escape,
                        found_invalid_unicode,
                        found_invalid_hex,
                    );
                } else {
                    return self.gen_template(
                        TemplateKind::Tail,
                        line_count,
                        last_len,
                        found_octal_escape,
                        found_invalid_unicode,
                        found_invalid_hex,
                    );
                }
            }
        }
        Err(RawError {
            msg: format!(
                "unterminated template: {:?}",
                String::from_utf8_lossy(&self.stream.buffer[self.current_start..self.stream.idx])
            ),
            idx: self.current_start,
        })
    }
    /// parse a single comment after finding `//`
    #[inline]
    fn single_comment(&mut self, kind: CommentKind) -> Res<RawItem> {
        trace!(
            "single_comment {:?} ({}, {})",
            kind,
            self.current_start,
            self.stream.idx
        );
        while !self.at_new_line() {
            if self.stream.next_char().is_none() {
                break;
            }
        }
        self.gen_comment(kind, 0, 0, self.local_index())
    }
    /// parse a multi-line comment after finding `/*`
    #[inline]
    fn multi_comment(&mut self, allow_html_comment_close: bool) -> Res<RawItem> {
        trace!(
            "multi_comment ({}, {}) allow_html_comment_close: {}",
            self.current_start,
            self.stream.idx,
            allow_html_comment_close
        );
        let mut new_line_count = 0usize;
        let mut last_len = 2usize; // we already skipped the /*
        let mut found_end = false;
        let mut end_idx = None;
        while let Some(c) = self.stream.next_char() {
            if c == '*' && self.look_ahead_byte_matches('/') {
                self.stream.skip_bytes(1);
                found_end = true;
                end_idx = Some(self.local_index());
                last_len = last_len.saturating_add(2);
                break;
            } else if c == '\r' {
                if self.look_ahead_byte_matches('\n') {
                    self.stream.skip_bytes(1);
                }
                new_line_count = new_line_count.saturating_add(1);
                last_len = 0;
            } else if Self::is_new_line_not_cr(c) {
                new_line_count = new_line_count.saturating_add(1);
                last_len = 0;
            } else {
                last_len = last_len.saturating_add(1);
            }
        }
        if found_end {
            if (new_line_count > 0 || allow_html_comment_close) && self.look_ahead_matches("-->") {
                self.stream.skip_bytes(3);

                while !self.stream.at_end() && !self.at_new_line() {
                    self.stream.skip_bytes(1);
                    last_len = last_len.saturating_add(1);
                }
            }
            let end_idx = end_idx.unwrap_or_else(|| self.local_index());
            self.gen_comment(CommentKind::Multi, new_line_count, last_len, end_idx)
        } else {
            Err(RawError {
                idx: self.current_start,
                msg: "unterminated multi-line comment".to_string(),
            })
        }
    }
    /// parse an html comment after finding `<!--`
    #[inline]
    fn html_comment(&mut self) -> Res<RawItem> {
        trace!("html_comment ({}, {})", self.current_start, self.stream.idx);
        let mut end_idx = None;
        while !self.stream.at_end() {
            if self.stream.at_new_line() {
                end_idx = Some(self.local_index());
                break;
            }

            if self.look_ahead_matches("-->") {
                self.stream.skip_bytes(3);
                end_idx = Some(self.local_index());
            } else {
                self.stream.skip_bytes(1);
            }
        }
        if let Some(end_idx) = end_idx {
            return self.gen_comment(CommentKind::Html, 0, 0, end_idx);
        }
        Err(RawError {
            msg: "unterminated html comment".to_string(),
            idx: self.current_start,
        })
    }
    /// parse a number literal after finding `0x` or `0X`
    #[inline]
    fn hex_number(&mut self) -> Res<RawItem> {
        trace!("hex_number ({}, {})", self.current_start, self.stream.idx);
        let mut prev_char = if let Some(c) = self.stream.next_char() {
            if !c.is_ascii_hexdigit() {
                return Err(RawError {
                    msg: "empty hex literal".to_string(),
                    idx: self.current_start,
                });
            }
            c
        } else {
            return Err(RawError {
                msg: "empty hex literal".to_string(),
                idx: self.current_start,
            });
        };

        while self.stream.at_hex() || self.stream.look_ahead_byte_matches(b'_') {
            let c = self.stream.next_char().unwrap();
            self.check_repeating_underscore(prev_char, c)?;
            prev_char = c;
        }
        let kind = self.bigint_guard(NumberKind::Hex);

        self.check_trailing_underscore(prev_char)?;
        self.check_trailing_ident_start()?;
        self.gen_number(kind)
    }
    /// parse a number literal after finding `0o` or `0O`
    #[inline]
    fn oct_number(&mut self) -> Res<RawItem> {
        trace!("oct_number ({}, {})", self.current_start, self.stream.idx);
        let mut prev_char = if let Some(c) = self.stream.next_char() {
            if !c.is_digit(8) {
                return Err(RawError {
                    msg: "empty octal literal".to_string(),
                    idx: self.current_start,
                });
            }
            c
        } else {
            return Err(RawError {
                msg: "empty octal literal".to_string(),
                idx: self.current_start,
            });
        };
        while self.stream.at_octal() || self.look_ahead_byte_matches('_') {
            let c = self.stream.next_char().unwrap();
            self.check_repeating_underscore(prev_char, c)?;
            prev_char = c;
        }
        let kind = self.bigint_guard(NumberKind::Oct);

        self.check_trailing_underscore(prev_char)?;
        self.check_trailing_ident_start()?;
        self.gen_number(kind)
    }
    /// parse a number literal after finding a `0b` or `0B`
    #[inline]
    fn bin_number(&mut self) -> Res<RawItem> {
        trace!("bin_number ({}, {})", self.current_start, self.stream.idx);
        let mut prev_char = if let Some(c) = self.stream.next_char() {
            if !c.is_digit(2) {
                return Err(RawError {
                    msg: "empty binary literal".to_string(),
                    idx: self.current_start,
                });
            }
            c
        } else {
            return Err(RawError {
                msg: "empty binary literal".to_string(),
                idx: self.current_start,
            });
        };
        while self.stream.at_binary() || self.stream.look_ahead_byte_matches(b'_') {
            let c = self.stream.next_char().unwrap();
            self.check_repeating_underscore(prev_char, c)?;
            prev_char = c;
        }
        let kind = self.bigint_guard(NumberKind::Bin);

        self.check_trailing_underscore(prev_char)?;
        self.check_trailing_ident_start()?;
        self.gen_number(kind)
    }
    /// parse a decimal or float literal
    /// like `1234` or `12.34` or `12.34e56`
    #[inline]
    fn dec_number(&mut self, seen_point: bool, mut prev_char: char) -> Res<RawItem> {
        trace!("dec_number ({}, {})", self.current_start, self.stream.idx);
        prev_char = self.consume_digits(10, prev_char)?;
        let mut check_for_n = !seen_point;
        if !seen_point && self.look_ahead_byte_matches('.') {
            check_for_n = false;
            self.stream.skip_bytes(1);
            prev_char = self.consume_digits(10, '.')?;
        }
        // if we find e || E, prev_char != _
        // allow for + or - next
        // at least one number is required next
        // go back to step 1
        if self.look_ahead_byte_matches('e') || self.look_ahead_byte_matches('E') {
            check_for_n = false;
            self.stream.skip_bytes(1);
            prev_char = 'e';
            if self.look_ahead_byte_matches('-') || self.look_ahead_byte_matches('+') {
                self.stream.skip_bytes(1);
                prev_char = '-';
            } else if !self.stream.at_decimal() {
                return Err(RawError {
                    msg: "Invalid decimal, exponents must be followed by +, - or decimal digits"
                        .to_string(),
                    idx: self.current_start,
                });
            }
            prev_char = self.consume_digits(10, prev_char)?;
        }
        let kind = self.bigint_guard(NumberKind::Dec);
        if kind == NumberKind::BigInt && !check_for_n {
            return Err(RawError {
                msg: "Invalid decimal, Floats cannot be BigInts".to_string(),
                idx: self.current_start,
            });
        }

        self.check_trailing_underscore(prev_char)?;
        self.check_trailing_ident_start()?;
        self.gen_number(kind)
    }
    /// Helper to consume consecutive digits, taking into account
    /// that _ is a valid numeric separator
    fn consume_digits(&mut self, radix: u32, mut prev_char: char) -> Res<char> {
        trace!(
            "consume_digits {}, {} ({}, {})",
            radix,
            prev_char,
            self.current_start,
            self.stream.idx
        );
        while let Some(c) = self.stream.next_char() {
            self.check_repeating_underscore(prev_char, c)?;
            if !c.is_digit(radix) && c != '_' {
                let _ = self.stream.prev_char();
                break;
            }
            prev_char = c;
        }
        Ok(prev_char)
    }
    /// Guard against a number ending with `_`
    #[inline]
    fn check_trailing_underscore(&self, prev_char: char) -> Res<()> {
        trace!(
            "check_trailing_underscore {} ({}, {})",
            prev_char,
            self.current_start,
            self.stream.idx
        );
        if prev_char == '_' {
            Err(RawError {
                msg: "Invalid decimal. Numbers cannot end with an underscore".to_string(),
                idx: self.current_start,
            })
        } else {
            Ok(())
        }
    }
    #[inline]
    fn check_trailing_ident_start(&mut self) -> Res<()> {
        if let Some(next) = self.stream.peek_char() {
            if Self::is_id_start(next) {
                return Err(RawError {
                    idx: self.stream.idx,
                    msg: "Number literal cannot be immediately followed by an identifier"
                        .to_string(),
                });
            }
        }
        Ok(())
    }
    /// Guard against a number literal having two `_` in a row
    #[inline]
    fn check_repeating_underscore(&self, char_1: char, char_2: char) -> Res<()> {
        trace!(
            "check_repeating_underscore {} {} ({}, {})",
            char_1,
            char_2,
            self.current_start,
            self.stream.idx
        );
        if char_1 == '_' && char_2 == '_' {
            Err(RawError {
                msg: "double numeric separator".to_string(),
                idx: self.current_start,
            })
        } else {
            Ok(())
        }
    }
    /// If a number literal ends with a `n` it would actually be a BigInt
    #[inline]
    fn bigint_guard(&mut self, number_kind: NumberKind) -> NumberKind {
        trace!(
            "bigint_guard {:?} ({}, {})",
            number_kind,
            self.current_start,
            self.stream.idx
        );
        if self.look_ahead_byte_matches('n') {
            let _ = self.stream.next_char();
            NumberKind::BigInt
        } else {
            number_kind
        }
    }
    /// check if a character has the unicode property of
    /// ID_CONTINUE
    #[inline]
    fn is_id_continue(c: char) -> bool {
        trace!(target:"idents", "is_id_continue {}", c);
        unicode::is_id_continue(c)
    }
    /// check if a character has the unicode property of
    /// ID_START
    #[inline]
    fn is_id_start(c: char) -> bool {
        trace!(target:"idents", "is_id_start {}", c);
        unicode::is_id_start(c)
    }
    /// Test if the next character matches a single byte
    /// character
    #[inline]
    fn look_ahead_byte_matches(&self, c: char) -> bool {
        trace!(
            "look_ahead_byte_matches {} ({}, {})",
            c,
            self.current_start,
            self.stream.idx
        );
        self.stream.look_ahead_byte_matches(c as u8)
    }
    /// Test if the next character matches a multi byte
    /// character
    #[inline]
    fn look_ahead_matches(&self, s: &str) -> bool {
        trace!(
            "look_ahead_matches {} ({}, {})",
            s,
            self.current_start,
            self.stream.idx
        );
        self.stream.look_ahead_matches(s.as_bytes())
    }
    /// Convenience method for wrapping a `Punct` in a `RawItem`
    #[inline]
    fn gen_punct(&self, p: Punct) -> Res<RawItem> {
        trace!(
            "gen_punct {:?} ({}, {})",
            p,
            self.current_start,
            self.stream.idx
        );
        self.gen_token(RawToken::Punct(p))
    }
    /// Convenience method for wrapping a `Number` in a `RawItem`
    #[inline]
    fn gen_number(&self, n: NumberKind) -> Res<RawItem> {
        trace!(
            "gen_number {:?} ({}, {})",
            n,
            self.current_start,
            self.stream.idx
        );
        self.gen_token(RawToken::Number(n))
    }
    /// Convenience method for wrapping a `Template` in a `RawItem`
    #[inline]
    fn gen_template(
        &self,
        kind: TemplateKind,
        new_line_count: usize,
        last_len: usize,
        has_octal_escape: bool,
        invalid_unicode: bool,
        invalid_hex: bool,
    ) -> Res<RawItem> {
        trace!(
            "gen_template {:?}, {}, {} ({}, {})",
            kind,
            new_line_count,
            last_len,
            self.current_start,
            self.stream.idx
        );
        self.gen_token(RawToken::Template {
            kind,
            new_line_count,
            last_len,
            has_octal_escape,
            found_invalid_unicode_escape: invalid_unicode,
            found_invalid_hex_escape: invalid_hex,
        })
    }
    /// Convenience method for wrapping a `RegEx` in a `RawItem`
    #[inline]
    fn gen_regex(&self, start_len: usize, body_idx: usize) -> Res<RawItem> {
        trace!(
            "gen_regex {}, {} ({}, {})",
            start_len,
            body_idx,
            self.current_start,
            self.stream.idx
        );
        Ok(RawItem {
            start: self.current_start.saturating_sub(start_len),
            end: self.stream.idx,
            ty: RawToken::RegEx(body_idx),
        })
    }
    /// Convenience method for wrapping a `Comment` in a `RawItem`
    #[inline]
    fn gen_comment(
        &self,
        kind: CommentKind,
        new_line_count: usize,
        last_len: usize,
        end_index: usize,
    ) -> Res<RawItem> {
        trace!(
            "gen_comment {:?} {}, {} ({}, {})",
            kind,
            new_line_count,
            last_len,
            self.current_start,
            self.stream.idx
        );
        self.gen_token(RawToken::Comment {
            kind,
            new_line_count,
            last_len,
            end_index,
        })
    }
    /// Convenience method for wrapping a `RawToken` in a `RawItem`
    #[inline]
    fn gen_token(&self, ty: RawToken) -> Res<RawItem> {
        trace!(
            "gen_token {:?} ({}, {})",
            ty,
            self.current_start,
            self.stream.idx
        );
        Ok(RawItem {
            start: self.current_start,
            end: self.stream.idx,
            ty,
        })
    }
    /// Skip any whitespace that might be coming up
    #[inline]
    pub fn skip_whitespace(&mut self) -> (usize, usize) {
        trace!(
            "skip_whitespace {}, {} {}",
            self.current_start,
            self.stream.idx,
            self.stream.len
        );
        let mut new_line_ct = 0usize;
        let mut leading_whitespace = 0usize;
        while self.stream.at_whitespace() {
            if self.at_new_line() {
                new_line_ct += 1;
                leading_whitespace = 0;
            }
            leading_whitespace = leading_whitespace.saturating_add(1);
            self.stream.skip(1);
        }
        (new_line_ct, leading_whitespace)
    }
    /// Check if the look ahead is a new line character
    #[inline]
    fn at_new_line(&mut self) -> bool {
        trace!("at_new_line ({}, {})", self.current_start, self.stream.idx);
        self.stream.at_new_line()
    }
    /// Get the current position inside of this token
    #[inline]
    fn local_index(&self) -> usize {
        self.stream.idx - self.current_start
    }
    /// Carriage Return is always a special case so that
    /// must be handled inline
    #[inline]
    fn is_new_line_not_cr(c: char) -> bool {
        trace!("is_new_line_not_cr {:?}", c);
        if c == '\n' {
            true
        } else if c < '\u{2028}' {
            false
        } else {
            c == '\u{2028}' || c == '\u{2029}'
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tokenizer_punct() {
        static PUNCTS: &[&str] = &[
            "{", "}", "(", ")", ".", ";", ",", "[", "]", ":", "?", "~", ">", "<", "=", "!", "+",
            "-", "/", "*", "%", "&", "|", "^", ">>>=", //3 char
            "...", "===", "!==", ">>>", "<<=", ">>=", "**=", //2 char
            "&&", "||", "==", "!=", "+=", "-=", "*=", "/=", "++", "--", "<<", ">>", "&=", "|=",
            "^=", "%=", "<=", ">=", "=>", "**", "@",
        ];
        for p in PUNCTS {
            let mut t = Tokenizer::new(p);
            let item = t.next(true).unwrap();
            assert!(item.ty.is_punct());
            assert!(t.stream.at_end());
        }
    }
    #[test]
    fn tokenizer_hashbang() {
        let b = "#!/usr/bin/env node";
        let mut t = Tokenizer::new(b);
        let item = t.next(true).unwrap();
        match item.ty {
            RawToken::Comment {
                kind: CommentKind::Hashbang,
                ..
            } => (),
            _ => panic!("expected hashbang comment, found {:?}", item.ty),
        }
        assert_eq!(&b[item.start..item.end], "#!/usr/bin/env node");

        let b = "#!";
        let mut t = Tokenizer::new(b);
        let item = t.next(true).unwrap();
        match item.ty {
            RawToken::Comment {
                kind: CommentKind::Hashbang,
                ..
            } => (),
            _ => panic!("expected hashbang comment, found {:?}", item.ty),
        }
        assert_eq!(&b[item.start..item.end], "#!");

        let b = "#!\nlol";
        let mut t = Tokenizer::new(b);
        let item = t.next(true).unwrap();
        match item.ty {
            RawToken::Comment {
                kind: CommentKind::Hashbang,
                ..
            } => (),
            _ => panic!("expected hashbang comment, found {:?}", item.ty),
        }
        assert_eq!(&b[item.start..item.end], "#!");

        let b = "#!/usr/bin/env node\nprocess.exit(1)";
        let mut t = Tokenizer::new(b);
        let item = t.next(true).unwrap();
        match item.ty {
            RawToken::Comment {
                kind: CommentKind::Hashbang,
                ..
            } => (),
            _ => panic!("expected hashbang comment, found {:?}", item.ty),
        }
        assert_eq!(&b[item.start..item.end], "#!/usr/bin/env node");

        // should not parse #! in as hashbang unless it's at the start of the text
        let b = "\n#!/usr/bin/env node";
        let mut t = Tokenizer::new(b);
        t.skip_whitespace();
        let item = t.next(true).unwrap();
        match item.ty {
            RawToken::Punct(Punct::Hash) => (),
            _ => panic!("expected hash, found {:?}", item.ty),
        }
    }
    #[test]
    fn tokenizer_strings() {
        static STRINGS: &[&str] = &[
            r#""things and stuff""#,
            r#"'people and places'"#,
            r#""with and escaped \"""#,
            r#"'another escaped \''"#,
            r#""with a new \
        line""#,
            r#"'another new line \
        hahaha'"#,
            "\"sequence double quoted\\\r\nis hard\"",
            "'new line sequence\\\r\nmight be harder'",
            r#""\
""#,
        ];
        for s in STRINGS {
            dbg!(&s);
            let mut t = Tokenizer::new(s);
            let item = t.next(true).unwrap();
            match &item.ty {
                RawToken::String { kind, .. } => {
                    if &s[0..1] == "'" {
                        match kind {
                            StringKind::Single => (),
                            StringKind::Double => {
                                panic!("Expected single quote string, found double")
                            }
                        }
                    } else {
                        match kind {
                            StringKind::Single => {
                                panic!("expected double quote string, found single")
                            }
                            StringKind::Double => (),
                        }
                    }
                }
                _ => panic!("Expected string, found {:?}", item.ty),
            }
            assert_eq!(s.len(), item.end - item.start);
            assert!(t.stream.at_end());
        }
    }

    #[test]
    fn tokenizer_idents() {
        let _ = pretty_env_logger::try_init();
        static IDENTS: &[&str] = &[
            r#"$"#,
            r#"_"#,
            r#"\u0078"#,
            r#"x$"#,
            r#"x_"#,
            r#"x\u0030"#,
            r#"x\u{e01d5}"#,
            r#"xa"#,
            r#"x0"#,
            r#"x0a"#,
            r#"x0123456789"#,
            r#"qwertyuiopasdfghjklzxcvbnm"#,
            r#"QWERTYUIOPASDFGHJKLZXCVBNM"#,
            r#"œ一"#,
            r#"ǻ둘"#,
            r#"ɤ〩"#,
            r#"φ"#,
            r#"ﬁⅷ"#,
            r#"ユニコード"#,
            r#"x‌‍"#,
            r#"\u08BE"#,
            r#"\u{8be}"#,
        ];
        for i in IDENTS {
            let mut t = Tokenizer::new(dbg!(i));
            let item = t.next(true).unwrap();
            assert_eq!(item.ty, RawToken::Ident);
            if !t.stream.at_end() {
                panic!(
                    "stream not at end - unparsed: {:?}",
                    String::from_utf8_lossy(&t.stream.buffer[t.stream.idx..])
                );
            }
            assert!(t.stream.at_end());
        }
    }

    #[test]
    #[should_panic = "invalid unicode escape sequence in identifier"]
    fn tokenizer_ident_bad_escape() {
        let mut t = Tokenizer::new(r#"\u"#);
        t.next(true).unwrap();
    }

    #[test]
    #[should_panic = "invalid unicode escape sequence in identifier"]
    fn tokenizer_ident_slash_only() {
        let mut t = Tokenizer::new(r#"\x"#);
        t.next(true).unwrap();
    }

    #[test]
    fn tokenizer_number() {
        static NUMBERS: &[&str] = &[
            "0",
            "00",
            "1234567890",
            "01234567",
            "0.",
            "0.00",
            "10.00",
            ".0",
            "1.",
            "0e0",
            "0E0",
            "0.e0",
            "0.00e+0",
            ".00e-0",
            "0x0",
            "0X0",
            "0x0123456789abcdefABCDEF",
            "0b0",
            "0b0100101",
            "0o0",
            "0o777",
            "2e308",
            "1e1",
            "0b1010_0001_1000_0101",
            "0xA0_B0_C0",
            "0o6_5",
            "2.0_00",
            "300_000",
            "4e56_789",
            "1n",
            "0x1n",
            "0o6n",
            "0b1n",
            "2_141_192_192",
            "0n",
        ];
        for n in NUMBERS {
            println!("n: {}", n);
            let mut t = Tokenizer::new(n);
            let item = t.next(true).unwrap();
            dbg!(&n[item.start..item.end]);
            assert!(matches!(item.ty, RawToken::Number(_)));
            assert!(t.stream.at_end());
        }
    }

    #[test]
    fn tokenizer_regex() {
        static REGEX: &[&str] = &[
            r#"/x/"#,
            r#"/|/"#,
            r#"/|||/"#,
            r#"/^$\b\B/"#,
            r#"/(?=(?!(?:(.))))/"#,
            r#"/a.\f\n\r\t\v\0\[\-\/\\\x00\u0000/"#,
            r#"/\d\D\s\S\w\W/"#,
            r#"/\ca\cb\cc\cd\ce\cf\cg\ch\ci\cj\ck\cl\cm\cn\co\cp\cq\cr\cs\ct\cu\cv\cw\cx\cy\cz/"#,
            r#"/\cA\cB\cC\cD\cE\cF\cG\cH\cI\cJ\cK\cL\cM\cN\cO\cP\cQ\cR\cS\cT\cU\cV\cW\cX\cY\cZ/"#,
            r#"/[a-z-]/"#,
            r#"/[^\b\-^]/"#,
            r#"/[/\]\\]/"#,
            r#"/./i"#,
            r#"/./g"#,
            r#"/./m"#,
            r#"/./igm"#,
            r#"/.*/"#,
            r#"/.*?/"#,
            r#"/.+/"#,
            r#"/.+?/"#,
            r#"/.?/"#,
            r#"/.??/"#,
            r#"/.{0}/"#,
            r#"/.{0,}/"#,
            r#"/.{0,0}/"#,
            r#"/=/"#,
            r#"/\u{12345}\u0F00/"#,
            r#"/a\/b/"#,
            r#"/\//"#,
            r#"/a/\u{12345}\u0F00"#,
            r#"/[\]\[\n\s\/]/"#,
        ];
        for r in REGEX {
            let mut t = Tokenizer::new(r);
            let next = t.next(true).unwrap();
            let item = t.next_regex(next.end - next.start).unwrap();
            assert!(matches!(item.ty, RawToken::RegEx(_)));
            assert!(t.stream.at_end());
        }
    }

    #[test]
    fn validated_regex() {
        pretty_env_logger::try_init().ok();
        const REGEX: &[&str] = &[
            r#"/([.+*?=^!:${}()[\]|/\\])/g"#,
            r#"/[\]\}\n\s\d\e\3]/"#,
            r#"/[\/\/\/\/\/\/\/\/\/\/\/]/"#,
        ];
        for (i, r) in REGEX.iter().enumerate() {
            let mut t = Tokenizer::new(r);
            let next = t.next(true).unwrap();
            let item = t.next_regex(next.end - next.start).unwrap();
            assert!(matches!(item.ty, RawToken::RegEx(_)));
            assert!(t.stream.at_end());
            let mut p = res_regex::RegexParser::new(&r[item.start..item.end]).unwrap_or_else(|e| {
                panic!("{}: {}", i, e);
            });
            p.validate().unwrap_or_else(|e| {
                panic!("{}: {}", i, e);
            });
        }
    }

    #[test]
    fn tokenizer_regex_term_in_class() {
        pretty_env_logger::try_init().ok();
        let regex = r#"/([.+*?=^!:${}()[\]|/\\])/g"#;
        let mut t = Tokenizer::new(regex);
        let next = t.next(true).unwrap();
        let item = t.next_regex(next.end - next.start).unwrap();
        assert_eq!(item.ty, RawToken::RegEx(26));
        assert_eq!(item.start, 0);
        assert_eq!(item.end, 27);
    }

    #[test]
    fn tokenizer_regex_out_of_order() {
        pretty_env_logger::try_init().ok();
        let regex = r#"/((?:[^BEGHLMOSWYZabcdhmswyz']+)|(?:'(?:[^']|'')*')|(?:G{1,5}|y{1,4}|Y{1,4}|M{1,5}|L{1,5}|w{1,2}|W{1}|d{1,2}|E{1,6}|c{1,6}|a{1,5}|b{1,5}|B{1,5}|h{1,2}|H{1,2}|m{1,2}|s{1,2}|S{1,3}|z{1,4}|Z{1,5}|O{1,4}))([\s\S]*)/"#;
        let mut t = Tokenizer::new(regex);
        let next = t.next(true).unwrap();
        let item = t.next_regex(next.end - next.start).unwrap();
        assert_eq!(item.ty, RawToken::RegEx(211));
        assert_eq!(item.start, 0);
        assert_eq!(item.end, 211);
    }

    #[test]
    #[should_panic = "new line in regex literal"]
    fn tokenizer_regex_new_line_negative() {
        let regex = "/a\\
                     ";
        let mut t = Tokenizer::new(regex);
        let next = t.next(true).unwrap();
        let _item = t.next_regex(next.end - next.start).unwrap();
    }
    #[test]
    #[should_panic = "new line in regex literal"]
    fn tokenizer_regex_line_term_negative() {
        let regex = "/a\r/";
        let mut t = Tokenizer::new(regex);
        let next = t.next(true).unwrap();
        let _item = t.next_regex(next.end - next.start).unwrap();
    }
    #[test]
    #[should_panic = "unterminated regex"]
    fn tokenizer_regex_unterm_negative() {
        let regex = "/asdf";
        let mut t = Tokenizer::new(regex);
        let next = t.next(true).unwrap();
        let _item = t.next_regex(next.end - next.start).unwrap();
    }

    #[test]
    fn tokenizer_template() {
        let subbed = "`things and stuff times ${} and animals and minerals`";
        println!("subbed: {}", subbed);
        let mut t = Tokenizer::new(subbed);
        let start = t.next(true).unwrap();
        check_temp(&start.ty, TemplateKind::Head);
        let end = t.next(true).unwrap();
        check_temp(&end.ty, TemplateKind::Tail);
        assert!(t.stream.at_end());
        let no_sub = "`things and stuff`";
        println!("no_sub {}", no_sub);
        t = Tokenizer::new(no_sub);
        let one = t.next(true).unwrap();
        check_temp(&one.ty, TemplateKind::NoSub);
        assert!(t.stream.at_end());
        let escaped_sub = r#"`\0\n\x0A\u000A\u{A}${}`"#;
        println!("escaped_sub: {}", escaped_sub);
        t = Tokenizer::new(escaped_sub);
        let start = t.next(true).unwrap();
        check_temp(&start.ty, TemplateKind::Head);
        let end = t.next(true).unwrap();
        check_temp(&end.ty, TemplateKind::Tail);
        assert!(t.stream.at_end());
        let escaped_no_sub = r#"`a\${b`"#;
        println!("escaped_no_sub: {}", escaped_no_sub);
        t = Tokenizer::new(escaped_no_sub);
        let one = t.next(true).unwrap();
        check_temp(&one.ty, TemplateKind::NoSub);
        assert!(t.stream.at_end());
        let double_sub =
            "`things and stuff times ${} and animals and minerals ${} and places and people`";
        println!("double_sub: {}", double_sub);
        t = Tokenizer::new(double_sub);
        let start = t.next(true).unwrap();
        check_temp(&start.ty, TemplateKind::Head);
        let mid = t.next(true).unwrap();
        check_temp(&mid.ty, TemplateKind::Body);
        let end = t.next(true).unwrap();
        check_temp(&end.ty, TemplateKind::Tail);
        assert!(t.stream.at_end());
    }
    #[test]
    fn invalid_octal_unicode_template() {
        let escaped = "(x=>{

            })`\\u0g`;";
        let mut t = Tokenizer::new(escaped);
        let _open_paren = t.next(true).unwrap();
        let _x = t.next(true).unwrap();
        let _arrow = t.next(true).unwrap();
        let _open_brace = t.next(true).unwrap();
        t.skip_whitespace();
        let _close_brace = t.next(true).unwrap();
        let _close_paren = t.next(true).unwrap();

        let temp = t.next(true).unwrap();
        if let RawToken::Template {
            kind,
            found_invalid_unicode_escape,
            ..
        } = &temp.ty
        {
            assert_eq!(&TemplateKind::NoSub, kind);
            assert!(*found_invalid_unicode_escape);
        } else {
            unreachable!()
        }
        let _semi = t.next(true).unwrap();
    }

    fn check_temp(temp: &RawToken, expected_kind: TemplateKind) {
        if let RawToken::Template { kind, .. } = temp {
            assert_eq!(kind, &expected_kind);
        }
    }

    #[test]
    fn oct_escape_template() {
        let mut t = Tokenizer::new(r#"`\01`"#);
        let temp = t.next(true).unwrap();
        if let RawToken::Template {
            kind,
            has_octal_escape,
            ..
        } = temp.ty
        {
            assert_eq!(kind, TemplateKind::NoSub);
            assert!(has_octal_escape);
        }
    }

    #[test]
    #[should_panic = "unterminated template"]
    fn untermed_template() {
        let mut t = Tokenizer::new(r#"`asdf"#);
        t.next(true).unwrap();
    }
    #[test]
    fn template_new_lines() {
        let ts = &[
            "`asdf\r\nasdf`",
            "`asdf\rasdf`",
            "`asdf\nasdf`",
            "`asdf\u{2028}asdf`",
            "`asdf\u{2029}asdf`",
        ];
        for template in ts {
            let mut t = Tokenizer::new(template);
            let temp = t.next(true).unwrap();
            if let RawToken::Template {
                kind,
                new_line_count,
                last_len,
                ..
            } = temp.ty
            {
                assert_eq!(kind, TemplateKind::NoSub);
                assert!(new_line_count == 1);
                assert!(last_len == 5, "{}", last_len);
            }
        }
    }

    #[test]
    fn invalid_unicode_escape_template() {
        let ts = &[r#"`\u{FFFFFFF}`"#, r#"`\u{AAA`"#, r#"`\u{AAG}`"#];
        for template in ts {
            let mut t = Tokenizer::new(template);
            let temp = t.next(true).unwrap();
            if let RawToken::Template {
                kind,
                found_invalid_unicode_escape,
                ..
            } = temp.ty
            {
                assert_eq!(kind, TemplateKind::NoSub);
                assert!(found_invalid_unicode_escape)
            }
        }
    }

    #[test]
    fn tokenizer_bools() {
        for b in &["true", "false"] {
            let mut t = Tokenizer::new(b);
            let item = t.next(true).unwrap();
            assert!(matches!(item.ty, RawToken::Boolean(_)));
            assert!(t.stream.at_end());
        }
    }

    #[test]
    fn tokenizer_null() {
        let mut t = Tokenizer::new("null");
        let item = t.next(true).unwrap();
        assert_eq!(item.ty, RawToken::Null);
        assert!(t.stream.at_end());
    }

    #[test]
    fn tokenizer_keyword() {
        static KEYWORDS: &[&str] = &[
            "implements",
            "interface",
            "package",
            "private",
            "protected",
            "public",
            "static",
            "yield",
            "let",
            "enum",
            "export",
            "import",
            "super",
            "break",
            "case",
            "catch",
            "continue",
            "debugger",
            "default",
            "delete",
            "do",
            "else",
            "finally",
            "for",
            "function",
            "if",
            "instanceof",
            "in",
            "new",
            "return",
            "switch",
            "this",
            "throw",
            "try",
            "typeof",
            "var",
            "void",
            "while",
            "with",
        ];
        for k in KEYWORDS {
            let mut t = Tokenizer::new(k);
            let item = t.next(true).unwrap();
            match item.ty {
                RawToken::Keyword(_) => (),
                _ => panic!("{} was not parsed as a keyword", k),
            }
            assert!(t.stream.at_end());
        }
    }

    #[test]
    fn tokenizer_comments() {
        static COMMENTS: &[&str] = &[
            "//this is a comment",
            "/*this is a
        multi-line comment*/",
            "<!-- This is an HTML comment -->",
            "<!-- This is an HTML comment --> with a trailer",
            "/*multi-line comment */-->with a trailer",
            "/*\nmulti-line\rweird\u{2028}new\u{2029}lines\r\n*/",
            "/*multi-line with embedded html <!-- this is not a unique comment -->*/",
        ];
        for c in COMMENTS {
            let mut t = Tokenizer::new(c);
            let item = t.next(true).unwrap();
            assert!(item.ty.is_comment());
            assert!(t.stream.at_end());
        }
    }

    #[test]
    fn tokenizer_html_comment() {
        static SUCCESS_COMMENTS: &[&str] = &[
            "<!--line feed\n",
            "<!--carriage return\r",
            "<!--crlf\r\n",
            "<!--line separator\u{2028}",
            "<!--paragraph separator\u{2029}",
            "<!--normally terminated-->",
        ];
        static FAIL_COMMENTS: &[&str] = &["<!--this will fail", "hello world"];
        for c in SUCCESS_COMMENTS {
            let mut t = Tokenizer::new(c);
            let item = t.next(true).unwrap();
            assert!(item.ty.is_comment());
        }
        for c in FAIL_COMMENTS {
            let mut t = Tokenizer::new(c);
            match t.next(true) {
                Err(_) => continue,
                Ok(item) => assert!(!item.ty.is_comment()),
            };
        }
    }

    #[test]
    fn tokenizer_white_space() {
        let js = "0
0 0
0 0
0 0 0 0 0";
        let mut t = Tokenizer::new(js);
        let _ = t.next(true);
        assert_eq!(t.skip_whitespace().0, 1); //\n
        let _ = t.next(true);
        assert_eq!(t.skip_whitespace().0, 0);
        let _ = t.next(true);
        assert_eq!(t.skip_whitespace().0, 1); //\r
        let _ = t.next(true);
        assert_eq!(t.skip_whitespace().0, 0);
        let _ = t.next(true);
        assert_eq!(t.skip_whitespace().0, 1); //\r\n
        let _ = t.next(true);
        assert_eq!(t.skip_whitespace().0, 0);
        let _ = t.next(true);
        assert_eq!(t.skip_whitespace().0, 1); // line separator
        let _ = t.next(true);
        assert_eq!(t.skip_whitespace().0, 0);
        let _ = t.next(true);
        assert_eq!(t.skip_whitespace().0, 1); // paragraph separator
    }

    #[test]
    #[should_panic = "escaped unicode codepoint too large"]
    fn char_too_large() {
        let js = r"asdf\u{FFFFFFF}";
        let mut t = Tokenizer::new(js);
        dbg!(t.next(true)).unwrap();
    }

    #[test]
    fn char_not_start() {
        let tests = [r"\u{30}", r"\u0030"];

        for i in &tests {
            let mut t = Tokenizer::new(i);
            assert!(t.next(true).is_err());
        }
    }

    #[test]
    fn char_not_continue() {
        let tests = [r"abcd\u{8a}", r"abcd\u008a"];

        for i in &tests {
            let mut t = Tokenizer::new(i);
            assert!(t.next(true).is_err());
        }
    }
    #[test]
    #[should_panic = "escaped unicode code points must end in }"]
    fn untermed_codepoint() {
        let mut t = Tokenizer::new(r#"\u{0000"#);
        t.next(true).unwrap();
    }
    #[test]
    #[should_panic = "escaped unicode code point contains a non-hex digit"]
    fn codepoint_not_hex() {
        let mut t = Tokenizer::new(r#"\u{11G}"#);
        t.next(true).unwrap();
    }
    #[test]
    #[should_panic = "escaped unicode code point is not a hex digit"]
    fn char_code_not_hex() {
        let mut t = Tokenizer::new(r#"\u11G0"#);
        t.next(true).unwrap();
    }
    #[test]
    #[should_panic = "escaped unicode char code is not a hex digit"]
    fn start_char_code_not_hex() {
        let mut t = Tokenizer::new(r#"\uG1G0"#);
        t.next(true).unwrap();
    }
    #[test]
    #[should_panic = "escaped unicode sequence does not have 4 characters"]
    fn char_code_short() {
        let mut t = Tokenizer::new(r#"\u010"#);
        t.next(true).unwrap();
    }
    #[test]
    #[should_panic = "unescaped new line in string literal"]
    fn unescaped_carriage_return_in_str() {
        let mut t = Tokenizer::new("'asdf\r'");
        t.next(true).unwrap();
    }
    #[test]
    #[should_panic = "Invalid escape in string literal"]
    fn short_unicode_escape_in_str() {
        let mut t = Tokenizer::new(r#"'\u"#);
        t.next(true).unwrap();
    }
    #[test]
    #[should_panic = "unterminated string literal"]
    fn untermed_str_lit() {
        let mut t = Tokenizer::new("'asdf");
        t.next(true).unwrap();
    }
    #[test]
    #[should_panic = "unknown punct"]
    fn unknown_punct() {
        let mut t = Tokenizer::new("¬");
        t.next(true).unwrap();
    }
    #[test]
    #[should_panic = "empty hex literal"]
    fn empty_hex() {
        let mut t = Tokenizer::new("0x");
        t.next(true).unwrap();
    }
    #[test]
    #[should_panic = "empty hex literal"]
    fn empty_hex2() {
        let mut t = Tokenizer::new("0x;");
        t.next(true).unwrap();
    }
    #[test]
    #[should_panic = "empty octal literal"]
    fn empty_oct() {
        let mut t = Tokenizer::new("0o");
        t.next(true).unwrap();
    }
    #[test]
    #[should_panic = "empty octal literal"]
    fn empty_oct2() {
        let mut t = Tokenizer::new("0o;");
        t.next(true).unwrap();
    }
    #[test]
    #[should_panic = "empty binary literal"]
    fn empty_bin() {
        let mut t = Tokenizer::new("0b");
        t.next(true).unwrap();
    }
    #[test]
    #[should_panic = "empty binary literal"]
    fn empty_bin2() {
        let mut t = Tokenizer::new("0b;");
        t.next(true).unwrap();
    }

    #[test]
    #[should_panic = "Invalid decimal, exponents must be followed by +, - or decimal digits"]
    fn empty_exp() {
        let mut t = Tokenizer::new("1e;");
        t.next(true).unwrap();
    }
    #[test]
    #[should_panic = "Invalid decimal, Floats cannot be BigInts"]
    fn float_big_int() {
        let mut t = Tokenizer::new("1e3n;");
        t.next(true).unwrap();
    }

    #[test]
    #[should_panic = "Invalid decimal. Numbers cannot end with an underscore"]
    fn trailing_sep() {
        let mut t = Tokenizer::new("1_;");
        t.next(true).unwrap();
    }

    #[test]
    #[should_panic = "double numeric separator"]
    fn rep_sep() {
        let mut t = Tokenizer::new("1__1;");
        t.next(true).unwrap();
    }

    #[test]
    fn template_octal() {
        let mut t = Tokenizer::new(r#"`a\7`"#);
        let item = t.next(true).unwrap();
        if let RawToken::Template {
            has_octal_escape, ..
        } = item.ty
        {
            assert!(has_octal_escape);
        }
    }

    #[test]
    fn template_invalid_unicode_char_code() {
        let mut t = Tokenizer::new(r#"`asdf\u99T`"#);
        let item = t.next(true).unwrap();
        if let RawToken::Template {
            found_invalid_unicode_escape,
            ..
        } = item.ty
        {
            assert!(found_invalid_unicode_escape);
        }
    }
    #[test]
    fn template_invalid_unicode_char_code2() {
        let mut t = Tokenizer::new(r#"`asdf\uT`"#);
        let item = t.next(true).unwrap();
        if let RawToken::Template {
            found_invalid_unicode_escape,
            ..
        } = item.ty
        {
            assert!(found_invalid_unicode_escape);
        }
    }
    #[test]
    #[should_panic = "Invalid escape sequence in template literal"]
    fn template_escape_u() {
        let mut t = Tokenizer::new(r#"`asdf\u"#);
        let item = t.next(true).unwrap();
        if let RawToken::Template {
            found_invalid_unicode_escape,
            ..
        } = item.ty
        {
            assert!(found_invalid_unicode_escape);
        }
    }

    #[test]
    fn template_with_valid_hex() {
        let mut t = Tokenizer::new(r#"`\x0A`"#);
        let item = t.next(true).unwrap();
        if let RawToken::Template {
            found_invalid_hex_escape,
            ..
        } = item.ty
        {
            assert!(!found_invalid_hex_escape);
        }
    }
    #[test]
    fn template_with_invalid_hex() {
        let mut t = Tokenizer::new(r#"`\x0G`"#);
        let item = t.next(true).unwrap();
        if let RawToken::Template {
            found_invalid_hex_escape,
            ..
        } = item.ty
        {
            assert!(found_invalid_hex_escape);
        }
    }

    #[test]
    #[should_panic = "Number literal cannot be immediately followed by an identifier"]
    fn number_followed_by_ident_start() {
        let mut t = Tokenizer::new("1234.56e78in []");
        t.next(true).unwrap();
    }

    #[test]
    fn regex_with_single_multi_code_point_char() {
        let regex = "/\u{a0}/";
        let mut t = Tokenizer::new(regex);
        let slash = t.next(true).unwrap();
        assert_eq!(
            slash,
            RawItem {
                end: 1,
                start: 0,
                ty: RawToken::Punct(Punct::ForwardSlash),
            }
        );
        let regex_tail = t.next_regex(slash.end - slash.start).unwrap();
        assert_eq!(
            regex_tail,
            RawItem {
                end: regex.len(),
                start: 0,
                ty: RawToken::RegEx(4),
            }
        )
    }
}
