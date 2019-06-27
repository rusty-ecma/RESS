// This example exists to allow for profiling
// applications to provide details about
// the criterion benchmarks
use ress::Tokenizer;
static NUMBERS: &[&str] = &[
    "0",
    "00",
    "1234567890",
    "01234567",
    "0.",
    "0.00",
    "10.00",
    ".0",
    "0e0",
    "0E0",
    "0.e0",
    "0.00e+0",
    ".00e-0",
    "0x0",
    "0X0",
    "0x0123456789abcdefABCDEF",
    "0b0",
    "0b0100101",
    "0o0",
    "0o01234567",
    "2e308",
];

fn main() {
    for _ in 0..1000 {
        for n in NUMBERS {
            let d = Tokenizer::new(n).next().unwrap();
            core::mem::forget(d);
        }
    }
}
