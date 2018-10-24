#![cfg(test)]
#![feature(test)]
extern crate test;
extern crate ress;
extern crate combine;

use test::Bencher;
use ress::numeric::{literal as number};
use combine::Parser;
#[bench]
fn number_non_decimal(b: &mut Bencher) {
    b.iter(|| {
        number().parse("0x5541ff6").unwrap();
    });
}