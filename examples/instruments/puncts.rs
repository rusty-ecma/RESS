// This example exists to allow for profiling
// applications to provide details about
// the criterion benchmarks
use ress::Tokenizer;

static PUNCTS: &[&str] = &[
    "{", "}", "(", ")", ".", ";", ",", "[", "]", ":", "?", "~", ">", "<", "=", "!", "+", "-", "/",
    "*", "%", "&", "|", "^", ">>>=", //3 char
    "...", "===", "!==", ">>>", "<<=", ">>=", "**=", //2 char
    "&&", "||", "==", "!=", "+=", "-=", "*=", "/=", "++", "--", "<<", ">>", "&=", "|=", "^=", "%=",
    "<=", ">=", "=>", "**",
];

fn main() {
    for _ in 0..1000 {
        for punct in PUNCTS {
            let d = Tokenizer::new(punct).next().unwrap();
            core::mem::forget(d);
        }
    }
}