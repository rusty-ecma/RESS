
use crate::{Span, tokenizer::{self, Tokenizer}};
use crate::tokenizer::RawToken;
use crate::Item;
use crate::tokens::{self, prelude::*};
use crate::error::{Error, RawError};

type Res<T> = Result<T, Error>;
type Ret<'a> = Option<Res<Item<Token<&'a str>>>>;

pub struct ManualScanner<'a> {
    pub stream: Tokenizer<'a>,
    pub eof: bool,
    pub pending_new_line: bool,
    original: &'a str,
    errored: bool,
    pub new_line_count: usize,
    line_cursor: usize,
    at_first_on_line: bool,
}

impl<'b> ManualScanner<'b> {
    pub fn new(text: &'b str) -> Self {
        let mut stream = Tokenizer::new(text);
        let (new_line_count, line_cursor) = stream.skip_whitespace();
        Self {
            stream,
            eof: false,
            pending_new_line: false,
            original: text,
            errored: false,
            new_line_count,
            line_cursor: usize::max(line_cursor, 1),
            at_first_on_line: true,
        }
    }

    /// Skip any upcoming comments to get the
    /// next valid js token
    pub fn skip_comments(&mut self) -> Res<()> {
        debug!(target: "ress", "skipping comments");
        let mut state = self.get_state();
        while let Some(item) = self.next_token() {
            if let Token::Comment(_) = item?.token {
                state = self.get_state();
            } else {
                break;
            }
        }
        self.set_state(state);
        Ok(())
    }
    /// Get a copy of the scanner's current state
    pub fn get_state(&self) -> ScannerState {
        ScannerState {
            cursor: self.stream.stream.idx,
            new_line_count: self.new_line_count,
            line_cursor: self.line_cursor,
            at_first_on_line: self.at_first_on_line,
        }
    }
    /// Set the scanner's current state to the state provided
    #[inline]
    pub fn set_state(&mut self, state: ScannerState) {
        self.stream.stream.idx = state.cursor;
        self.new_line_count = state.new_line_count;
        self.line_cursor = state.line_cursor;
        self.at_first_on_line = state.at_first_on_line;
    }

    pub fn next_token(&mut self) -> Ret<'b> {
        if self.eof {
            debug!("end of iterator, returning None");
            return None;
        };
        let (_, prev_lines, prev_line_cursor) = self.capture_cursors();
        let next = match self.stream.next(self.at_first_on_line) {
            Ok(n) => n,
            Err(e) => {
                self.errored = true;
                return Some(self.error(e));
            }
        };

