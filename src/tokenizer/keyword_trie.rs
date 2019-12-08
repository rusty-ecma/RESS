use crate::tokenizer::{RawKeyword, RawToken, Res, Tokenizer};

type MaybeKeyword = Res<Option<RawToken>>;

impl<'a> Tokenizer<'a> {
    pub(crate) fn keyword(&mut self, start: char) -> MaybeKeyword {
        match start {
            'a' => self.a_keywords(),
            'b' => self.b_keywords(),
            'c' => self.c_keywords(),
            'd' => self.d_keywords(),
            'e' => self.e_keywords(),
            'f' => self.f_keywords(),
            'i' => self.i_keywords(),
            'l' => self.l_keywords(),
            'n' => self.n_keywords(),
            'p' => self.p_keywords(),
            'r' => self.r_keywords(),
            's' => self.s_keywords(),
            't' => self.t_keywords(),
            'v' => self.v_keywords(),
            'w' => self.w_keywords(),
            'y' => self.y_keywords(),
            _ => Ok(None),
        }
    }
    fn a_keywords(&mut self) -> MaybeKeyword {
        self.suffix_for_token("wait", RawToken::Keyword(RawKeyword::Await))
    }
    fn b_keywords(&mut self) -> MaybeKeyword {
        self.suffix_for_token("reak", RawToken::Keyword(RawKeyword::Break))
    }

