#![allow(non_snake_case)]
extern crate pest;
#[macro_use]
extern crate pest_derive;
// extern crate estree_rs;
use pest::{Parser, iterators::Pair};
const _GRAMMAR: &str = include_str!("js.pest");

#[derive(Parser)]
#[grammar = "js.pest"]
pub struct JsParser;

pub fn parse(js: &str)  {
    let rules = vec![
        Rule::Program,
    ];
    for rule in rules {
        try_rule(rule, js);
    }
}
pub fn try_rule(rule: Rule, js: &str) {
    println!("trying: {:?}", &rule);
    match JsParser::parse(rule, js) {
        Ok(x) => {
            println!("Success:\n{}", &x);
            for pair in x {
                recurse(pair, 0);
            }
        },
        Err(e) => {
            println!("Error:\n{}", e);
            println!("Debug:\n{:?}", e);
        }
    }
}
pub fn recurse(pair: Pair<Rule>, i: usize) {
    println!("\n----------\n{:?}\n{}\n----------", pair.as_rule(), pair.as_str());
    for pair in pair.into_inner() {
        recurse(pair, i + 1);
    }
}