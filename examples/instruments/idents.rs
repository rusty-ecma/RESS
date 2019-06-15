// This example exists to allow for profiling
// applications to provide details about
// the criterion benchmarks
use ress::Tokenizer;

static IDENTS: &[&str] = &[
    r#"$"#,
    r#"_"#,
    r#"\u0078"#,
    r#"x$"#,
    r#"x_"#,
    r#"x\u0030"#,
    r#"xa"#,
    r#"x0"#,
    r#"x0a"#,
    r#"x0123456789"#,
    r#"qwertyuiopasdfghjklzxcvbnm"#,
    r#"QWERTYUIOPASDFGHJKLZXCVBNM"#,
    r#"œ一"#,
    r#"ǻ둘"#,
    r#"ɤ〩"#,
    r#"φ"#,
    r#"ﬁⅷ"#,
    r#"ユニコード"#,
    r#"x‌‍"#,
];

fn main() {
    for _ in 0..1000 {
        for i in IDENTS {
            let d = Tokenizer::new(i).next().unwrap();
            core::mem::forget(d);
        }
    }
}