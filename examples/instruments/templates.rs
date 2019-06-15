// This example exists to allow for profiling
// applications to provide details about
// the criterion benchmarks
use ress::Tokenizer;

static TEMPLATE_STARTS: &[&str] = &[
    "`things and stuff times ${",
    "`things and stuff`",
    r#"`a\${b`"#,
    r#"`\0\n\x0A\u000A\u{A}${"#,
];

static TEMPLATE_CONTINUATIONS: &[&str] = &[
    "`${} and animals and minerals`",
    "`${}`",
    "`${} and animals and minerals`",
    "`${} and places and people ${",
];

fn main() {
    for _ in 0..1000 {
        for s in TEMPLATE_CONTINUATIONS {
           parse_two(s);
        }
        for s in TEMPLATE_STARTS {
            parse(s);
        }
    }
}
#[inline]
fn parse_two(s: &str) {
    let mut t = Tokenizer::new(&s);
    let _ = t.next().unwrap();;
    let d = t.next().unwrap();
    core::mem::forget(d);
} 
#[inline]
fn parse(s: &str) {
    let e = Tokenizer::new(s).next().unwrap();
    core::mem::forget(e);
}