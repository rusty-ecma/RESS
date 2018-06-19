extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate estree_rs;

use estree_rs::*;

use pest::{Parser, iterators::Pair};
const _GRAMMAR: &str = include_str!("js.pest");

#[derive(Parser)]
#[grammar = "js.pest"]
pub struct JsParser;

pub fn parse(js: &str)  {
    let x = JsParser::parse(Rule::Program, js).unwrap();
    for pair in x {
        recurse(pair, 0);
    }
}

pub fn recurse(pair: Pair<Rule>, i: usize) {
    let inner = pair.into_inner();
    for pair in inner {
        println!("{0}{1}\n{0}----------\n{0}{2:#?}", "    ".repeat(i), pair.as_str(), pair);
        recurse(pair, i + 1);
    }
}