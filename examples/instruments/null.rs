#![allow(clippy::forget_non_drop)]
// This example exists to allow for profiling
// applications to provide details about
// the criterion benchmarks
use ress::Tokenizer;

fn main() {
    for _ in 0..1000 {
        let null = Tokenizer::new("null").next(true).unwrap();
        core::mem::forget(null);
    }
}
