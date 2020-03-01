// This example exists to allow for profiling
// applications to provide details about
// the criterion benchmarks
use ress::Tokenizer;

fn main() {
    for _ in 0..1000 {
        let t = Tokenizer::new("true").next(true).unwrap();
        core::mem::forget(t);
        let f = Tokenizer::new("false").next(true).unwrap();
        core::mem::forget(f);
    }
}
