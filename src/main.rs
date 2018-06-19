extern crate js_parser;
extern crate pest;
use pest::Parser;
use js_parser::JsParser;
fn main() {
    let js = r#"function() {
    let x = 0;
}"#;
    js_parser::parse(js);
}
