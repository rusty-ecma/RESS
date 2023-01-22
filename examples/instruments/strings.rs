#![allow(clippy::forget_non_drop)]
// This example exists to allow for profiling
// applications to provide details about
// the criterion benchmarks
use ress::Tokenizer;

static STRINGS: &[&str] = &[
    r#""things and stuff""#,
    r#"'people and places'"#,
    r#""with and escaped \"""#,
    r#"'another escaped \''"#,
    r#""with a new \
line""#,
    r#"'another new line \
hahaha'"#,
    "\"sequence double quoted\\\r\nis hard\"",
    "'new line sequence\\\r\nmight be harder'",
];

fn main() {
    for _ in 0..1000 {
        for s in STRINGS {
            let d = Tokenizer::new(s).next(true).unwrap();
            core::mem::forget(d);
        }
    }
}
