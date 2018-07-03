#![feature(test)]

extern crate test;

use test::Bencher;
extern crate js_parse;
use js_parse::tokenize;

#[bench]
fn jquery(b: &mut Bencher) {
    let jq = ::std::fs::read_to_string("./tests/jquery.js").unwrap();
    let parsed = tokenize(&jq);
}
#[bench]
fn angular1(b: &mut Bencher) {
    let ng = ::std::fs::read_to_string("./tests/angular.js").unwrap();
    let parsed = tokenize(&ng);
}
