use ress::prelude::StringLit::Single;
use ress::prelude::*;
use ress::tokens::InnerString;

#[test]
fn vue_number_error() {
    let js = "refElm = isUndef(newCh[newEndIdx + 1]) ? null : newCh[newEndIdx + 1].elm;";
    for item in Scanner::new(js) {
        println!("{:?}", item);
    }
}
#[test]
fn moment_regex_error() {
    let js = r"function removeFormattingTokens(input) {
        if (input.match(/\[[\s\S]/)) {
            return input.replace(/^\[|\]$/g, '');
        }
        return input.replace(/\\/g, '');
    }";
    for item in Scanner::new(js) {
        println!("{:?}", item);
    }
}

#[test]
fn number_member() {
    compare(
        "20..toString()",
        &[
            Token::Number("20.".into()),
            Token::Punct(Punct::Period),
            Token::Ident("toString".into()),
            Token::Punct(Punct::OpenParen),
            Token::Punct(Punct::CloseParen),
        ],
    );
}
#[test]
fn if_then_regex() {
    compare(
        "if (1) /a/",
        &[
            Token::Keyword(Keyword::If("If")),
            Token::Punct(Punct::OpenParen),
            Token::Number("1".into()),
            Token::Punct(Punct::CloseParen),
            Token::RegEx(RegEx {
                body: "a",
                flags: None,
            }),
        ],
    );
}

#[test]
fn line_terminator_in_string_literal() {
    let js = "' '";
    for _ in Scanner::new(js) {
        // just testing for panics on the byte index
        // for now
        //TODO: Allow this character in string literals
        // as per spec under feature "json superset"
    }
}

#[test]
fn lots_of_arcs() {
    let mut top = "".to_string();
    let mut bottom = "[".to_string();
    let ascii_start = 97;
    for i in 0..26 {
        let id = std::char::from_u32(ascii_start + i).unwrap();
        let obj = format!("{{{}:{}}}", id, i);
        top.push_str(&format!("({})", obj));
        if i != 25 {
            top.push_str(", ");
        }
        bottom.push_str(&format!("{},", obj));
    }
    bottom.push(']');
    let js = format!("{}\n\n{}", top, bottom);

    let s = Scanner::new(&js);
    for item in s {
        println!("{:?}", item.unwrap());
    }
}

#[test]
fn div_over_regex() {
    let js = "if (true) {
  ({} / function(){return 1});
}
";
    for tok in panicking_scanner(js) {
        eprintln!("{:?}", tok)
    }
}
#[test]
fn regex_over_div() {
    let js = "{}/\\d/g;;";
    compare(
        js,
        &[
            Token::Punct(Punct::OpenBrace),
            Token::Punct(Punct::CloseBrace),
            Token::RegEx(RegEx::from_parts("\\d", Some("g"))),
            Token::Punct(Punct::SemiColon),
            Token::Punct(Punct::SemiColon),
        ],
    );
}

#[test]
// #[ignore = "regex needs fixing, this is a valid regex"]
fn regex_over_div3() {
    let js = "function name(){}/\\d/g;;";
    compare(
        js,
        &[
            Token::Keyword(Keyword::Function("function")),
            Token::Ident("name".into()),
            Token::Punct(Punct::OpenParen),
            Token::Punct(Punct::CloseParen),
            Token::Punct(Punct::OpenBrace),
            Token::Punct(Punct::CloseBrace),
            Token::RegEx(RegEx::from_parts("\\d", Some("g"))),
            Token::Punct(Punct::SemiColon),
            Token::Punct(Punct::SemiColon),
        ],
    );
}

#[test]
#[ignore = "regex needs fixing, this is a valid regex"]
fn regex_over_div4() {
    pretty_env_logger::formatted_builder()
        .is_test(true)
        .try_init()
        .ok();
    let js = "'use strict';function name(){}/\\d/g;;";
    compare(
        js,
        &[
            Token::String(StringLit::single("use strict", false)),
            Token::Punct(Punct::SemiColon),
            Token::Keyword(Keyword::Function("function")),
            Token::Ident("name".into()),
            Token::Punct(Punct::OpenParen),
            Token::Punct(Punct::CloseParen),
            Token::Punct(Punct::OpenBrace),
            Token::Punct(Punct::CloseBrace),
            Token::RegEx(RegEx::from_parts("\\d", Some("g"))),
            Token::Punct(Punct::SemiColon),
            Token::Punct(Punct::SemiColon),
        ],
    );
}

