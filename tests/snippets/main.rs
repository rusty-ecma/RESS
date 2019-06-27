use ress::Scanner;

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
fn for_regex_error() {
    use ress::prelude::*;
    let expecatation = vec![
        Token::Keyword(Keyword::For),
        Token::Punct(Punct::OpenParen),
        Token::Number("1".into()),
        Token::Punct(Punct::CloseParen),
        Token::RegEx(RegEx {body: "a", flags: None}),
        Token::Punct(Punct::Period),
        Token::Ident("test".into()),
        Token::Punct(Punct::OpenParen),
        Token::String(StringLit::Single("a")),
        Token::Punct(Punct::CloseParen),
    ];
    for (i, (item, tok)) in Scanner::new("for(1) /a/.test('a')").zip(expecatation.iter()).enumerate() {
        let item = item.unwrap();
        assert_eq!((i, &item.token), (i, tok));
    }
}