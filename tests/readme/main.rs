#![cfg(test)]

use ress::prelude::*;

#[test]
fn semi_example() {
    static JS: &str = include_str!("index.js");
    let s = Scanner::new(JS);
    for token in s {
        let token = token.unwrap().token;
        if token.matches_punct_str(";") {
            panic!("A semi-colon!? Heathen!");
        }
    }
    println!("Good show! Why use something that's optional?")
}

#[test]
#[allow(unused_variables)]
fn failed_compile_borrow() {
    // look_ahead
    let js = "function() { return; }";
    let mut s = Scanner::new(js);
    let current = s.next();
    let next = s.look_ahead();
    let new_current = s.next();
    assert_eq!(next, new_current);
    // peekable (fails to compile)
    let p = Scanner::new(js).peekable();
    let current = s.next(); // <-- first mutable borrow
                            // let next = p.peek(); // <-- second mutable borrow
}

#[test]
fn get_set_state() {
    let js = "function() {
    return 0;
};";
    let mut s = Scanner::new(js);
    let start = s.get_state();
    assert_eq!(
        s.next().unwrap().unwrap().token,
        Token::Keyword(Keyword::Function)
    );
    assert_eq!(
        s.next().unwrap().unwrap().token,
        Token::Punct(Punct::OpenParen)
    );
    assert_eq!(
        s.next().unwrap().unwrap().token,
        Token::Punct(Punct::CloseParen)
    );
    s.set_state(start);
    assert_eq!(
        s.next().unwrap().unwrap().token,
        Token::Keyword(Keyword::Function)
    );
}
