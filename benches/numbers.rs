#![cfg(test)]
#![feature(test)]
extern crate combine;
extern crate ress;
extern crate test;

use combine::Parser;
use ress::numeric::literal as number;
use test::Bencher;
#[bench]
fn number_non_decimal(b: &mut Bencher) {
    b.iter(|| {
        number().parse("0x5541ff6").unwrap();
    });
}
