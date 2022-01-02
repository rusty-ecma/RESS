use ress::prelude::*;

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
        &vec![
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
        &vec![
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
    for tok in panicing_scanner(js) {
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
fn regex_over_div2() {
    let js = "function(){}/\\d/g;;";
    compare(
        js,
        &[
            Token::Keyword(Keyword::Function("function")),
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
fn regex_over_div4() {
    let _ = pretty_env_logger::try_init();
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
        &js,
        &[Token::RegEx(RegEx {
            body: r" \{[\s\S]*$".into(),
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
    let _ = pretty_env_logger::try_init();
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
fn regex_out_of_order() {
    pretty_env_logger::try_init().ok();
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

fn compare(js: &str, expectation: &[Token<&str>]) {
    for (i, (par, ex)) in panicing_scanner(js).zip(expectation.iter()).enumerate() {
        assert_eq!((i, &par), (i, ex));
    }
}

fn compare_with_position(js: &str, expectation: &[(Token<&str>, usize, usize)]) {
    let scanner = Scanner::new(js).map(|r| r.unwrap());
    for (i, (r, ex)) in scanner.zip(expectation.iter()).enumerate() {
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
    for _ in panicing_scanner(js) {}
}

fn panicing_scanner<'a>(js: &'a str) -> impl Iterator<Item = Token<&'a str>> {
    Scanner::new(js).map(|r| r.unwrap().token)
}