#[test]
fn html_comment_close() {
    let js = "
--> stuff is in a comment
  --> also a comment
/*multi-comment*/--> with trailer
/*---*/
let a;
/*first comment*/ /*second comment*/--> with trailer";
    compare(
        js,
        &[
            Token::Comment(Comment {
                kind: ress::tokens::CommentKind::Html,
                content: "",
                tail_content: Some(" stuff is in a comment"),
            }),
            Token::Comment(Comment {
                kind: ress::tokens::CommentKind::Html,
                content: "",
                tail_content: Some(" also a comment"),
            }),
            Token::Comment(Comment {
                kind: ress::tokens::CommentKind::Multi,
                content: "multi-comment",
                tail_content: Some(" with trailer"),
            }),
            Token::Comment(Comment {
                kind: ress::tokens::CommentKind::Multi,
                content: "---",
                tail_content: None,
            }),
            Token::Keyword(Keyword::Let("let")),
            Token::Ident("a".into()),
            Token::Punct(Punct::SemiColon),
            Token::Comment(Comment {
                kind: ress::tokens::CommentKind::Multi,
                content: "first comment",
                tail_content: None,
            }),
            Token::Comment(Comment {
                kind: ress::tokens::CommentKind::Multi,
                content: "second comment",
                tail_content: Some(" with trailer"),
            }),
        ],
    );
}
#[test]
fn decrement_greater_than() {
    compare(
        "for (var x = 0; x --> 0;);",
        &[
            Token::Keyword(Keyword::For("for")),
            Token::Punct(Punct::OpenParen),
            Token::Keyword(Keyword::Var("var")),
            Token::Ident("x".into()),
            Token::Punct(Punct::Equal),
            Token::Number("0".into()),
            Token::Punct(Punct::SemiColon),
            Token::Ident("x".into()),
            Token::Punct(Punct::DoubleDash),
            Token::Punct(Punct::GreaterThan),
            Token::Number("0".into()),
            Token::Punct(Punct::SemiColon),
            Token::Punct(Punct::CloseParen),
            Token::Punct(Punct::SemiColon),
        ],
    )
}
#[test]
fn decrement_greater_than_inline_multi() {
    compare(
        "for (var x = 0; x /**/--> 0;);",
        &[
            Token::Keyword(Keyword::For("for")),
            Token::Punct(Punct::OpenParen),
            Token::Keyword(Keyword::Var("var")),
            Token::Ident("x".into()),
            Token::Punct(Punct::Equal),
            Token::Number("0".into()),
            Token::Punct(Punct::SemiColon),
            Token::Ident("x".into()),
            Token::Comment(Comment::new_multi_line("")),
            Token::Punct(Punct::DoubleDash),
            Token::Punct(Punct::GreaterThan),
            Token::Number("0".into()),
            Token::Punct(Punct::SemiColon),
            Token::Punct(Punct::CloseParen),
            Token::Punct(Punct::SemiColon),
        ],
    )
}

#[test]
#[should_panic = "unterminated multi-line comment"]
fn star_only_regex() {
    run_failure("/*/");
}

#[test]
fn leading_space_regex() {
    let js = r"/ \{[\s\S]*$/";
    compare(
        js,
        &[Token::RegEx(RegEx {
            body: r" \{[\s\S]*$",
            flags: None,
        })],
    )
}

#[test]
#[should_panic]
fn var_escaped_cr() {
    let js = r"var\u000Dx;";
    run_failure(js);
}

#[test]
fn long_comment() {
    pretty_env_logger::formatted_builder()
        .is_test(true)
        .try_init()
        .ok();
    let inner = "\n* <!-- I should not be a unique comment -->\n*\n";
    let js = format!("/*{}*/", inner);
    compare(
        &js,
        &[Token::Comment(Comment {
            kind: ress::tokens::CommentKind::Multi,
            content: inner,
            tail_content: None,
        })],
    )
}

