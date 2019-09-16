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
            Token::Keyword(Keyword::If),
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

fn compare(js: &str, expectation: &[Token<&str>]) {
    for (i, (item, tok)) in Scanner::new(js).zip(expectation.iter()).enumerate() {
        let item = item.unwrap();
        assert_eq!((i, &item.token), (i, tok));
    }
}
