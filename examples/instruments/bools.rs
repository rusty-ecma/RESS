// This example exists to allow for profiling
// applications to provide details about
// the criterion benchmarks
use ress::Tokenizer;

fn main() {
    for _ in 0..1000 {
        let t = Tokenizer::new("true").next().unwrap();
        core::mem::forget(t);
        let f = Tokenizer::new("false").next().unwrap();
        core::mem::forget(f);
    }
}