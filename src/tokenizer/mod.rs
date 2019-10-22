use crate::tokens::{CommentKind, Keyword, NumberKind, Punct};
use crate::{is_line_term, OpenCurlyKind};
mod buffer;
mod tokens;
mod unicode;
pub use self::tokens::{RawToken, StringKind, TemplateKind};
use crate::error::RawError;
use unicode::{is_id_continue, is_id_start};
pub(crate) type Res<T> = Result<T, RawError>;

#[derive(Debug)]
pub struct RawItem {
    pub ty: tokens::RawToken,
    pub start: usize,
    pub end: usize,
}

pub struct Tokenizer<'a> {
    pub(super) stream: buffer::JSBuffer<'a>,
    pub(super) current_start: usize,
    pub(super) curly_stack: Vec<OpenCurlyKind>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(stream: &'a str) -> Self {
        Tokenizer {
            current_start: 0,
            stream: stream.into(),
            curly_stack: Vec::with_capacity(2),
        }
    }

    pub fn next(&mut self) -> Res<RawItem> {
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
        if Self::is_id_start(next_char) {
            return self.ident(next_char);
        }
        if next_char == '"' || next_char == '\'' {
            return self.string(next_char);
        }
        if next_char == '(' || next_char == ')' || next_char == ';' {
            return self.punct(next_char);
        }
        if next_char.is_digit(10) {
            return self.number(next_char);
        }
        if next_char == '`' {
            return self.template(next_char);
        }
        if next_char == '}' && self.curly_stack.last() == Some(&OpenCurlyKind::Template) {
            self.curly_stack.pop();
            return self.template(next_char);
        }
        self.punct(next_char)
    }

    pub fn next_regex(&mut self, start_len: usize) -> Res<RawItem> {
        self.current_start = self.stream.idx;
        let mut end_of_body = false;
        let mut body_idx = 0;
        if self.look_ahead_matches("\\/") {
            self.stream.skip(2);
        }
        let mut in_class = false;
        while let Some(c) = self.stream.next_char() {
            if end_of_body {
                if c == '\\' {
                    if self.look_ahead_byte_matches('u') {
                        // unicode escape
                        self.stream.skip(1);
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
                } else if self.look_ahead_byte_matches('[')
                    || self.look_ahead_byte_matches('/')
                    || self.look_ahead_byte_matches('\\')
                {
                    self.stream.skip(1);
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

    fn ident(&mut self, start: char) -> Res<RawItem> {
        if start == '\\' {
            // TODO validate escaped ident start
            self.escaped_ident_part()?;
        }
        while let Some(c) = self.stream.next_char() {
            if c == '\\' {
                self.escaped_ident_part()?;
            }
            if !Self::is_id_continue(c) && c != '\u{200C}' && c != '\u{200D}' {
                // if we have moved past the last valid identifier, go back 1
                let _ = self.stream.prev_char();
                break;
            }
        }
        if let Some(k) = self.at_keyword() {
            self.gen_token(k)
        } else {
            self.gen_token(RawToken::Ident)
        }
    }
    /// Includes keywords, booleans & null
    fn at_keyword(&self) -> Option<RawToken> {
        let ident = &self.stream.buffer[self.current_start..self.stream.idx];
        match self.stream.idx - self.current_start {
            2 if ident == b"do" => Some(RawToken::Keyword(Keyword::Do)),
            2 if ident == b"if" => Some(RawToken::Keyword(Keyword::If)),
            2 if ident == b"in" => Some(RawToken::Keyword(Keyword::In)),
            3 if ident == b"for" => Some(RawToken::Keyword(Keyword::For)),
            3 if ident == b"new" => Some(RawToken::Keyword(Keyword::New)),
            3 if ident == b"try" => Some(RawToken::Keyword(Keyword::Try)),
            3 if ident == b"var" => Some(RawToken::Keyword(Keyword::Var)),
            3 if ident == b"let" => Some(RawToken::Keyword(Keyword::Let)),
            4 if ident == b"case" => Some(RawToken::Keyword(Keyword::Case)),
            4 if ident == b"this" => Some(RawToken::Keyword(Keyword::This)),
            4 if ident == b"void" => Some(RawToken::Keyword(Keyword::Void)),
            4 if ident == b"with" => Some(RawToken::Keyword(Keyword::With)),
            4 if ident == b"enum" => Some(RawToken::Keyword(Keyword::Enum)),
            4 if ident == b"else" => Some(RawToken::Keyword(Keyword::Else)),
            4 if ident == b"true" => Some(RawToken::Boolean(true)),
            4 if ident == b"null" => Some(RawToken::Null),
            5 if ident == b"await" => Some(RawToken::Keyword(Keyword::Await)),
            5 if ident == b"break" => Some(RawToken::Keyword(Keyword::Break)),
            5 if ident == b"catch" => Some(RawToken::Keyword(Keyword::Catch)),
            5 if ident == b"class" => Some(RawToken::Keyword(Keyword::Class)),
            5 if ident == b"const" => Some(RawToken::Keyword(Keyword::Const)),
            5 if ident == b"throw" => Some(RawToken::Keyword(Keyword::Throw)),
            5 if ident == b"while" => Some(RawToken::Keyword(Keyword::While)),
            5 if ident == b"super" => Some(RawToken::Keyword(Keyword::Super)),
            5 if ident == b"yield" => Some(RawToken::Keyword(Keyword::Yield)),
            5 if ident == b"false" => Some(RawToken::Boolean(false)),
            6 if ident == b"delete" => Some(RawToken::Keyword(Keyword::Delete)),
            6 if ident == b"return" => Some(RawToken::Keyword(Keyword::Return)),
            6 if ident == b"switch" => Some(RawToken::Keyword(Keyword::Switch)),
            6 if ident == b"typeof" => Some(RawToken::Keyword(Keyword::TypeOf)),
            6 if ident == b"export" => Some(RawToken::Keyword(Keyword::Export)),
            6 if ident == b"import" => Some(RawToken::Keyword(Keyword::Import)),
            6 if ident == b"static" => Some(RawToken::Keyword(Keyword::Static)),
            6 if ident == b"public" => Some(RawToken::Keyword(Keyword::Public)),
            7 if ident == b"default" => Some(RawToken::Keyword(Keyword::Default)),
            7 if ident == b"finally" => Some(RawToken::Keyword(Keyword::Finally)),
            7 if ident == b"package" => Some(RawToken::Keyword(Keyword::Package)),
            7 if ident == b"private" => Some(RawToken::Keyword(Keyword::Private)),
            8 if ident == b"continue" => Some(RawToken::Keyword(Keyword::Continue)),
            8 if ident == b"debugger" => Some(RawToken::Keyword(Keyword::Debugger)),
            8 if ident == b"function" => Some(RawToken::Keyword(Keyword::Function)),
            9 if ident == b"interface" => Some(RawToken::Keyword(Keyword::Interface)),
            9 if ident == b"protected" => Some(RawToken::Keyword(Keyword::Protected)),
            10 if ident == b"instanceof" => Some(RawToken::Keyword(Keyword::InstanceOf)),
            10 if ident == b"implements" => Some(RawToken::Keyword(Keyword::Implements)),
            _ => None,
        }
    }
    /// picking up after the \
    #[inline]
    fn escaped_ident_part(&mut self) -> Res<()> {
        if let Some('u') = self.stream.next_char() {
            if let Some(c) = self.stream.next_char() {
                if c == '{' {
                    self.escaped_with_code_point()?;
                } else {
                    self.escaped_with_hex4(c)?;
                }
            }
            Ok(())
        } else {
            Err(RawError {
                msg: "invalid unicode escape sequence in identifier".to_string(),
                idx: self.current_start,
            })
        }
    }
    #[inline]
    fn escaped_with_code_point(&mut self) -> Res<()> {
        let mut code: u32 = 0;
        let mut last_char: char = '{';
        while let Some(c) = self.stream.next_char() {
            last_char = c;
            if c == '}' {
                break;
            }
            if !c.is_digit(16) {
                return Err(RawError {
                    msg: "escaped unicode code point is not a hex digit".to_string(),
                    idx: self.stream.idx,
                });
            }

            code += match u32::from_str_radix(c.encode_utf8(&mut [0; 4]), 16) {
                Ok(n) => n,
                Err(e) => return Err(RawError {
                    msg: format!("escaped unicode code point could not be converted to a u32 with the error {}", e),
                    idx: self.stream.idx,
                })
            }
        }
        if code > 0x10FFF {
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
            Ok(())
        }
    }
    #[inline]
    fn escaped_with_hex4(&mut self, start: char) -> Res<()> {
        if !start.is_digit(16) {
            return Err(RawError {
                msg: "escaped unicode char code is not a hex digit".to_string(),
                idx: self.stream.idx,
            });
        }
        for _ in 0..3 {
            if let Some(c) = self.stream.next_char() {
                if !c.is_digit(16) {
                    return Err(RawError {
                        msg: "escaped unicode char code is not a hex digit".to_string(),
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
        Ok(())
    }

    fn string(&mut self, quote: char) -> Res<RawItem> {
        let mut escaped = false;
        let mut last_len = 1usize; // we already skipped the quote char
        let mut new_line_count = 0usize;
        while let Some(c) = self.stream.next_char() {
            if c == '\\' {
                if escaped {
                    escaped = false;
                } else {
                    escaped = true;
                }
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
                    self.stream.skip(1);
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
                    });
                }
                escaped = false;
            } else {
                last_len = last_len.saturating_add(1);
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
    fn punct(&mut self, c: char) -> Res<RawItem> {
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
            '-' => self.minus(),
            '/' => self.forward_slash(),
            '%' => self.percent(),
            '^' => self.caret(),
            _ => Err(RawError {
                msg: format!("unknown punct {:?}", c),
                idx: self.current_start,
            }),
        }
    }
    #[inline]
    fn open_curly(&mut self, curly: OpenCurlyKind, punct: Punct) -> Res<RawItem> {
        self.curly_stack.push(curly);
        self.gen_punct(punct)
    }
    #[inline]
    fn close_curly(&mut self, punct: Punct) -> Res<RawItem> {
        let _ = self.curly_stack.pop();
        self.gen_punct(punct)
    }
    #[inline]
    fn period(&mut self) -> Res<RawItem> {
        if self.look_ahead_matches("..") {
            self.stream.skip(2);
            self.gen_punct(Punct::Ellipsis)
        } else if self.stream.at_decimal() {
            self.dec_number(true, '.')
        } else {
            self.gen_punct(Punct::Period)
        }
    }
    #[inline]
    fn greater_than(&mut self) -> Res<RawItem> {
        if self.look_ahead_matches(">>=") {
            self.stream.skip(3);
            self.gen_punct(Punct::TripleGreaterThanEqual)
        } else if self.look_ahead_matches(">>") {
            self.stream.skip(2);
            self.gen_punct(Punct::TripleGreaterThan)
        } else if self.look_ahead_matches(">=") {
            self.stream.skip(2);
            self.gen_punct(Punct::DoubleGreaterThanEqual)
        } else if self.look_ahead_byte_matches('>') {
            self.stream.skip(1);
            self.gen_punct(Punct::DoubleGreaterThan)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip(1);
            self.gen_punct(Punct::GreaterThanEqual)
        } else {
            self.gen_punct(Punct::GreaterThan)
        }
    }
    #[inline]
    fn less_than(&mut self) -> Res<RawItem> {
        if self.look_ahead_matches("<=") {
            self.stream.skip(2);
            self.gen_punct(Punct::DoubleLessThanEqual)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip(1);
            self.gen_punct(Punct::LessThanEqual)
        } else if self.look_ahead_byte_matches('<') {
            self.stream.skip(1);
            self.gen_punct(Punct::DoubleLessThan)
        } else if self.look_ahead_matches("!--") {
            self.stream.skip(3);
            self.html_comment()
        } else {
            self.gen_punct(Punct::LessThan)
        }
    }
    #[inline]
    fn equals(&mut self) -> Res<RawItem> {
        if self.look_ahead_matches("==") {
            self.stream.skip(2);
            self.gen_punct(Punct::TripleEqual)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip(1);
            self.gen_punct(Punct::DoubleEqual)
        } else if self.look_ahead_byte_matches('>') {
            self.stream.skip(1);
            self.gen_punct(Punct::EqualGreaterThan)
        } else {
            self.gen_punct(Punct::Equal)
        }
    }
    #[inline]
    fn bang(&mut self) -> Res<RawItem> {
        if self.look_ahead_matches("==") {
            self.stream.skip(2);
            self.gen_punct(Punct::BangDoubleEqual)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip(1);
            self.gen_punct(Punct::BangEqual)
        } else {
            self.gen_punct(Punct::Bang)
        }
    }
    #[inline]
    fn asterisk(&mut self) -> Res<RawItem> {
        if self.look_ahead_matches("*=") {
            self.stream.skip(2);
            self.gen_punct(Punct::DoubleAsteriskEqual)
        } else if self.look_ahead_byte_matches('*') {
            self.stream.skip(1);
            self.gen_punct(Punct::DoubleAsterisk)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip(1);
            self.gen_punct(Punct::AsteriskEqual)
        } else {
            self.gen_punct(Punct::Asterisk)
        }
    }
    #[inline]
    fn ampersand(&mut self) -> Res<RawItem> {
        if self.look_ahead_byte_matches('&') {
            self.stream.skip(1);
            self.gen_punct(Punct::DoubleAmpersand)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip(1);
            self.gen_punct(Punct::AmpersandEqual)
        } else {
            self.gen_punct(Punct::Ampersand)
        }
    }
    #[inline]
    fn pipe(&mut self) -> Res<RawItem> {
        if self.look_ahead_byte_matches('|') {
            self.stream.skip(1);
            self.gen_punct(Punct::DoublePipe)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip(1);
            self.gen_punct(Punct::PipeEqual)
        } else {
            self.gen_punct(Punct::Pipe)
        }
    }
    #[inline]
    fn plus(&mut self) -> Res<RawItem> {
        if self.look_ahead_byte_matches('+') {
            self.stream.skip(1);
            self.gen_punct(Punct::DoublePlus)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip(1);
            self.gen_punct(Punct::PlusEqual)
        } else {
            self.gen_punct(Punct::Plus)
        }
    }
    #[inline]
    fn minus(&mut self) -> Res<RawItem> {
        if self.look_ahead_byte_matches('-') {
            self.stream.skip(1);
            self.gen_punct(Punct::DoubleDash)
        } else if self.look_ahead_byte_matches('=') {
            self.stream.skip(1);
            self.gen_punct(Punct::DashEqual)
        } else {
            self.gen_punct(Punct::Dash)
        }
    }
    #[inline]
    fn forward_slash(&mut self) -> Res<RawItem> {
        if self.look_ahead_byte_matches('=') {
            self.stream.skip(1);
            self.gen_punct(Punct::ForwardSlashEqual)
        } else if self.look_ahead_byte_matches('*') {
            self.multi_comment()
        } else if self.look_ahead_byte_matches('/') {
            self.single_comment()
        } else {
            self.gen_punct(Punct::ForwardSlash)
        }
    }
    #[inline]
    fn percent(&mut self) -> Res<RawItem> {
        if self.look_ahead_byte_matches('=') {
            self.stream.skip(1);
            self.gen_punct(Punct::PercentEqual)
        } else {
            self.gen_punct(Punct::Percent)
        }
    }
    #[inline]
    fn caret(&mut self) -> Res<RawItem> {
        if self.look_ahead_byte_matches('=') {
            self.stream.skip(1);
            self.gen_punct(Punct::CaretEqual)
        } else {
            self.gen_punct(Punct::Caret)
        }
    }
    fn hash(&mut self) -> Res<RawItem> {
        // hashbang comment can only appear at the start
        if self.current_start == 0 && self.look_ahead_byte_matches('!') {
            while !self.at_new_line() {
                if self.stream.next_char().is_none() {
                    break;
                }
            }
            self.gen_comment(CommentKind::Hashbang, 0, 0)
        } else {
            self.gen_punct(Punct::Hash)
        }
    }
    fn number(&mut self, start: char) -> Res<RawItem> {
        if start != '.' {
            if let Some(next) = self.stream.next_char() {
                if start == '0' {
                    if next.eq_ignore_ascii_case(&'x') {
                        self.hex_number()
                    } else if next.eq_ignore_ascii_case(&'o') {
                        self.oct_number()
                    } else if next.eq_ignore_ascii_case(&'b') {
                        self.bin_number()
                    } else if next.is_digit(10) {
                        self.dec_number(false, next)
                    } else if next == '.' {
                        self.dec_number(true, next)
                    } else {
                        let _ = self.stream.prev_char();
                        self.dec_number(false, next)
                    }
                } else if next == '.' {
                    self.dec_number(true, next)
                } else {
                    let _ = self.stream.prev_char();
                    self.dec_number(next == '.', next)
                }
            } else {
                self.gen_number(NumberKind::Dec)
            }
        } else {
            self.punct(start)
        }
    }
    fn template(&mut self, start: char) -> Res<RawItem> {
        let mut line_count = 0usize;
        let mut last_len = 1usize; // we already skipped the start char
        while let Some(c) = self.stream.next_char() {
            last_len = last_len.saturating_add(1);
            if c == '\\' {
                if self.look_ahead_matches("${") {
                    self.stream.skip(2);
                } else if self.look_ahead_byte_matches('`') || self.look_ahead_byte_matches('\\') {
                    self.stream.skip(1);
                } else if self.look_ahead_byte_matches('0') {
                    if let Some(_zero) = self.stream.next_char() {
                        if self.stream.at_decimal() {
                            return Err(RawError {
                                msg: format!(
                                    "Template contains invalid octal literal \\0{}",
                                    self.stream.next_char().unwrap_or('?')
                                ),
                                idx: self.stream.idx.saturating_sub(1),
                            });
                        } else {
                            let _ = self.stream.prev_char();
                        }
                    }
                } else if self.stream.at_octal() {
                    return Err(RawError {
                        msg: format!(
                            "Template contains invalid octal literal \\0{}",
                            self.stream.next_char().unwrap_or('?')
                        ),
                        idx: self.stream.idx.saturating_sub(1),
                    });
                }
            } else if c == '\r' {
                if self.look_ahead_byte_matches('\n') {
                    self.stream.skip(1);
                }
                line_count = line_count.saturating_add(1);
                last_len = 0;
            } else if Self::is_new_line_not_cr(c) {
                line_count = line_count.saturating_add(1);
                last_len = 0;
            } else if c == '$' {
                if self.look_ahead_byte_matches('{') {
                    self.stream.skip(1);
                    self.curly_stack.push(OpenCurlyKind::Template);
                    if start == '`' {
                        return self.gen_template(TemplateKind::Head, line_count, last_len);
                    } else {
                        return self.gen_template(TemplateKind::Body, line_count, last_len);
                    }
                }
            } else if c == '`' {
                if start == '`' {
                    return self.gen_template(TemplateKind::NoSub, line_count, last_len);
                } else {
                    return self.gen_template(TemplateKind::Tail, line_count, last_len);
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
    #[inline]
    fn single_comment(&mut self) -> Res<RawItem> {
        while !self.at_new_line() {
            if self.stream.next_char().is_none() {
                break;
            }
        }
        self.gen_comment(CommentKind::Single, 0, 0)
    }
    #[inline]
    fn multi_comment(&mut self) -> Res<RawItem> {
        let mut new_line_count = 0usize;
        let mut last_len = 1usize; // we already skipped the /
        while let Some(c) = self.stream.next_char() {
            if c == '*' && self.look_ahead_byte_matches('/') {
                self.stream.skip(1);
                return self.gen_comment(
                    CommentKind::Multi,
                    new_line_count,
                    last_len.saturating_add(2),
                );
            } else if c == '\r' {
                if self.look_ahead_byte_matches('\n') {
                    self.stream.skip(1);
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
        Err(RawError {
            idx: self.current_start,
            msg: "unterminated multi-line comment".to_string(),
        })
    }
    #[inline]
    fn html_comment(&mut self) -> Res<RawItem> {
        let mut found_end = false;
        while !self.stream.at_end() {
            if self.stream.at_new_line() {
                    found_end = true;
                    break;
            }
            
            if self.look_ahead_matches("-->") {
                found_end = true;
                self.stream.skip(3);
            } else {    
                self.stream.skip(1);
            }
        }
        if found_end {
            return self.gen_comment(CommentKind::Html, 0, 0);
        }
        Err(RawError {
            msg: "unterminated html comment".to_string(),
            idx: self.current_start,
        })
    }
    #[inline]
    fn hex_number(&mut self) -> Res<RawItem> {
        let mut prev_char = if let Some(c) = self.stream.next_char() {
            if !c.is_digit(16) {
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
        while let Some(c) = self.stream.next_char() {
            self.check_repeating_underscore(prev_char, c)?;
            if !c.is_digit(16) && c != '_' {
                let _ = self.stream.prev_char();
                break;
            }
            prev_char = c;
        }
        self.check_trailing_underscore(prev_char)?;
        self.gen_number(NumberKind::Hex)
    }
    #[inline]
    fn oct_number(&mut self) -> Res<RawItem> {
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
        while let Some(c) = self.stream.next_char() {
            self.check_repeating_underscore(prev_char, c)?;
            if !c.is_digit(8) && c != '_' {
                let _ = self.stream.prev_char();
                break;
            }
            prev_char = c;
        }
        self.check_trailing_underscore(prev_char)?;
        self.gen_number(NumberKind::Oct)
    }

    #[inline]
    fn bin_number(&mut self) -> Res<RawItem> {
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
        while let Some(c) = self.stream.next_char() {
            self.check_repeating_underscore(prev_char, c)?;
            if !c.is_digit(2) && c != '_' {
                let _ = self.stream.prev_char();
                break;
            }
            prev_char = c;
        }
        self.check_trailing_underscore(prev_char)?;
        self.gen_number(NumberKind::Bin)
    }
    #[inline]
    fn dec_number(&mut self, seen_point: bool, mut prev_char: char) -> Res<RawItem> {
        prev_char = self.consume_digits(10, prev_char)?;

        if !seen_point && self.look_ahead_byte_matches('.') {
            self.stream.skip(1);
            prev_char = self.consume_digits(10, '.')?;
        }
        // if we find e || E, prev_char != _
        // allow for + or - next
        // at least one number is required next
        // go back to step 1
        if self.look_ahead_byte_matches('e') || self.look_ahead_byte_matches('E') {
            self.stream.skip(1);
            prev_char = 'e';            
            if self.look_ahead_byte_matches('-') || self.look_ahead_byte_matches('+') {
                self.stream.skip(1);
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

        self.check_trailing_underscore(prev_char)?;
        self.gen_number(NumberKind::Dec)
    }

    fn consume_digits(&mut self, radix: u32, mut prev_char: char) -> Res<char> {
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

    #[inline]
    fn check_trailing_underscore(&self, prev_char: char) -> Res<()> {
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
    fn check_repeating_underscore(&self, char_1: char, char_2: char) -> Res<()> {
        if char_1 == '_' && char_2 == '_' {
            Err(RawError {
                msg: "double numeric seperator".to_string(),
                idx: self.current_start,
            })
        } else {
            Ok(())
        }
    }
    #[inline]
    fn is_id_continue(c: char) -> bool {
        c == '$'
            || c == '_'
            || (c >= 'A' && c <= 'Z')
            || (c >= 'a' && c <= 'z')
            || c == '\\'
            || (c >= '0' && c <= '9')
            || is_id_continue(c)
    }
    #[inline]
    fn is_id_start(c: char) -> bool {
        c == '$'
            || c == '_'
            || (c >= 'A' && c <= 'Z')
            || (c >= 'a' && c <= 'z')
            || c == '\\'
            || is_id_start(c)
    }
    #[inline]
    fn look_ahead_byte_matches(&self, c: char) -> bool {
        self.stream.look_ahead_byte_matches(c as u8)
    }
    #[inline]
    fn look_ahead_matches(&self, s: &str) -> bool {
        self.stream.look_ahead_matches(s.as_bytes())
    }
    #[inline]
    fn gen_punct(&self, p: Punct) -> Res<RawItem> {
        self.gen_token(RawToken::Punct(p))
    }
    #[inline]
    fn gen_number(&self, n: NumberKind) -> Res<RawItem> {
        // result = Number(n)
        self.gen_token(RawToken::Number(n))
    }
    #[inline]
    fn gen_template(
        &self,
        kind: TemplateKind,
        new_line_count: usize,
        last_len: usize,
    ) -> Res<RawItem> {
        self.gen_token(RawToken::Template {
            kind,
            new_line_count,
            last_len,
        })
    }
    #[inline]
    fn gen_regex(&self, start_len: usize, body_idx: usize) -> Res<RawItem> {
        Ok(RawItem {
            start: self.current_start.saturating_sub(start_len),
            end: self.stream.idx,
            ty: RawToken::RegEx(body_idx),
        })
    }
    #[inline]
    fn gen_comment(
        &self,
        kind: CommentKind,
        new_line_count: usize,
        last_len: usize,
    ) -> Res<RawItem> {
        self.gen_token(RawToken::Comment {
            kind,
            new_line_count,
            last_len,
        })
    }
    #[inline]
    fn gen_token(&self, ty: RawToken) -> Res<RawItem> {
        Ok(RawItem {
            start: self.current_start,
            end: self.stream.idx,
            ty,
        })
    }
    #[inline]
    pub fn skip_whitespace(&mut self) -> (usize, usize) {
        let mut ct = 0usize;
        let mut leading_whitespace = 0usize;
        while self.stream.at_whitespace() {
            if self.at_new_line() {
                ct += 1;
                leading_whitespace = 0;
            }
            leading_whitespace = leading_whitespace.saturating_add(1);
            self.stream.skip(1);
        }
        (ct, leading_whitespace)
    }
    #[inline]
    fn at_new_line(&mut self) -> bool {
        self.stream.at_new_line()
    }
    /// Carrage Return is always a special case so that
    /// must be handled inline
    #[inline]
    fn is_new_line_not_cr(c: char) -> bool {
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
            "^=", "%=", "<=", ">=", "=>", "**",
        ];
        for p in PUNCTS {
            let mut t = Tokenizer::new(p);
            let item = t.next().unwrap();
            assert!(item.ty.is_punct());
            assert!(t.stream.at_end());
        }
    }
    #[test]
    fn tokenizer_hashbang() {
        let b = "#!/usr/bin/env node";
        let mut t = Tokenizer::new(b);
        let item = t.next().unwrap();
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
        let item = t.next().unwrap();
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
        let item = t.next().unwrap();
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
        let item = t.next().unwrap();
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
        let item = t.next().unwrap();
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
            let mut t = Tokenizer::new(s);
            let item = t.next().unwrap();
            match &item.ty {
                RawToken::String {
                    kind,
                    new_line_count: _,
                    last_len: _,
                } => {
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
        static IDENTS: &[&str] = &[
            r#"$"#,
            r#"_"#,
            r#"\u0078"#,
            r#"x$"#,
            r#"x_"#,
            r#"x\u0030"#,
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
        ];
        for i in IDENTS {
            let mut t = Tokenizer::new(i);
            let item = t.next().unwrap();
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
        ];
        for n in NUMBERS {
            println!("n: {}", n);
            let mut t = Tokenizer::new(n);
            let item = t.next().unwrap();
            dbg!(&n[item.start..item.end]);
            assert!(match item.ty {
                RawToken::Number(_) => true,
                _ => false,
            });
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
        ];
        for r in REGEX {
            let mut t = Tokenizer::new(r);
            let next = t.next().unwrap();
            let item = t.next_regex(next.end - next.start).unwrap();
            assert!(match item.ty {
                RawToken::RegEx(_) => true,
                _ => false,
            });
            assert!(t.stream.at_end());
        }
    }

    #[test]
    fn tokenizer_template() {
        let subbed = "`things and stuff times ${} and animals and minerals`";
        println!("subbed: {}", subbed);
        let mut t = Tokenizer::new(subbed);
        let start = t.next().unwrap();
        check_temp(&start.ty, TemplateKind::Head);
        let end = t.next().unwrap();
        check_temp(&end.ty, TemplateKind::Tail);
        assert!(t.stream.at_end());
        let no_sub = "`things and stuff`";
        println!("no_sub {}", no_sub);
        t = Tokenizer::new(no_sub);
        let one = t.next().unwrap();
        check_temp(&one.ty, TemplateKind::NoSub);
        assert!(t.stream.at_end());
        let escaped_sub = r#"`\0\n\x0A\u000A\u{A}${}`"#;
        println!("escaped_sub: {}", escaped_sub);
        t = Tokenizer::new(escaped_sub);
        let start = t.next().unwrap();
        check_temp(&start.ty, TemplateKind::Head);
        let end = t.next().unwrap();
        check_temp(&end.ty, TemplateKind::Tail);
        assert!(t.stream.at_end());
        let escaped_no_sub = r#"`a\${b`"#;
        println!("escaped_no_sub: {}", escaped_no_sub);
        t = Tokenizer::new(escaped_no_sub);
        let one = t.next().unwrap();
        check_temp(&one.ty, TemplateKind::NoSub);
        assert!(t.stream.at_end());
        let double_sub =
            "`things and stuff times ${} and animals and minerals ${} and places and people`";
        println!("double_sub: {}", double_sub);
        t = Tokenizer::new(double_sub);
        let start = t.next().unwrap();
        check_temp(&start.ty, TemplateKind::Head);
        let mid = t.next().unwrap();
        check_temp(&mid.ty, TemplateKind::Body);
        let end = t.next().unwrap();
        check_temp(&end.ty, TemplateKind::Tail);
        assert!(t.stream.at_end());
    }

    fn check_temp(temp: &RawToken, expected_kind: TemplateKind) {
        match temp {
            RawToken::Template {
                kind,
                new_line_count: _,
                last_len: _,
            } => {
                assert_eq!(kind, &expected_kind);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn tokenizer_bools() {
        for b in &["true", "false"] {
            let mut t = Tokenizer::new(b);
            let item = t.next().unwrap();
            assert!(match item.ty {
                RawToken::Boolean(_) => true,
                _ => false,
            });
            assert!(t.stream.at_end());
        }
    }

    #[test]
    fn tokenizer_null() {
        let mut t = Tokenizer::new("null");
        let item = t.next().unwrap();
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
            let item = t.next().unwrap();
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
        ];
        for c in COMMENTS {
            let mut t = Tokenizer::new(c);
            let item = t.next().unwrap();
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
            "<!--normally terminated-->"
        ];
        static FAIL_COMMENTS: &[&str] = &[
            "<!--this will fail",
            "hello world",
        ];
        for c in SUCCESS_COMMENTS {
            let mut t = Tokenizer::new(c);
            let item = t.next().unwrap();
            assert!(item.ty.is_comment());
        }
        for c in FAIL_COMMENTS {
            let mut t = Tokenizer::new(c);
            match t.next() {
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
        let _ = t.next();
        assert_eq!(t.skip_whitespace().0, 1); //\n
        let _ = t.next();
        assert_eq!(t.skip_whitespace().0, 0);
        let _ = t.next();
        assert_eq!(t.skip_whitespace().0, 1); //\r
        let _ = t.next();
        assert_eq!(t.skip_whitespace().0, 0);
        let _ = t.next();
        assert_eq!(t.skip_whitespace().0, 1); //\r\n
        let _ = t.next();
        assert_eq!(t.skip_whitespace().0, 0);
        let _ = t.next();
        assert_eq!(t.skip_whitespace().0, 1); // line seperator
        let _ = t.next();
        assert_eq!(t.skip_whitespace().0, 0);
        let _ = t.next();
        assert_eq!(t.skip_whitespace().0, 1); // paragraph separator
    }
}