        let mut len = next.end - next.start;
        let ret = {
            let mut new_lines = 0;
            let s = &self.original[next.start..next.end];
            let token = match next.ty {
                RawToken::Boolean(b) => Token::Boolean(b.into()),
                RawToken::Comment {
                    kind,
                    new_line_count,
                    last_len,
                    end_index,
                } => {
                    len = last_len;
                    new_lines = new_line_count;
                    match kind {
                        tokens::CommentKind::Multi => {
                            let (tail_content, tail_start) =
                                if let Some(tail_start) = s[end_index..].find("-->") {
                                    let actual_start = end_index + tail_start;
                                    (Some(&s[actual_start + 3..]), actual_start)
                                } else {
                                    (None, s.len())
                                };
                            let content = s[..tail_start]
                                .trim_start_matches("/*")
                                .trim_end_matches("*/");
                            Token::Comment(Comment {
                                kind: tokens::CommentKind::Multi,
                                content,
                                tail_content,
                            })
                        }
                        tokens::CommentKind::Single => {
                            Token::Comment(Comment::new_single_line(s.trim_start_matches("//")))
                        }
                        tokens::CommentKind::Html => {
                            let start_idx = if s.starts_with("<!--") { 4 } else { 0 };
                            let (content, tail) = if let Some(idx) = s.rfind("-->") {
                                let actual_end = idx.saturating_add(3);
                                if actual_end < next.end {
                                    let tail = &s[actual_end..];
                                    let tail = if tail == "" { None } else { Some(tail) };
                                    (&s[start_idx..idx], tail)
                                } else {
                                    (&s[start_idx..], None)
                                }
                            } else {
                                (&s[start_idx..], None)
                            };
                            if start_idx == 0 && !self.at_first_on_line(next.start) {
                                self.errored = true;
                                return Some(Err(Error {
                                    line: self.new_line_count,
                                    column: self.line_cursor,
                                    msg: "--> comments must either be a part of a full HTML comment or the first item on a new line".to_string()
                                }));
                            }
                            Token::Comment(Comment::new_html(content, tail))
                        }
                        tokens::CommentKind::Hashbang => {
                            Token::Comment(Comment::new_hashbang(&s[2..]))
                        }
                    }
                }
                RawToken::EoF => {
                    self.eof = true;
                    return Some(Ok(Item::new_(
                        Token::EoF,
                        self.original.len(),
                        self.original.len(),
                        prev_lines.saturating_add(1),
                        prev_line_cursor,
                        self.new_line_count.saturating_add(1),
                        self.line_cursor,
                    )));
                }
                RawToken::Ident => Token::Ident(Ident::from(s)),
                RawToken::Keyword(k) => Token::Keyword(k.with_str(s)),
                RawToken::Null => Token::Null,
                RawToken::Number(_) => Token::Number(Number::from(s)),
                RawToken::Punct(p) => Token::Punct(p),
                RawToken::RegEx(_) => unreachable!("Regex from next"),
                RawToken::String {
                    kind,
                    new_line_count,
                    last_len,
                    found_octal_escape,
                } => {
                    len = last_len;
                    new_lines = new_line_count;
                    let s = &s[1..s.len() - 1];
                    match kind {
                        tokenizer::StringKind::Double => {
                            Token::String(StringLit::double(s, found_octal_escape))
                        }
                        tokenizer::StringKind::Single => {
                            Token::String(StringLit::single(s, found_octal_escape))
                        }
                    }
                }
                RawToken::Template {
                    kind,
                    new_line_count,
                    last_len,
                    has_octal_escape,
                    found_invalid_unicode_escape,
                    found_invalid_hex_escape,
                } => {
                    len = last_len;
                    new_lines = new_line_count;
                    match kind {
                        tokenizer::TemplateKind::Head => {
                            let s = &s[1..s.len() - 2];
                            Token::Template(Template::template_head(
                                s,
                                has_octal_escape,
                                found_invalid_unicode_escape,
                                found_invalid_hex_escape,
                            ))
                        }
                        tokenizer::TemplateKind::Body => {
                            let s = &s[1..s.len() - 2];
                            Token::Template(Template::template_middle(
                                s,
                                has_octal_escape,
                                found_invalid_unicode_escape,
                                found_invalid_hex_escape,
                            ))
                        }
                        tokenizer::TemplateKind::Tail => {
                            let s = &s[1..s.len() - 1];
                            Token::Template(Template::template_tail(
                                s,
                                has_octal_escape,
                                found_invalid_unicode_escape,
                                found_invalid_hex_escape,
                            ))
                        }
                        tokenizer::TemplateKind::NoSub => {
                            let s = &s[1..s.len() - 1];
                            Token::Template(Template::no_sub_template(
                                s,
                                has_octal_escape,
                                found_invalid_unicode_escape,
                                found_invalid_hex_escape,
                            ))
                        }
                    }
                }
            };
            self.at_first_on_line = self.at_first_on_line && token.is_multi_line_comment();
            self.bump_line_cursors(new_lines, len);
            Item::new_(
                token,
                next.start,
                next.end,
                prev_lines.saturating_add(1),
                prev_line_cursor,
                self.new_line_count.saturating_add(1),
                self.line_cursor,
            )
        };
        let (new_line_count, leading_whitespace) = self.stream.skip_whitespace();
        self.bump_line_cursors(new_line_count, leading_whitespace);
        self.pending_new_line = new_line_count > 0;
        Some(Ok(ret))
    }
    /// Get the next token as a regular expression. The previous token
    /// should have been `/` or `/=`, 
    pub fn next_regex(&mut self, prev_len: usize) -> Option<Res<Item<Token<&'b str>>>> {
        let (_, prev_lines, prev_line_cursor) = self.capture_cursors();
        let next = match dbg!(self.stream.next_regex(prev_len)) {
            Ok(n) => n,
            Err(e) => {
                self.errored = true;
                return Some(self.error(e));
            }
        };
        let ret = match next.ty {
            RawToken::RegEx(body_end) => {
                self.line_cursor = self.line_cursor.saturating_sub(prev_len);
                self.line_cursor = self.line_cursor.saturating_add(next.end - next.start);
                let flags = if next.end > body_end {
                    Some(&self.original[body_end..next.end])
                } else {
                    None
                };
                Item::new_(
                    Token::RegEx(RegEx {
                        body: &self.original[next.start + 1..body_end - 1],
                        flags,
                    }),
                    next.start,
                    next.end,
                    prev_lines + 1,
                    prev_line_cursor.saturating_sub(prev_len),
                    prev_lines + 1,
                    self.line_cursor,
                )
            }
            _ => {
                todo!();
                // Some(self.error(todo!()))
            },
        };
        let (new_line_count, leading_whitespace) = self.stream.skip_whitespace();
        self.bump_line_cursors(new_line_count, leading_whitespace);
        self.pending_new_line = new_line_count > 0;
        Some(Ok(ret))
    }

    fn capture_cursors(&self) -> (usize, usize, usize) {
        (self.stream.stream.idx,
        self.new_line_count,
        self.line_cursor)
    }
    
    /// Get a string for any given span
    pub fn string_for(&self, span: &Span) -> Option<String> {
        Some(self.str_for(span)?.to_string())
    }
    /// Get a &str for any given span
    pub fn str_for(&self, span: &Span) -> Option<&'b str> {
        if self.original.len() < span.start || self.original.len() < span.end {
            None
        } else {
            Some(&self.original[span.start..span.end])
        }
    }
    /// Get the line/column pair for any given byte index
    pub fn position_for(&self, idx: usize) -> (usize, usize) {
        let mut line_ct = 1;
        // This is the byte position, not the character
        // position to account for multi byte chars
        let mut byte_position = 0;
        // loop over the characters
        for (i, c) in self.original.chars().enumerate() {
            if i >= idx {
                return (line_ct, byte_position);
            }
            match c {
                '\r' => {
                    // look ahead 1 char to see if it is a newline pair
                    // if so, don't include it, it will get included in the next
                    // iteration
                    if let Some(next) = self.original.get(byte_position..byte_position + 2) {
                        if next != "\r\n" {
                            line_ct += 1;
                            byte_position = 0;
                        }
                    }
                }
                '\n' | '\u{2028}' | '\u{2029}' => {
                    line_ct += 1;
                    byte_position = 0;
                }
                _ => byte_position += c.len_utf8(),
            };
        }
        (line_ct, byte_position)
    }
    #[inline]
    /// Helper to handle new lines
    fn bump_line_cursors(&mut self, ct: usize, len: usize) {
        if ct != 0 {
            self.line_cursor = len;
            self.new_line_count += ct;
            self.at_first_on_line = true;
        } else {
            self.line_cursor += len;
        }
    }
    #[inline]
    fn at_first_on_line(&self, token_start: usize) -> bool {
        trace!("at_first_on_line");
        if self.line_cursor <= 1 {
            return true;
        }
        let start = token_start.saturating_sub(self.line_cursor - 1);
        let prefix = &self.original[start..token_start];
        trace!("prefix: {:?}", prefix);
        prefix.chars().all(|c| c.is_whitespace())
    }
    /// Helper to handle the error cases
    fn error<T>(&self, raw_error: RawError) -> Res<T> {
        let RawError { idx, msg } = raw_error;
        let (line, column) = self.position_for(idx);
        Err(Error { line, column, msg })
    }
}

#[derive(Clone)]
/// All of the important state
/// for the scanner, used to
/// cache and reset a `Scanner`
pub struct ScannerState {
    pub cursor: usize,
    pub new_line_count: usize,
    pub line_cursor: usize,
    pub at_first_on_line: bool,
}
