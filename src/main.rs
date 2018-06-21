extern crate js_parser;
// extern crate pest;

fn main() {
    let js = include_str!("../tests/index.js");
    js_parser::parse(js);
}
