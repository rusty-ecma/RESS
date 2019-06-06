use crate::tokens::{CommentKind, Keyword, NumberKind, Punct};
use unic_ucd_ident::{is_id_continue, is_id_start};
use crate::{is_line_term, OpenCurlyKind};
mod buffer;
mod tokens;
pub(super) use self::tokens::{RawToken, StringKind, TemplateKind};

pub struct RawItem {
    pub ty: tokens::RawToken,
    pub start: usize,
    pub end: usize,
}

pub struct Tokenizer<'a> {
    pub(super) stream: buffer::JSBuffer<'a>,
    current_start: usize,
    pub(super) curly_stack: Vec<OpenCurlyKind>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(stream: &'a str) -> Self {
        Tokenizer {
            current_start: 0,
            stream: stream.into(),
            curly_stack: Vec::new(),
        }
    }

    pub fn next_(&mut self) -> RawItem {
        self.current_start = self.stream.idx;
        let next_char = match self.stream.next_char() {
            Some(ch) => ch,
            None => {
                return RawItem {
                    start: self.stream.idx,
                    end: self.stream.idx,
                    ty: RawToken::EoF,
                }
            }
        };
        if is_id_start(next_char) || next_char == '$' || next_char == '_' || next_char == '\\' {
            return self.ident(next_char);
        }
        if next_char == '"' || next_char == '\'' {
            return self.string(next_char);
        }
        if next_char == '(' || next_char == ')' || next_char == ';' {
            return self.punct(next_char);
        }
        if next_char == '.' || next_char.is_digit(10) {
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

    pub fn next_regex(&mut self) -> RawItem {
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
                    if self.look_ahead_matches("u{") {
                        self.stream.skip(2);
                        self.escaped_with_code_point();
                    } else if self.look_ahead_matches("u") {
                        self.stream.skip(1);
                        let start = self
                            .stream
                            .next_char()
                            .expect("unexpected end of file when parsing regex");
                        self.escaped_with_hex4(start);
                    }
                } else if !is_id_continue(c) {
                    let _ = self.stream.prev_char();
                    return self.gen_regex(body_idx);
                }
            } else if c == '\\' {
                    if self.stream.at_new_line() {
                        panic!("new line in regex literal at {}", self.stream.idx);
                } else if self.look_ahead_matches("[") 
                    || self.look_ahead_matches("/") {
                        self.stream.skip(1);
                    }
                } else if is_line_term(c) {
                    panic!("new line in regex literal at {}", self.stream.idx);
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
            return self.gen_regex(body_idx);
        }
        panic!("unterminated regex at {}", self.current_start);
    }

    fn ident(&mut self, start: char) -> RawItem {
        if start == '\\' {
            // TODO validate escaped ident start
            self.escaped_ident_part();
        }
        while let Some(c) = self.stream.next_char() {
            if c == '\\' {
                self.escaped_ident_part();
            }
            if !is_id_continue(c) && c != '$' && c != '\u{200C}' && c != '\u{200D}' {
                // if we have moved past the last valid identifier, go back 1
                let _ = self.stream.prev_char();
                break;
            }
        }
        if let Some(k) = self.at_keyword() {
            self.gen_token(RawToken::Keyword(k))
        } else if let Some(b) = self.at_bool() {
            self.gen_token(RawToken::Boolean(b))
        } else if &self.stream.buffer[self.current_start..self.stream.idx] == b"null" {
            self.gen_token(RawToken::Null)
        } else {
            self.gen_token(RawToken::Ident)
        }
    }

    fn at_keyword(&self) -> Option<Keyword> {
        match &self.stream.buffer[self.current_start..self.stream.idx] {
            b"await" => Some(Keyword::Await),
            b"break" => Some(Keyword::Break),
            b"case" => Some(Keyword::Case),
            b"catch" => Some(Keyword::Catch),
            b"class" => Some(Keyword::Class),
            b"const" => Some(Keyword::Const),
            b"continue" => Some(Keyword::Continue),
            b"debugger" => Some(Keyword::Debugger),
            b"default" => Some(Keyword::Default),
            b"delete" => Some(Keyword::Delete),
            b"do" => Some(Keyword::Do),
            b"else" => Some(Keyword::Else),
            b"finally" => Some(Keyword::Finally),
            b"for" => Some(Keyword::For),
            b"function" => Some(Keyword::Function),
            b"if" => Some(Keyword::If),
            b"instanceof" => Some(Keyword::InstanceOf),
            b"in" => Some(Keyword::In),
            b"new" => Some(Keyword::New),
            b"return" => Some(Keyword::Return),
            b"switch" => Some(Keyword::Switch),
            b"this" => Some(Keyword::This),
            b"throw" => Some(Keyword::Throw),
            b"try" => Some(Keyword::Try),
            b"typeof" => Some(Keyword::TypeOf),
            b"var" => Some(Keyword::Var),
            b"void" => Some(Keyword::Void),
            b"while" => Some(Keyword::While),
            b"with" => Some(Keyword::With),
            b"export" => Some(Keyword::Export),
            b"import" => Some(Keyword::Import),
            b"super" => Some(Keyword::Super),
            b"enum" => Some(Keyword::Enum),
            b"implements" => Some(Keyword::Implements),
            b"interface" => Some(Keyword::Interface),
            b"package" => Some(Keyword::Package),
            b"private" => Some(Keyword::Private),
            b"protected" => Some(Keyword::Protected),
            b"public" => Some(Keyword::Public),
            b"static" => Some(Keyword::Static),
            b"yield" => Some(Keyword::Yield),
            b"let" => Some(Keyword::Let),
            _ => None,
        }
    }

    fn at_bool(&self) -> Option<bool> {
        match &self.stream.buffer[self.current_start..self.stream.idx] {
            b"true" => Some(true),
            b"false" => Some(false),
            _ => None,
        }
    }
    /// picking up after the \
    fn escaped_ident_part(&mut self) {
        if let Some('u') = self.stream.next_char() {
            if let Some(c) = self.stream.next_char() {
                if c == '{' {
                    self.escaped_with_code_point();
                } else {
                    self.escaped_with_hex4(c)
                }
            }
        } else {
            panic!(
                "invalid unicode escape sequence starting at {}",
                self.current_start
            );
        }
    }

    fn escaped_with_code_point(&mut self) {
        let mut code: u32 = 0;
        let mut last_char: char = '{';
        while let Some(c) = self.stream.next_char() {
            last_char = c;
            if c == '}' {
                break;
            }
            assert!(c.is_digit(16));
            code += u32::from_str_radix(c.encode_utf8(&mut [0; 4]), 16)
                .expect("invalid hex digit in escaped unicode codepoint");
        }
        assert!(code < 0x10FFF);
        assert!(last_char == '}');
    }

    fn escaped_with_hex4(&mut self, start: char) {
        assert!(start.is_digit(16));
        for _ in 0..3 {
            assert!(self.stream.next_char().unwrap().is_digit(16))
        }
    }

    fn string(&mut self, quote: char) -> RawItem {
        let mut escaped = false;
        loop {
            if self.look_ahead_matches(r#"\"#) {
                if escaped {
                    escaped = false;
                } else {
                    escaped = true;
                }
                self.stream.skip(1)
            } else if self.look_ahead_matches("\r\n") {
                if !escaped {
                    panic!("unescaped new line in string literal")
                } else {
                    self.stream.skip(2);
                    escaped = false;
                }
            } else if (self.look_ahead_matches("\n")
                || self.look_ahead_matches("\r")
                || self.look_ahead_matches("\u{2028}")
                || self.look_ahead_matches("\u{2029}"))
                && !escaped
            {
                panic!("unescaped new line in string literal");
            } else if self.stream.look_ahead_matches(&[quote as u8]) {
                self.stream.skip(1);
                if !escaped {
                    break;
                }
                escaped = false;
            } else {
                self.stream.skip(1);
                escaped = false;
            }
        }
        let inner = if quote == '"' {
            StringKind::Double
        } else {
            StringKind::Single
        };
        self.gen_token(RawToken::String(inner))
    }
    fn punct(&mut self, c: char) -> RawItem {
        match c {
            '(' => self.gen_punct(Punct::OpenParen),
            '{' => {
                self.curly_stack.push(OpenCurlyKind::Block);
                self.gen_punct(Punct::OpenBrace)
            }
            ')' => self.gen_punct(Punct::CloseParen),
            '}' => {
                let _ = self.curly_stack.pop();
                self.gen_punct(Punct::CloseBrace)
            }
            ';' => self.gen_punct(Punct::SemiColon),
            ',' => self.gen_punct(Punct::Comma),
            '[' => self.gen_punct(Punct::OpenBracket),
            ']' => self.gen_punct(Punct::CloseBracket),
            ':' => self.gen_punct(Punct::Colon),
            '?' => self.gen_punct(Punct::QuestionMark),
            '#' => self.gen_punct(Punct::Hash),
            '~' => self.gen_punct(Punct::Tilde),
            '.' => {
                // ...
                if self.look_ahead_matches("..") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::Ellipsis)
                } else {
                    self.gen_punct(Punct::Period)
                }
            }
            '>' => {
                if self.look_ahead_matches(">>=") {
                    self.stream.skip(3);
                    self.gen_punct(Punct::TripleGreaterThanEqual)
                } else if self.look_ahead_matches(">>") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::TripleGreaterThan)
                } else if self.look_ahead_matches(">=") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::DoubleGreaterThanEqual)
                } else if self.look_ahead_matches(">") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::DoubleGreaterThan)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::GreaterThanEqual)
                } else {
                    self.gen_punct(Punct::GreaterThan)
                }
            }
            '<' => {
                if self.look_ahead_matches("<=") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::DoubleLessThanEqual)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::LessThanEqual)
                } else if self.look_ahead_matches("<") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::DoubleLessThan)
                } else if self.look_ahead_matches("!--") {
                    self.stream.skip(3);
                    self.html_comment()
                } else {
                    self.gen_punct(Punct::LessThan)
                }
            }
            '=' => {
                if self.look_ahead_matches("==") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::TripleEqual)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::DoubleEqual)
                } else if self.look_ahead_matches(">") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::EqualGreaterThan)
                } else {
                    self.gen_punct(Punct::Equal)
                }
            }
            '!' => {
                if self.look_ahead_matches("==") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::BangDoubleEqual)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::BangEqual)
                } else {
                    self.gen_punct(Punct::Bang)
                }
            }
            '*' => {
                if self.look_ahead_matches("*=") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::DoubleAsteriskEqual)
                } else if self.look_ahead_matches("*") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::DoubleAsterisk)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::AsteriskEqual)
                } else {
                    self.gen_punct(Punct::Asterisk)
                }
            }
            '&' => {
                if self.look_ahead_matches("&") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::DoubleAmpersand)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::AmpersandEqual)
                } else {
                    self.gen_punct(Punct::Ampersand)
                }
            }
            '|' => {
                if self.look_ahead_matches("|") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::DoublePipe)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::PipeEqual)
                } else {
                    self.gen_punct(Punct::Pipe)
                }
            }
            '+' => {
                if self.look_ahead_matches("+") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::DoublePlus)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::PlusEqual)
                } else {
                    self.gen_punct(Punct::Plus)
                }
            }
            '-' => {
                if self.look_ahead_matches("-") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::DoubleDash)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::DashEqual)
                } else {
                    self.gen_punct(Punct::Dash)
                }
            }
            '/' => {
                if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::ForwardSlashEqual)
                } else if self.look_ahead_matches("*") {
                    self.multi_comment()
                } else if self.look_ahead_matches("/") {
                    self.single_comment()
                } else {
                    self.gen_punct(Punct::ForwardSlash)
                }
            }
            '%' => {
                if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::PercentEqual)
                } else {
                    self.gen_punct(Punct::Percent)
                }
            }
            '^' => {
                if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::CaretEqual)
                } else {
                    self.gen_punct(Punct::Caret)
                }
            }
            _ => unreachable!("unknown punct {:?} {}", c, self.current_start),
        }
    }
    fn number(&mut self, start: char) -> RawItem {
        if let Some(next) = self.stream.next_char() {
            if start.is_digit(10) && !next.is_digit(10) {
                let _ = self.stream.prev_char();
                self.dec_number(false)
            } else if start == '.' {
                if !next.is_digit(10) {
                    let _ = self.stream.prev_char();
                    return self.punct(start);
                }
                self.dec_number(true)
            } else if start == '0' {
                if next == 'o' || next == 'O' {
                    self.oct_number()
                } else if next == 'x' || next == 'X' {
                    self.hex_number()
                } else if next == 'b' || next == 'B' {
                    self.bin_number()
                } else {
                    self.dec_number(next == '.')
                }
            } else {
                self.dec_number(next == '.')
            }
        } else {
            let _ = self.stream.prev_char();
            if start == '.' {
                self.gen_punct(Punct::Period)
            } else {
                self.dec_number(false)
            }
        }
    }
    fn template(&mut self, start: char) -> RawItem {
        // if self.look_ahead_matches("${") {
        //     self.stream.skip(2);
        //     self.curly_stack.push(OpenCurlyKind::Template);
        //     return self.gen_template(Template::Head);
        // } else if self.look_ahead_matches("\\${") {
        //     self.stream.skip(2);
        // } else if self.look_ahead_matches("\\`") {
        //     self.stream.skip(1);
        // } else if self.look_ahead_matches("`") {
        //     self.stream.skip(1);
        //     if start == '`' {
        //         return self.gen_template(Template::NoSub);
        //     } else {
        //         return self.gen_template(Template::Tail);
        //     }
        // }
        while let Some(c) = self.stream.next_char() {
            println!("template char: {}", c);
            if c == '\\' {
                if self.look_ahead_matches("${") {
                    self.stream.skip(2);
                } else if self.look_ahead_matches("`") {
                    self.stream.skip(1);
                } else if self.look_ahead_matches("0") {
                    if let Some(_zero) = self.stream.next_char() {
                        if self.stream.at_decimal() {
                            panic!(
                                "Template contains octal literal at {}",
                                self.stream.idx.saturating_sub(1)
                            );
                        } else {
                            let _ = self.stream.prev_char();
                        }
                    }
                } else if self.stream.at_octal() {
                    panic!(
                        "Template contains octal literal starting at {}",
                        self.stream.idx
                    );
                }
            } else if c == '$' {
                if self.look_ahead_matches("{") {
                    self.stream.skip(1);
                    self.curly_stack.push(OpenCurlyKind::Template);
                    if start == '`' {
                        return self.gen_template(TemplateKind::Head);
                    } else {
                        return self.gen_template(TemplateKind::Body);
                    }
                }
            } else if c == '`' {
                if start == '`' {
                    return self.gen_template(TemplateKind::NoSub);
                } else {
                    return self.gen_template(TemplateKind::Tail);
                }
            }
        }
        panic!("unterminated template");
    }
    fn single_comment(&mut self) -> RawItem {
        while !self.at_new_line() {
            if self.stream.next_char().is_none() {
                break;
            }
        }
        self.gen_comment(CommentKind::Single)
    }
    fn multi_comment(&mut self) -> RawItem {
        if self.look_ahead_matches("*/") {
            self.stream.skip(2);
            return self.gen_comment(CommentKind::Multi);
        }
        while let Some(_) = self.stream.next_char() {
            if self.look_ahead_matches("*/") {
                self.stream.skip(2);
                return self.gen_comment(CommentKind::Multi);
            }
        }
        panic!(
            "unterminated multi-line comment starting at {}",
            self.current_start
        );
    }
    fn html_comment(&mut self) -> RawItem {
        let mut found_end = if self.look_ahead_matches("-->") {
            self.stream.skip(3);
            true
        } else {
            false
        };
        while let Some(_) = self.stream.next_char() {
            if self.look_ahead_matches("-->") {
                self.stream.skip(3);
                found_end = true;
            }
        }
        while !self.at_new_line() && !self.stream.at_end() {
            self.stream.skip(1)
        }
        if found_end {
            return self.gen_token(RawToken::Comment(CommentKind::Html));
        }
        panic!(
            "unterminated html comment starting at {}",
            self.current_start
        );
    }
    fn hex_number(&mut self) -> RawItem {
        if let Some(c) = self.stream.next_char() {
            if !c.is_digit(16) {
                panic!("empty hex literal")
            }
        } else {
            panic!("empty hex literal")
        }
        while let Some(c) = self.stream.next_char() {
            if !c.is_digit(16) {
                let _ = self.stream.prev_char();
                break;
            }
        }
        self.gen_number(NumberKind::Hex)
    }
    fn oct_number(&mut self) -> RawItem {
        if let Some(c) = self.stream.next_char() {
            if !c.is_digit(8) {
                panic!("empty octal literal");
            }
        } else {
            panic!("empty octal literal");
        }
        while let Some(c) = self.stream.next_char() {
            if !c.is_digit(8) {
                let _ = self.stream.prev_char();
                break;
            }
        }
        self.gen_number(NumberKind::Oct)
    }
    fn bin_number(&mut self) -> RawItem {
        if let Some(c) = self.stream.next_char() {
            if !c.is_digit(2) {
                panic!("empty bin literal");
            }
        } else {
            panic!("empty bin literal");
        }
        while let Some(c) = self.stream.next_char() {
            if !c.is_digit(2) {
                let _ = self.stream.prev_char();
                break;
            }
        }
        self.gen_number(NumberKind::Bin)
    }
    fn dec_number(&mut self, seen_point: bool) -> RawItem {
        let mut maybe_e: Option<char> = None;

        if seen_point {
            while let Some(c) = self.stream.next_char() {
                if !c.is_digit(10) && c != '.' {
                    maybe_e = Some(c);
                    break;
                }
            }
        } else {
            while let Some(c) = self.stream.next_char() {
                if !c.is_digit(10) {
                    maybe_e = Some(c);
                    break;
                }
            }
        }
        if let Some(maybe_e) = maybe_e {
            if maybe_e != 'e' && maybe_e != 'E' {
                let _ = self.stream.prev_char();
            } else {
                if let Some(c) = self.stream.next_char() {
                    if c != '+' && c != '-' && !c.is_digit(10) {
                        panic!("Invalid decimal starting at {}", self.current_start);
                    }
                }
                while let Some(c) = self.stream.next_char() {
                    if !c.is_digit(10) {
                        let _ = self.stream.prev_char();
                        break;
                    }
                }
            }
        }
        self.gen_number(NumberKind::Dec)
    }
    #[inline]
    fn look_ahead_matches(&self, s: &str) -> bool {
        self.stream.look_ahead_matches(s.as_bytes())
    }
    #[inline]
    fn gen_punct(&self, p: Punct) -> RawItem {
        self.gen_token(RawToken::Punct(p))
    }
    #[inline]
    fn gen_number(&self, n: NumberKind) -> RawItem {
        self.gen_token(RawToken::Numeric(n))
    }
    #[inline]
    fn gen_template(&self, t: TemplateKind) -> RawItem {
        self.gen_token(RawToken::Template(t))
    }
    #[inline]
    fn gen_regex(&self, body_idx: usize) -> RawItem {
        RawItem {
            start: self.current_start.saturating_sub(1),
            end: self.stream.idx,
            ty: RawToken::RegEx(body_idx),
        }
    }
    #[inline]
    fn gen_comment(&self, comment: CommentKind) -> RawItem {
        self.gen_token(RawToken::Comment(comment))
    }
    #[inline]
    fn gen_token(&self, ty: RawToken) -> RawItem {
        RawItem {
            start: self.current_start,
            end: self.stream.idx,
            ty,
        }
    }
    #[inline]
    pub fn skip_whitespace(&mut self) {
        while self.stream.at_whitespace() {
            self.stream.skip(1);
        }
    }
    #[inline]
    fn at_new_line(&mut self) -> bool {
        self.stream.at_new_line()
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
            let item = t.next_();
            println!("{:?}", item.ty);
            assert!(item.ty.is_punct())
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
        ];
        for s in STRINGS {
            let item = Tokenizer::new(s).next_();
            match &item.ty {
                RawToken::String(ref lit) => {
                    if &s[0..1] == "'" {
                        match lit {
                            StringKind::Single => (),
                            StringKind::Double => {
                                panic!("Expected single quote string, found double")
                            }
                        }
                    } else {
                        match lit {
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
            println!("attempting {}", i);
            let item = Tokenizer::new(i).next_();
            assert_eq!(item.ty, RawToken::Ident);
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
            ".0",
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
        ];
        for n in NUMBERS {
            println!("n: {}", n);
            let mut t = Tokenizer::new(n);
            let item = t.next_();
            assert!(match item.ty {
                RawToken::Numeric(_) => true,
                _ => false,
            });
        }
    }

    #[test]
    fn tokenizer_regex() {
        static REGEX: &[&str] = &[
            r#"x/"#,
            r#"|/"#,
            r#"|||/"#,
            r#"^$\b\B/"#,
            r#"(?=(?!(?:(.))))/"#,
            r#"a.\f\n\r\t\v\0\[\-\/\\\x00\u0000/"#,
            r#"\d\D\s\S\w\W/"#,
            r#"\ca\cb\cc\cd\ce\cf\cg\ch\ci\cj\ck\cl\cm\cn\co\cp\cq\cr\cs\ct\cu\cv\cw\cx\cy\cz/"#,
            r#"\cA\cB\cC\cD\cE\cF\cG\cH\cI\cJ\cK\cL\cM\cN\cO\cP\cQ\cR\cS\cT\cU\cV\cW\cX\cY\cZ/"#,
            r#"[a-z-]/"#,
            r#"[^\b\-^]/"#,
            r#"[/\]\\]/"#,
            r#"./i"#,
            r#"./g"#,
            r#"./m"#,
            r#"./igm"#,
            r#".*/"#,
            r#".*?/"#,
            r#".+/"#,
            r#".+?/"#,
            r#".?/"#,
            r#".??/"#,
            r#".{0}/"#,
            r#".{0,}/"#,
            r#".{0,0}/"#,
        ];
        for r in REGEX {
            let mut t = Tokenizer::new(r);
            let item = t.next_regex();
            assert!(match item.ty {
                RawToken::RegEx(_) => true,
                _ => false,
            });
        }
    }

    #[test]
    fn tokenizer_template() {
        let subbed = "`things and stuff times ${} and animals and minerals`";
        println!("subbed: {}", subbed);
        let mut t = Tokenizer::new(subbed);
        let start = t.next_();
        assert_eq!(start.ty, RawToken::Template(TemplateKind::Head));
        let end = t.next_();
        assert_eq!(end.ty, RawToken::Template(TemplateKind::Tail));
        let no_sub = "`things and stuff`";
        println!("no_sub {}", no_sub);
        t = Tokenizer::new(no_sub);
        let one = t.next_();
        assert_eq!(one.ty, RawToken::Template(TemplateKind::NoSub));
        let escaped_sub = r#"`\0\n\x0A\u000A\u{A}${}`"#;
        println!("escaped_sub: {}", escaped_sub);
        t = Tokenizer::new(escaped_sub);
        let start = t.next_();
        assert_eq!(start.ty, RawToken::Template(TemplateKind::Head));
        let end = t.next_();
        assert_eq!(end.ty, RawToken::Template(TemplateKind::Tail));
        let escaped_no_sub = r#"`a\${b`"#;
        println!("escaped_no_sub: {}", escaped_no_sub);
        t = Tokenizer::new(escaped_no_sub);
        let one = t.next_();
        assert_eq!(one.ty, RawToken::Template(TemplateKind::NoSub));
        let double_sub =
            "`things and stuff times ${} and animals and minerals ${} and places and people`";
        println!("double_sub: {}", double_sub);
        t = Tokenizer::new(double_sub);
        let start = t.next_();
        assert_eq!(start.ty, RawToken::Template(TemplateKind::Head));
        let mid = t.next_();
        assert_eq!(mid.ty, RawToken::Template(TemplateKind::Body));
        let end = t.next_();
        assert_eq!(end.ty, RawToken::Template(TemplateKind::Tail));
    }

    #[test]
    fn tokenizer_bools() {
        for b in &["true", "false"] {
            let mut t = Tokenizer::new(b);
            let item = t.next_();
            assert!(match item.ty {
                RawToken::Boolean(_) => true,
                _ => false,
            });
        }
    }

    #[test]
    fn tokenizer_null() {
        let mut t = Tokenizer::new("null");
        let item = t.next_();
        assert_eq!(item.ty, RawToken::Null);
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
            let item = t.next_();
            assert!(match item.ty {
                RawToken::Keyword(_) => true,
                _ => false,
            })
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
            let item = t.next_();
            assert!(item.ty.is_comment());
        }
    }
}