    fn c_keywords(&mut self) -> MaybeKeyword {
        if self.eat_ch_or_escaped('a')? {
            if self.eat_ch_or_escaped('s')? {
                self.suffix_for_token("e", RawToken::Keyword(RawKeyword::Case))
            } else if self.eat_ch_or_escaped('t')? {
                self.suffix_for_token("ch", RawToken::Keyword(RawKeyword::Catch))
            } else {
                Ok(None)
            }
        } else if self.eat_ch_or_escaped('l')? {
            self.suffix_for_token("ass", RawToken::Keyword(RawKeyword::Class))
        } else if self.eat_ch_or_escaped('o')? && self.eat_ch_or_escaped('n')? {
            if self.eat_ch_or_escaped('s')? {
                self.suffix_for_token("t", RawToken::Keyword(RawKeyword::Const))
            } else if self.eat_ch_or_escaped('t')? {
                self.suffix_for_token("inue", RawToken::Keyword(RawKeyword::Continue))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn d_keywords(&mut self) -> MaybeKeyword {
        if self.eat_ch_or_escaped('e')? {
            if self.eat_ch_or_escaped('b')? {
                self.suffix_for_token("ugger", RawToken::Keyword(RawKeyword::Debugger))
            } else if self.eat_ch_or_escaped('f')? {
                self.suffix_for_token("ault", RawToken::Keyword(RawKeyword::Default))
            } else if self.eat_ch_or_escaped('l')? {
                self.suffix_for_token("ete", RawToken::Keyword(RawKeyword::Delete))
            } else {
                Ok(None)
            }
        } else if self.eat_ch_or_escaped('o')? {
            if self.at_ident_end() {
                Ok(Some(RawToken::Keyword(RawKeyword::Do)))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn e_keywords(&mut self) -> MaybeKeyword {
        if self.eat_ch_or_escaped('l')? {
            self.suffix_for_token("se", RawToken::Keyword(RawKeyword::Else))
        } else if self.eat_ch_or_escaped('n')? {
            self.suffix_for_token("um", RawToken::Keyword(RawKeyword::Enum))
        } else if self.eat_ch_or_escaped('x')? {
            if self.eat_ch_or_escaped('p')? {
                self.suffix_for_token("ort", RawToken::Keyword(RawKeyword::Export))
            } else if self.eat_ch_or_escaped('t')? {
                self.suffix_for_token("ends", RawToken::Keyword(RawKeyword::Extends))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn f_keywords(&mut self) -> MaybeKeyword {
        if self.eat_ch_or_escaped('a')? {
            self.suffix_for_token("lse", RawToken::Boolean(false))
        } else if self.eat_ch_or_escaped('i')? {
            self.suffix_for_token("nally", RawToken::Keyword(RawKeyword::Finally))
        } else if self.eat_ch_or_escaped('o')? {
            self.suffix_for_token("r", RawToken::Keyword(RawKeyword::For))
        } else if self.eat_ch_or_escaped('u')? {
            self.suffix_for_token("nction", RawToken::Keyword(RawKeyword::Function))
        } else {
            Ok(None)
        }
    }

    fn i_keywords(&mut self) -> MaybeKeyword {
        if self.eat_ch_or_escaped('f')? && self.at_ident_end() {
            Ok(Some(RawToken::Keyword(RawKeyword::If)))
        } else if self.eat_ch_or_escaped('m')? && self.eat_ch_or_escaped('p')? {
            if self.eat_ch_or_escaped('l')? {
                self.suffix_for_token("ements", RawToken::Keyword(RawKeyword::Implements))
            } else if self.eat_ch_or_escaped('o')? {
                self.suffix_for_token("rt", RawToken::Keyword(RawKeyword::Import))
            } else {
                Ok(None)
            }
        } else if self.eat_ch_or_escaped('n')? {
            if self.eat_ch_or_escaped('s')? {
                self.suffix_for_token("tanceof", RawToken::Keyword(RawKeyword::InstanceOf))
            } else if self.eat_ch_or_escaped('t')? {
                self.suffix_for_token("erface", RawToken::Keyword(RawKeyword::Interface))
            } else if self.at_ident_end() {
                Ok(Some(RawToken::Keyword(RawKeyword::In)))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn l_keywords(&mut self) -> MaybeKeyword {
        self.suffix_for_token("et", RawToken::Keyword(RawKeyword::Let))
    }

    fn n_keywords(&mut self) -> MaybeKeyword {
        if self.eat_ch_or_escaped('e')? {
            self.suffix_for_token("w", RawToken::Keyword(RawKeyword::New))
        } else if self.eat_ch_or_escaped('u')? {
            self.suffix_for_token("ll", RawToken::Null)
        } else {
            Ok(None)
        }
    }

    fn p_keywords(&mut self) -> MaybeKeyword {
        if self.eat_ch_or_escaped('a')? {
            self.suffix_for_token("ckage", RawToken::Keyword(RawKeyword::Package))
        } else if self.eat_ch_or_escaped('r')? {
            if self.eat_ch_or_escaped('i')? {
                self.suffix_for_token("vate", RawToken::Keyword(RawKeyword::Private))
            } else if self.eat_ch_or_escaped('o')? {
                self.suffix_for_token("tected", RawToken::Keyword(RawKeyword::Protected))
            } else {
                Ok(None)
            }
        } else if self.eat_ch_or_escaped('u')? {
            self.suffix_for_token("blic", RawToken::Keyword(RawKeyword::Public))
        } else {
            Ok(None)
        }
    }

    fn r_keywords(&mut self) -> MaybeKeyword {
        self.suffix_for_token("eturn", RawToken::Keyword(RawKeyword::Return))
    }

    fn s_keywords(&mut self) -> MaybeKeyword {
        if self.eat_ch_or_escaped('t')? {
            self.suffix_for_token("atic", RawToken::Keyword(RawKeyword::Static))
        } else if self.eat_ch_or_escaped('u')? {
            self.suffix_for_token("per", RawToken::Keyword(RawKeyword::Super))
        } else if self.eat_ch_or_escaped('w')? {
            self.suffix_for_token("itch", RawToken::Keyword(RawKeyword::Switch))
        } else {
            Ok(None)
        }
    }

    fn t_keywords(&mut self) -> MaybeKeyword {
        if self.eat_ch_or_escaped('h')? {
            if self.eat_ch_or_escaped('i')? {
                self.suffix_for_token("s", RawToken::Keyword(RawKeyword::This))
            } else if self.eat_ch_or_escaped('r')? {
                self.suffix_for_token("ow", RawToken::Keyword(RawKeyword::Throw))
            } else {
                Ok(None)
            }
        } else if self.eat_ch_or_escaped('r')? {
            if self.eat_ch_or_escaped('u')? {
                self.suffix_for_token("e", RawToken::Boolean(true))
            } else if self.eat_ch_or_escaped('y')? && self.at_ident_end() {
                Ok(Some(RawToken::Keyword(RawKeyword::Try)))
            } else {
                Ok(None)
            }
        } else if self.eat_ch_or_escaped('y')? {
            self.suffix_for_token("peof", RawToken::Keyword(RawKeyword::TypeOf))
        } else {
            Ok(None)
        }
    }

    fn v_keywords(&mut self) -> MaybeKeyword {
        if self.eat_ch_or_escaped('a')? {
            self.suffix_for_token("r", RawToken::Keyword(RawKeyword::Var))
        } else if self.eat_ch_or_escaped('o')? {
            self.suffix_for_token("id", RawToken::Keyword(RawKeyword::Void))
        } else {
            Ok(None)
        }
    }

    fn w_keywords(&mut self) -> MaybeKeyword {
        if self.eat_ch_or_escaped('h')? {
            self.suffix_for_token("ile", RawToken::Keyword(RawKeyword::While))
        } else if self.eat_ch_or_escaped('i')? {
            self.suffix_for_token("th", RawToken::Keyword(RawKeyword::With))
        } else {
            Ok(None)
        }
    }

    fn y_keywords(&mut self) -> MaybeKeyword {
        self.suffix_for_token("ield", RawToken::Keyword(RawKeyword::Yield))
    }

    fn suffix_for_token(&mut self, suffix: &str, tok: RawToken) -> MaybeKeyword {
        if self.eat_chs_or_escaped(suffix)? {
            if self.at_ident_end() {
                Ok(Some(tok))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn at_ident_end(&mut self) -> bool {
        if let Some(c) = self.stream.next_char() {
            if !Self::is_id_continue(c) && c != '\u{200C}' && c != '\u{200D}' {
                let _ = self.stream.prev_char();
                true
            } else {
                false
            }
        } else {
            true
        }
    }

    fn eat_chs_or_escaped(&mut self, chars: &str) -> Res<bool> {
        for c in chars.chars() {
            if !self.eat_ch_or_escaped(c)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub(crate) fn eat_ch_or_escaped(&mut self, ch: char) -> Res<bool> {
        Ok(if self.look_ahead_byte_matches(ch) {
            self.stream.skip_bytes(1);
            true
        } else if self.look_ahead_matches("\\u") {
            let start = self.stream.idx;
            self.stream.skip_bytes(1);
            let c = self.escaped_ident_part()?;
            if c != ch {
                self.stream.idx = start;
                false
            } else {
                true
            }
        } else {
            false
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn keyword_await() {
        test_with_escapes("await", RawToken::Keyword(RawKeyword::Await));
    }

    #[test]
    fn keyword_break() {
        test_with_escapes("break", RawToken::Keyword(RawKeyword::Break));
    }

    #[test]
    fn keyword_case() {
        test_with_escapes("case", RawToken::Keyword(RawKeyword::Case));
    }

    #[test]
    fn keyword_catch() {
        test_with_escapes("catch", RawToken::Keyword(RawKeyword::Catch));
    }
    #[test]
    fn keyword_const() {
        test_with_escapes("const", RawToken::Keyword(RawKeyword::Const));
    }
    #[test]
    fn keyword_continue() {
        test_with_escapes("continue", RawToken::Keyword(RawKeyword::Continue));
    }
    #[test]
    fn keyword_class() {
        test_with_escapes("class", RawToken::Keyword(RawKeyword::Class));
    }
    #[test]
    fn keyword_debugger() {
        test_with_escapes("debugger", RawToken::Keyword(RawKeyword::Debugger));
    }
    #[test]
    fn keyword_default() {
        test_with_escapes("default", RawToken::Keyword(RawKeyword::Default));
    }
    #[test]
    fn keyword_delete() {
        test_with_escapes("delete", RawToken::Keyword(RawKeyword::Delete));
    }
    #[test]
    fn keyword_do() {
        test_with_escapes("do", RawToken::Keyword(RawKeyword::Do));
    }

    #[test]
    fn keyword_else() {
        test_with_escapes("else", RawToken::Keyword(RawKeyword::Else));
    }
    #[test]
    fn keyword_enum() {
        test_with_escapes("enum", RawToken::Keyword(RawKeyword::Enum));
    }
    #[test]
    fn keyword_export() {
        test_with_escapes("export", RawToken::Keyword(RawKeyword::Export));
    }
    #[test]
    fn keyword_extends() {
        test_with_escapes("extends", RawToken::Keyword(RawKeyword::Extends));
    }
    #[test]
    fn keyword_false() {
        test_with_escapes("false", RawToken::Boolean(false));
    }
    #[test]
    fn keyword_finally() {
        test_with_escapes("finally", RawToken::Keyword(RawKeyword::Finally));
    }
    #[test]
    fn keyword_for() {
        test_with_escapes("for", RawToken::Keyword(RawKeyword::For));
    }
    #[test]
    fn keyword_function() {
        test_with_escapes("function", RawToken::Keyword(RawKeyword::Function));
    }
    #[test]
    fn keyword_if() {
        test_with_escapes("if", RawToken::Keyword(RawKeyword::If));
    }
    #[test]
    fn keyword_implements() {
        test_with_escapes("implements", RawToken::Keyword(RawKeyword::Implements));
    }
    #[test]
    fn keyword_import() {
        test_with_escapes("import", RawToken::Keyword(RawKeyword::Import));
    }
    #[test]
    fn keyword_in() {
        test_with_escapes("in", RawToken::Keyword(RawKeyword::In));
    }
    #[test]
    fn keyword_instance_of() {
        test_with_escapes("instanceof", RawToken::Keyword(RawKeyword::InstanceOf));
    }
    #[test]
    fn keyword_interface() {
        test_with_escapes("interface", RawToken::Keyword(RawKeyword::Interface));
    }
    #[test]
    fn keyword_let() {
        test_with_escapes("let", RawToken::Keyword(RawKeyword::Let));
    }
    #[test]
    fn keyword_new() {
        test_with_escapes("new", RawToken::Keyword(RawKeyword::New));
    }
    #[test]
    fn keyword_null() {
        test_with_escapes("null", RawToken::Null);
    }
    #[test]
    fn keyword_package() {
        test_with_escapes("package", RawToken::Keyword(RawKeyword::Package));
    }
    #[test]
    fn keyword_private() {
        test_with_escapes("private", RawToken::Keyword(RawKeyword::Private));
    }
    #[test]
    fn keyword_protected() {
        test_with_escapes("protected", RawToken::Keyword(RawKeyword::Protected));
    }
    #[test]
    fn keyword_public() {
        test_with_escapes("public", RawToken::Keyword(RawKeyword::Public));
    }
    #[test]
    fn keyword_return() {
        test_with_escapes("return", RawToken::Keyword(RawKeyword::Return));
    }
    #[test]
    fn keyword_static() {
        test_with_escapes("static", RawToken::Keyword(RawKeyword::Static));
    }
    #[test]
    fn keyword_super() {
        test_with_escapes("super", RawToken::Keyword(RawKeyword::Super));
    }
    #[test]
    fn keyword_switch() {
        test_with_escapes("switch", RawToken::Keyword(RawKeyword::Switch));
    }
    #[test]
    fn keyword_this() {
        test_with_escapes("this", RawToken::Keyword(RawKeyword::This));
    }
    #[test]
    fn keyword_throw() {
        test_with_escapes("throw", RawToken::Keyword(RawKeyword::Throw));
    }
    #[test]
    fn keyword_true() {
        test_with_escapes("true", RawToken::Boolean(true));
    }
    #[test]
    fn keyword_try() {
        test_with_escapes("try", RawToken::Keyword(RawKeyword::Try));
    }
    #[test]
    fn keyword_type_of() {
        test_with_escapes("typeof", RawToken::Keyword(RawKeyword::TypeOf));
    }
    #[test]
    fn keyword_var() {
        test_with_escapes("var", RawToken::Keyword(RawKeyword::Var));
    }
    #[test]
    fn keyword_void() {
        test_with_escapes("void", RawToken::Keyword(RawKeyword::Void));
    }
    #[test]
    fn keyword_while() {
        test_with_escapes("while", RawToken::Keyword(RawKeyword::While));
    }
    #[test]
    fn keyword_with() {
        test_with_escapes("with", RawToken::Keyword(RawKeyword::With));
    }
    #[test]
    fn keyword_yield() {
        test_with_escapes("yield", RawToken::Keyword(RawKeyword::Yield));
    }

    fn test_with_escapes(k: &str, expect: RawToken) {
        let start = k.chars().next().expect("empty keyword");
        let first = test_keyword(start, k)
            .expect(&format!("failed to parse {}", k))
            .expect(&format!("failed to parse {}", k));
        assert_eq!(first, expect);
        let mut escape_char_code;
        let mut escape_code_points;
        for i in 0..k.chars().count() {
            escape_char_code = String::new();
            escape_code_points = String::new();
            for (j, c) in k.chars().enumerate() {
                if j == i {
                    escape_char_code.push_str(&format!(r#"\u{:04X}"#, c as u8));
                    escape_code_points.push_str(&format!(r#"\u{{{:06X}}}"#, c as u8));
                } else {
                    escape_char_code.push(c);
                    escape_code_points.push(c);
                }
            }
            let second = test_keyword(start, &escape_char_code)
                .expect(&format!(
                    "failed to parse escaped keyword {}",
                    escape_char_code
                ))
                .expect(&format!(
                    "failed to parse escaped keyword {}",
                    escape_char_code
                ));
            assert_eq!(
                second, expect,
                "{} doesn't match expected keyword",
                escape_char_code
            );
            let third = test_keyword(start, &escape_code_points)
                .expect(&format!(
                    "failed to parse escaped keyword {}",
                    escape_code_points
                ))
                .expect(&format!(
                    "failed to parse escaped keyword {}",
                    escape_code_points
                ));
            assert_eq!(
                third, expect,
                "{} doesn't match expected keyword",
                escape_code_points
            );
        }
        let not = format!("{}_not", k);
        assert_eq!(
            test_keyword(start, &not).expect(&format!("Failed to parse not keyword {}", not)),
            None
        );
    }

    fn test_keyword(start: char, k: &str) -> MaybeKeyword {
        dbg!(start);
        dbg!(k);
        let mut t = Tokenizer::new(k);
        assert!(
            t.eat_ch_or_escaped(start)?,
            "start didn't match first character {}, {}",
            start,
            k
        );
        t.keyword(start)
    }
}