#[test]
fn regex_column() {
    compare_with_position(
        "'abc'.match(/abc/);",
        &[
            (Token::String(StringLit::single("abc", false)), 1, 1),
            (Token::Punct(Punct::Period), 1, 6),
            (Token::Ident("match".into()), 1, 7),
            (Token::Punct(Punct::OpenParen), 1, 12),
            (Token::RegEx(RegEx::from_parts("abc", None)), 1, 13),
            (Token::Punct(Punct::CloseParen), 1, 18),
            (Token::Punct(Punct::SemiColon), 1, 19),
        ],
    );
}

#[test]
fn regex_spaces() {
    let scanner = Scanner::new("var = / a /");
    let mut last_end = 0;
    for (i, item) in scanner.enumerate() {
        let item = item.unwrap();
        if item.token.is_eof() {
            break;
        }

        assert_eq!(
            1,
            item.location.start.column - last_end,
            "{} for {:?}",
            i,
            item
        );
        last_end = item.location.end.column;
    }
}

#[test]
fn nullish_coalescing_assignment() {
    let js = r"a??=b";
    compare(
        js,
        &[
            Token::Ident(Ident::from("a")),
            Token::Punct(Punct::DoubleQuestionMarkEqual),
            Token::Ident(Ident::from("b")),
            Token::EoF,
        ],
    )
}

#[test]
fn logical_or_assignment() {
    let js = r"a||=b";
    compare(
        js,
        &[
            Token::Ident(Ident::from("a")),
            Token::Punct(Punct::DoublePipeEqual),
            Token::Ident(Ident::from("b")),
            Token::EoF,
        ],
    )
}

#[test]
fn logical_and_assignment() {
    let js = r"a&&=b";
    compare(
        js,
        &[
            Token::Ident(Ident::from("a")),
            Token::Punct(Punct::DoubleAmpersandEqual),
            Token::Ident(Ident::from("b")),
            Token::EoF,
        ],
    )
}

#[test]
fn nullish_coalescing() {
    let js = r#"a??b"#;
    compare(
        js,
        &[
            Token::Ident(Ident::from("a")),
            Token::Punct(Punct::DoubleQuestionMark),
            Token::Ident(Ident::from("b")),
            Token::EoF,
        ],
    )
}

#[test]
fn optional_chaining1() {
    let js = r#"a?.b"#;
    compare(
        js,
        &[
            Token::Ident(Ident::from("a")),
            Token::Punct(Punct::QuestionMarkDot),
            Token::Ident(Ident::from("b")),
            Token::EoF,
        ],
    )
}

#[test]
fn optional_chaining2() {
    let js = r#"a?.()"#;
    compare(
        js,
        &[
            Token::Ident(Ident::from("a")),
            Token::Punct(Punct::QuestionMarkDot),
            Token::Punct(Punct::OpenParen),
            Token::Punct(Punct::CloseParen),
            Token::EoF,
        ],
    )
}

#[test]
fn optional_chaining3() {
    let js = r#"a?.['b']"#;
    compare(
        js,
        &[
            Token::Ident(Ident::from("a")),
            Token::Punct(Punct::QuestionMarkDot),
            Token::Punct(Punct::OpenBracket),
            Token::String(Single(InnerString {
                content: "b",
                contains_octal_escape: false,
            })),
            Token::Punct(Punct::CloseBracket),
            Token::EoF,
        ],
    )
}

#[test]
fn optional_chaining4() {
    let js = r#"a==b?.123:.321"#;
    compare(
        js,
        &[
            Token::Ident(Ident::from("a")),
            Token::Punct(Punct::DoubleEqual),
            Token::Ident(Ident::from("b")),
            Token::Punct(Punct::QuestionMark),
            Token::Number(Number::from(".123")),
            Token::Punct(Punct::Colon),
            Token::Number(Number::from(".321")),
            Token::EoF,
        ],
    )
}

