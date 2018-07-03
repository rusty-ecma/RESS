extern crate js_parse;
use js_parse::tokenize;

#[test]
fn jquery() {
    let jq = ::std::fs::read_to_string("./tests/jquery.js").unwrap();
    let _ = tokenize(&jq);
}

#[test]
fn angular1() {
    let ng = ::std::fs::read_to_string("./tests/angular.js").unwrap();
    let _ = tokenize(&ng);
}