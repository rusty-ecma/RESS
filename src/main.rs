extern crate js_parser;
extern crate pest;
use pest::Parser;
use js_parser::JsParser;
fn main() {
    let js = "\
    function(x) {
        let y = 0;
    }\
    ";
    js_parser::parse(js);
}
