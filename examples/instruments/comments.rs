#![allow(clippy::forget_non_drop)]
// This example exists to allow for profiling
// applications to provide details about
// the criterion benchmarks
use ress::Tokenizer;

static COMMENTS: &[&str] = &[
    "//this is a comment",
    "/*this is a
multi-line comment*/",
    "<!-- This is an HTML comment -->",
    "<!-- This is an HTML comment --> with a trailer",
];

fn main() {
    for _ in 0..1000 {
        for c in COMMENTS {
            let d = Tokenizer::new(c).next(true).unwrap();
            core::mem::forget(d);
        }
    }
}
