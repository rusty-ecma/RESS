extern crate js_parser;
extern crate pest;
use pest::Parser;
use js_parser::*;


fn test_many(rule: Rule, list: Vec<&str>) {
    for x in list {
        test_one(rule, x);
    }
}

fn test_one(rule: Rule, test: &str) {
    if let Ok(pair) = JsParser::parse(rule, test) {
        println!("matched {:?} to {:?}\npair: {:?}", test, rule, pair);
    }
}


mod tokens;
mod keywords;
mod literals;