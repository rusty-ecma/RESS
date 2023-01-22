#![allow(clippy::forget_non_drop)]
// This example exists to allow for profiling
// applications to provide details about
// the criterion benchmarks
use ress::Tokenizer;

static REGEX: &[&str] = &[
    r#"x/"#,
    r#"|/"#,
    r#"|||/"#,
    r#"^$\b\B/"#,
    r#"(?=(?!(?:(.))))/"#,
    r#"a.\f\n\r\t\v\0\[\-\/\\\x00\u0000/"#,
    r#"\d\D\s\S\w\W/"#,
    r#"\ca\cb\cc\cd\ce\cf\cg\ch\ci\cj\ck\cl\cm\cn\co\cp\cq\cr\cs\ct\cu\cv\cw\cx\cy\cz/"#,
    r#"\cA\cB\cC\cD\cE\cF\cG\cH\cI\cJ\cK\cL\cM\cN\cO\cP\cQ\cR\cS\cT\cU\cV\cW\cX\cY\cZ/"#,
    r#"[a-z-]/"#,
    r#"[^\b\-^]/"#,
    r#"[/\]\\]/"#,
    r#"./i"#,
    r#"./g"#,
    r#"./m"#,
    r#"./igm"#,
    r#".*/"#,
    r#".*?/"#,
    r#".+/"#,
    r#".+?/"#,
    r#".?/"#,
    r#".??/"#,
    r#".{0}/"#,
    r#".{0,}/"#,
    r#".{0,0}/"#,
];

fn main() {
    for _ in 0..1000 {
        for r in REGEX {
            let d = Tokenizer::new(r).next_regex(1).unwrap();
            core::mem::forget(d);
        }
    }
}
