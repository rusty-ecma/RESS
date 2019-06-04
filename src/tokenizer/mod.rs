use unic_ucd_ident::{is_id_continue, is_id_start};

mod buffer;

use crate::{
    refs::{
        RefToken as Token,
        tokens::{
            StringLit,
            Number,
        },
    },
    Punct,
    Keyword,
    OpenCurlyKind,
};
pub struct RawToken {
    ty: Token,
    start: usize,
    end: usize,
}

pub struct Tokenizer<'a> {
    stream: buffer::JSBuffer<'a>,
    current_start: usize,
    curly_stack: Vec<OpenCurlyKind>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(stream: &'a str) -> Self {
        Tokenizer {
            current_start: 0,
            stream: stream.into(),
            curly_stack: Vec::new(),
        }
    }

    pub fn next_(&mut self) -> RawToken {
        self.current_start = self.stream.idx;
        let next_char = match self.stream.next_char() {
            Some(ch) => ch,
            None => return RawToken {
                start: self.stream.idx,
                end: self.stream.idx,
                ty: Token::EoF
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
        if next_char == '`' 
        || (next_char == '}' && self.curly_stack.last() == Some(&OpenCurlyKind::Template)) {
            return self.template(next_char);
        }
        self.punct(next_char)
    }

    fn ident(&mut self, start: char) -> RawToken {
        if start == '\\' {
            // TODO validate escaped ident start
            self.escaped_ident_part();
        }
        while let Some(c) = self.stream.next_char() {
            if c == '\\' {
                self.escaped_ident_part();
            }
            if !is_id_continue(c)
            && c != '$'
            && c != '\u{200C}'
            && c != '\u{200D}' {
                // if we have moved past the last valid identifier, go back 1
                let _ = self.stream.prev_char();
                break;
            }
        }
        if let Some(k) = self.at_keyword() {
            self.gen_token(Token::Keyword(k))
        } else {
            self.gen_token(Token::Ident)
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
            panic!("invalid unicode escape sequence starting at {}", self.current_start);
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
            code += u32::from_str_radix(c.encode_utf8(&mut [0;4]), 16)
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

    fn string(&mut self, quote: char) -> RawToken {
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
                || self.look_ahead_matches("\u{2029}")) && !escaped {
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
            StringLit::Double
        } else {
            StringLit::Single
        };
        self.gen_token(Token::String(inner))
    }
    fn punct(&mut self, c: char) -> RawToken {
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
            '#' => self.gen_punct(Punct::Private),
            '~' => self.gen_punct(Punct::BitwiseNot),
            '.' => {
                // ...
                if self.look_ahead_matches("..") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::Spread)
                } else {
                    self.gen_punct(Punct::Period)
                }
            },
            '>' => {
                if self.look_ahead_matches(">>=") {
                    self.stream.skip(3);
                    self.gen_punct(Punct::UnsignedRightShiftAssign)
                } else if self.look_ahead_matches(">>")  {
                    self.stream.skip(2);
                    self.gen_punct(Punct::UnsignedRightShift)
                } else if self.look_ahead_matches(">=") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::RightShiftAssign)
                } else if self.look_ahead_matches(">") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::RightShift)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::GreaterThanEqual)
                } else {
                    self.gen_punct(Punct::GreaterThan)
                }
            },
            '<' => {
                if self.look_ahead_matches("<=") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::LeftShiftAssign)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::LessThanEqual)
                } else if self.look_ahead_matches("<") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::LeftShift)
                } else {
                    self.gen_punct(Punct::LessThan)
                }
            },
            '=' => {
                if self.look_ahead_matches("==") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::StrictEquals)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::Equal)
                } else if self.look_ahead_matches(">") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::FatArrow)
                } else {
                    self.gen_punct(Punct::Assign)
                }
            },
            '!' => {
                if self.look_ahead_matches("==") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::StrictNotEquals)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::NotEqual)
                } else {
                    self.gen_punct(Punct::Not)
                }
            },
            '*' => {
                if self.look_ahead_matches("*=") {
                    self.stream.skip(2);
                    self.gen_punct(Punct::ExponentAssign)
                } else if self.look_ahead_matches("*") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::Exponent)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::MultiplyAssign)
                } else {
                    self.gen_punct(Punct::Asterisk)
                }
            },
            '&' => {
                if self.look_ahead_matches("&") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::LogicalAnd)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::BitwiseAndAssign)
                } else {
                    self.gen_punct(Punct::And)
                }
            },
            '|' => {
                if self.look_ahead_matches("|") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::LogicalOr)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::BitwiseOrAssign)
                } else {
                    self.gen_punct(Punct::Pipe)
                }
            },
            '+' => {
                if self.look_ahead_matches("+") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::Increment)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::AddAssign)
                } else {
                    self.gen_punct(Punct::Plus)
                }
            },
            '-' => {
                if self.look_ahead_matches("-") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::Decrement)
                } else if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::SubtractAssign)
                } else {
                    self.gen_punct(Punct::Minus)
                }
            },
            '/' => {
                if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::DivideAssign)
                } else {
                    self.gen_punct(Punct::ForwardSlash)
                }
            },
            '%' => {
                if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::ModuloAssign)
                } else {
                    self.gen_punct(Punct::Modulo)
                }
            },
            '^' => {
                if self.look_ahead_matches("=") {
                    self.stream.skip(1);
                    self.gen_punct(Punct::BitwiseXOrAssign)
                } else {
                    self.gen_punct(Punct::Caret)
                }
            }
            _ => unimplemented!("unknown punct {}", c),
        }
    }
    fn number(&mut self, start: char) -> RawToken {
        assert!(start == '.' || start.is_digit(10));
        if let Some(next) = self.stream.next_char() {
            if start == '.' {
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
    fn template(&mut self, start: char) -> RawToken {
        unimplemented!("template");
    }
    fn hex_number(&mut self) -> RawToken {
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
        self.gen_number(Number::Hex)
    }
    fn oct_number(&mut self) -> RawToken {
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
        self.gen_number(Number::Oct)
    }
    fn bin_number(&mut self) -> RawToken {
        if let Some(c)  = self.stream.next_char() {
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
        self.gen_number(Number::Bin)
    }

    fn dec_number(&mut self, seen_point: bool) -> RawToken {
        let mut maybe_e: Option<char> = None;
        if !seen_point {
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
        self.gen_number(Number::Dec)
    }
    fn look_ahead_matches(&self, s: &str) -> bool {
        self.stream.look_ahead_matches(s.as_bytes())
    }
    fn gen_punct(&self, p: Punct) -> RawToken {
        self.gen_token(Token::Punct(p))
    }
    fn gen_number(&self, n: Number) -> RawToken {
        self.gen_token(Token::Numeric(n))
    }
    fn gen_token(&self, ty: Token) -> RawToken {
        RawToken {
            start: self.current_start,
            end: self.stream.idx,
            ty,
        }
    }

    fn at_whitespace(&mut self) -> bool {
        self.stream.at_whitespace()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tokenizer_punct() {
        static PUNCTS: &[&str] = &[
            "{", "}", "(", ")", ".", ";", ",", "[", "]", ":", "?", "~", ">", "<", "=", "!", "+", "-", "/",
            "*", "%", "&", "|", "^", ">>>=", //3 char
            "...", "===", "!==", ">>>", "<<=", ">>=", "**=", //2 char
            "&&", "||", "==", "!=", "+=", "-=", "*=", "/=", "++", "--", "<<", ">>", "&=", "|=", "^=", "%=",
            "<=", ">=", "=>", "**",
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
                Token::String(ref lit) => {
                    if &s[0..1] == "'" {
                        match lit {
                            StringLit::Single => (),
                            StringLit::Double => panic!("Expected single quote string, found double"),
                        }
                    } else {
                        match lit {
                            StringLit::Single => panic!("expected double quote string, found single"),
                            StringLit::Double => (),
                        }
                    }
                },
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
            assert!(item.ty.is_ident());
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
            assert!(item.ty.is_number());
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
            let item = t.next_();
            assert!(item.ty.is_regex());
        }
    }

    #[test]
    fn tokenizer_template_start() {
        static TEMPLATE_STARTS: &[&str] = &[
            "`things and stuff times ${",
            "`things and stuff`",
            r#"`a\${b`"#,
            r#"`\0\n\x0A\u000A\u{A}${"#,
        ];
        for s in TEMPLATE_STARTS {
            let mut t = Tokenizer::new(s);
            let item = t.next_();
            assert!(item.ty.is_template_head());
        }
    }
    #[test]
    fn tokenizer_template_cont() {
        static TEMPLATE_CONTINUATIONS: &[&str] = &[
            " and animals and minerals`",
            "`}`",
            " and animals and minerals`",
            " and places and people ${",
        ];;
        for s in TEMPLATE_CONTINUATIONS {
            let mut t = Tokenizer::new(s);
            let item = t.next_();
            assert!(item.ty.is_template_body() || item.ty.is_template_tail());
        }
    }

    #[test]
    fn tokenizer_bools() {
        for b in &["true", "false"] {
            let mut t = Tokenizer::new(b);
            let item = t.next_();
            assert!(item.ty.is_bool());
        }
    }

    #[test]
    fn tokenizer_null() {
        let mut t = Tokenizer::new("null");
        let item = t.next_();
        assert!(item.ty.is_null());
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
            assert!(item.ty.is_keyword())
        }
    }
}