#[test]
fn regex_out_of_order() {
    pretty_env_logger::formatted_builder()
        .is_test(true)
        .try_init()
        .ok();
    let regex = r#"((?:[^BEGHLMOSWYZabcdhmswyz']+)|(?:'(?:[^']|'')*')|(?:G{1,5}|y{1,4}|Y{1,4}|M{1,5}|L{1,5}|w{1,2}|W{1}|d{1,2}|E{1,6}|c{1,6}|a{1,5}|b{1,5}|B{1,5}|h{1,2}|H{1,2}|m{1,2}|s{1,2}|S{1,3}|z{1,4}|Z{1,5}|O{1,4}))([\s\S]*)"#;
    let js = format!("var DATE_FORMATS_SPLIT = /{}/", &regex);
    compare_with_position(
        js.as_str(),
        &[
            (Token::Keyword(Keyword::Var("var")), 1, 1),
            (Token::Ident("DATE_FORMATS_SPLIT".into()), 1, 5),
            (Token::Punct(Punct::Equal), 1, 24),
            (Token::RegEx(RegEx::from_parts(regex, None)), 1, 26),
        ],
    );
}

#[test]
fn regex_pattern() {
    pretty_env_logger::formatted_builder()
        .is_test(true)
        .try_init()
        .ok();
    let re = r#" \{[\s\S]*$"#;
    let js = format!("/{re}/");

    let mut scanner = Scanner::new(&js);
    let Item {
        location,
        token: Token::RegEx(re2),
        ..
    } = scanner.next().unwrap().unwrap()
    else {
        panic!("Expected regex");
    };
    assert_eq!(location.start.line, 1);
    assert_eq!(location.end.line, 1);
    assert_eq!(location.start.column, 1);
    assert_eq!(re2.body, re);
    assert_eq!(location.end.column, re.len() + 3);
}

#[test]
fn regex_over_a0() {
    let js = r#"val = / /"#;
    compare(
        js,
        &[
            Token::Ident("val".into()),
            Token::Punct(Punct::Equal),
            Token::RegEx(RegEx {
                body: "\u{a0}",
                flags: None,
            }),
        ],
    )
}

#[test]
fn regex_over_a0_manual() {
    use ress::ManualScanner;
    let js = r#"val = / /"#;
    let mut scanner = ManualScanner::new(js);
    assert_eq!(
        scanner.next_token().unwrap().unwrap().token,
        Token::Ident("val".into())
    );
    assert_eq!(
        scanner.next_token().unwrap().unwrap().token,
        Token::Punct(Punct::Equal)
    );
    assert_eq!(
        scanner.next_token().unwrap().unwrap().token,
        Token::Punct(Punct::ForwardSlash)
    );
    assert_eq!(
        scanner.next_regex(1).unwrap().unwrap().token,
        Token::RegEx(RegEx {
            body: "\u{a0}",
            flags: None
        })
    );
}

#[test]
fn regex_all_whitespaces() {
    let re: String = [
        '\t', '\u{000b}', '\u{000c}', ' ', '\u{feff}', '\u{2000}', '\u{2001}', '\u{2002}',
        '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}',
        '\u{200a}', '\u{202f}', '\u{205f}', '\u{3000}',
    ]
    .iter()
    .collect();
    run_failure(&format!("var = /{re}/"));
}

#[track_caller]
fn compare(js: &str, expectation: &[Token<&str>]) {
    for (i, (par, ex)) in panicking_scanner(js).zip(expectation.iter()).enumerate() {
        assert_eq!((i, &par), (i, ex));
    }
}

fn compare_with_position(js: &str, expectation: &[(Token<&str>, usize, usize)]) {
    let scanner = Scanner::new(js);
    let mut i = 0;
    let mut expectation = expectation.iter();
    for r in scanner {
        let r = r.unwrap();
        if r.is_eof() {
            return;
        }
        i += 1;
        let ex = expectation
            .next()
            .ok_or_else(|| {
                panic!("expectations too short for {:?}", r);
            })
            .unwrap();
        assert_eq!((i, &r.token), (i, &ex.0), "{:?} vs {:?}", r, ex.0);
        assert_eq!(
            (i, r.location.start.line),
            (i, ex.1),
            "{:?} vs {:?}",
            r,
            ex.0
        );
        assert_eq!(
            (i, r.location.start.column),
            (i, ex.2),
            "{:?} vs {:?}",
            r,
            ex.0
        );
    }
}

fn run_failure(js: &str) {
    for _ in panicking_scanner(js) {}
}

fn panicking_scanner(js: &str) -> impl Iterator<Item = Token<&str>> {
    Scanner::new(js).map(|r| r.unwrap().token)
}
