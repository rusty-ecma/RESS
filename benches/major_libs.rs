#![cfg(test)]
#![feature(test)]
extern crate test;
extern crate ress;
use test::Bencher;
use ress::Scanner;
#[bench]
fn angular(b: &mut Bencher) {
    let js = include_str!("../node_modules/angular/angular.js");
    b.iter(|| {
        let s = Scanner::new(js);
        let _: Vec<ress::Item> = s.collect();
    })
}