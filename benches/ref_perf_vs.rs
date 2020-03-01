#![cfg(test)]
#![feature(test)]
extern crate ress;
extern crate test;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate criterion;

use criterion::{black_box, Criterion};
use ress::{Scanner, Tokenizer};

static KEYWORDS: &[&str] = &[
    "implements",
    "interface",
    "package",
    "private",
    "protected",
    "public",
    "static",
    "yield",
    "let",
    "enum",
    "export",
    "import",
    "super",
    "break",
    "case",
    "catch",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "finally",
    "for",
    "function",
    "if",
    "instanceof",
    "in",
    "new",
    "return",
    "switch",
    "this",
    "throw",
    "try",
    "typeof",
    "var",
    "void",
    "while",
    "with",
];
static PUNCTS: &[&str] = &[
    "{", "}", "(", ")", ".", ";", ",", "[", "]", ":", "?", "~", ">", "<", "=", "!", "+", "-", "/",
    "*", "%", "&", "|", "^", "#", "@", ">>>=", "...", "===", "!==", ">>>", "<<=", ">>=", "**=",
    "&&", "||", "==", "!=", "+=", "-=", "*=", "/=", "++", "--", "<<", ">>", "&=", "|=", "^=", "%=",
    "<=", ">=", "=>", "**",
];

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

static COMMENTS: &[&str] = &[
    "//this is a comment",
    "/*this is a
multi-line comment*/",
    "<!-- This is an HTML comment -->",
    "<!-- This is an HTML comment --> with a trailer",
];

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

static BOOLS: &[&str] = &["true", "false"];

static NULL: &[&str] = &["null"];

lazy_static! {
    static ref TOKENS: Vec<&'static str> = COMMENTS
        .into_iter()
        .chain(KEYWORDS.into_iter())
        .chain(NUMBERS.into_iter())
        .chain(PUNCTS.into_iter())
        .chain(IDENTS.into_iter())
        .chain(BOOLS.into_iter())
        .chain(NULL.into_iter())
        .chain(TEMPLATE_STARTS.into_iter())
        .map(|s| *s)
        .collect();
    static ref JS: String = TOKENS.join("\n");
}

fn keywords(c: &mut Criterion) {
    c.bench_function("keywords", |b| {
        b.iter(|| {
            for key in KEYWORDS {
                black_box(Tokenizer::new(key).next(true).unwrap());
            }
        })
    });
}

fn punct(c: &mut Criterion) {
    c.bench_function("punct", |b| {
        b.iter(|| {
            for punct in PUNCTS {
                black_box(Tokenizer::new(punct).next(true).unwrap());
            }
        })
    });
}

fn strings(c: &mut Criterion) {
    c.bench_function("strings", |b| {
        b.iter(|| {
            for s in STRINGS {
                black_box(Tokenizer::new(s).next(true).unwrap());
            }
        })
    });
}

fn comments(c: &mut Criterion) {
    c.bench_function("comments", |b| {
        b.iter(|| {
            for c in COMMENTS {
                black_box(Tokenizer::new(c).next(true).unwrap());
            }
        })
    });
}

fn numbers(c: &mut Criterion) {
    c.bench_function("numbers", |b| {
        b.iter(|| {
            for n in NUMBERS {
                black_box(Tokenizer::new(n).next(true).unwrap());
            }
        })
    });
}

fn regex(c: &mut Criterion) {
    c.bench_function("regex", |b| {
        b.iter(|| {
            for r in REGEX {
                black_box(Tokenizer::new(r).next_regex(1).unwrap());
            }
        })
    });
}

fn templates(c: &mut Criterion) {
    c.bench_function("TEMPLATE_CONTINUATIONS", |b| {
        b.iter(|| {
            for s in TEMPLATE_CONTINUATIONS {
                let mut t = Tokenizer::new(&s);
                let _ = t.next(true).unwrap();
                black_box(t.next(true).unwrap());
            }
        })
    });
    c.bench_function("TEMPLATE_STARTS", |b| {
        b.iter(|| {
            for s in TEMPLATE_STARTS {
                black_box(Tokenizer::new(s).next(true).unwrap());
            }
        })
    });
}

fn bools(c: &mut Criterion) {
    c.bench_function("bools", |b| {
        b.iter(|| {
            for b in BOOLS {
                black_box(Tokenizer::new(b).next(true).unwrap());
            }
        })
    });
}

fn null(c: &mut Criterion) {
    c.bench_function("null", |b| {
        b.iter(|| {
            for b in NULL {
                black_box(Tokenizer::new(b).next(true).unwrap());
            }
        })
    });
}

fn idents(c: &mut Criterion) {
    c.bench_function("idents", |b| {
        b.iter(|| {
            for i in IDENTS {
                black_box(Tokenizer::new(i).next(true).unwrap());
            }
        })
    });
}

pub fn token(c: &mut Criterion) {
    c.bench_function("token", |b| {
        b.iter(|| {
            for s in TOKENS.iter() {
                black_box(Tokenizer::new(s).next(true).unwrap());
            }
        })
    });
}

fn scanner(c: &mut Criterion) {
    c.bench_function("scanner", |b| {
        b.iter(|| {
            let s = Scanner::new(&JS);
            black_box(s.collect::<Vec<_>>())
        })
    });
}

criterion_group!(
    benches, punct, keywords, idents, strings, comments, numbers, regex, templates, bools, null,
    token, scanner
);
criterion_main!(benches);
