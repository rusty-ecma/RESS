
extern crate js_parse;
use js_parse::tokenize;

#[test]
fn jquery() {
    let jq = ::std::fs::read_to_string("./tests/jquery.js").unwrap();
    let _ = tokenize(&jq);
}