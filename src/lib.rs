extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate estree_rs;

use estree_rs::*;

use pest::Parser;
const _GRAMMAR: &str = include_str!("js.pest");

#[derive(Parser)]
#[grammar = "js.pest"]
pub struct JsParser;

pub fn parse(js: &str)  {
    let x = JsParser::parse(Rule::keywords, js).unwrap();
    println!("{:#?}", &x);
}