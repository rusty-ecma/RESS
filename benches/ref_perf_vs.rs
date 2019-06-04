#![cfg(test)]
#![feature(test)]
extern crate combine;
extern crate ress;
extern crate test;
#[macro_use]
extern crate lazy_static;

use combine::Parser;
use test::{black_box, Bencher};
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
    "*", "%", "&", "|", "^", ">>>=", //3 char
    "...", "===", "!==", ">>>", "<<=", ">>=", "**=", //2 char
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
    " and animals and minerals`",
    "`}`",
    " and animals and minerals`",
    " and places and people ${",
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
}

#[bench]
fn keywords(b: &mut Bencher) {
    b.iter(|| {
        for key in KEYWORDS {
            black_box(ress::keywords::literal().parse(*key).unwrap());
        }
    })
}

#[bench]
fn keywords_ref(b: &mut Bencher) {
    b.iter(|| {
        for key in KEYWORDS {
            black_box(ress::refs::keywords::literal().parse(*key).unwrap());
        }
    })
}

#[bench]
fn keywords_tok(b: &mut Bencher) {
    b.iter(|| {
        for key in KEYWORDS {
            black_box(ress::tokenizer::Tokenizer::new(key).next_());
        }
    })
}

#[bench]
fn punct(b: &mut Bencher) {
    b.iter(|| {
        for punct in PUNCTS {
            black_box(ress::punct::punctuation().parse(*punct).unwrap());
        }
    });
}

#[bench]
fn punct_ref(b: &mut Bencher) {
    b.iter(|| {
        for punct in PUNCTS {
            black_box(ress::refs::punct::punctuation().parse(*punct).unwrap());
        }
    });
}

#[bench]
fn punct_tok(b: &mut Bencher) {
    b.iter(|| {
        for punct in PUNCTS {
            black_box(ress::tokenizer::Tokenizer::new(punct).next_());
        }
    })
}

#[bench]
fn strings(b: &mut Bencher) {
    b.iter(|| {
        for s in STRINGS {
            black_box(ress::strings::literal().parse(*s).unwrap());
        }
    })
}

#[bench]
fn strings_ref(b: &mut Bencher) {
    b.iter(|| {
        for s in STRINGS {
            black_box(ress::refs::strings::literal().parse(*s).unwrap());
        }
    })
}

#[bench]
fn strings_tok(b: &mut Bencher) {
    b.iter(|| {
        for s in STRINGS {
            println!("{}", s);
            black_box(ress::tokenizer::Tokenizer::new(s).next_());
        }
    })
}

#[bench]
fn comments(b: &mut Bencher) {
    b.iter(|| {
        for c in COMMENTS {
            black_box(ress::comments::comment().parse(*c).unwrap());
        }
    })
}

#[bench]
fn comments_refs(b: &mut Bencher) {
    b.iter(|| {
        for c in COMMENTS {
            black_box(ress::refs::comments::comment().parse(*c).unwrap());
        }
    })
}

#[bench]
fn numbers(b: &mut Bencher) {
    b.iter(|| {
        for n in NUMBERS {
            black_box(ress::numeric::literal().parse(*n).unwrap());
        }
    })
}

#[bench]
fn numbers_ref(b: &mut Bencher) {
    b.iter(|| {
        for n in NUMBERS {
            black_box(ress::refs::numbers::literal().parse(*n).unwrap());
        }
    })
}

#[bench]
fn numbers_tok(b: &mut Bencher) {
    b.iter(|| {
        for n in NUMBERS {
            black_box(ress::tokenizer::Tokenizer::new(n).next_());
        }
    })
}

#[bench]
fn regex(b: &mut Bencher) {
    b.iter(|| {
        for r in REGEX {
            black_box(ress::regex::regex_tail().parse(*r).unwrap());
        }
    })
}

#[bench]
fn regex_ref(b: &mut Bencher) {
    b.iter(|| {
        for r in REGEX {
            black_box(ress::refs::regex::regex_tail().parse(*r).unwrap());
        }
    })
}

#[bench]
fn templates(b: &mut Bencher) {
    b.iter(|| {
        for t in TEMPLATE_CONTINUATIONS {
            black_box(ress::strings::template_continuation().parse(*t).unwrap());
        }
        for t in TEMPLATE_STARTS {
            black_box(ress::strings::template_start().parse(*t).unwrap());
        }
    })
}

#[bench]
fn templates_ref(b: &mut Bencher) {
    b.iter(|| {
        for t in TEMPLATE_CONTINUATIONS {
            black_box(
                ress::refs::strings::template_continuation()
                    .parse(*t)
                    .unwrap(),
            );
        }
        for t in TEMPLATE_STARTS {
            black_box(ress::refs::strings::template_start().parse(*t).unwrap());
        }
    })
}

#[bench]
fn bools(b: &mut Bencher) {
    b.iter(|| {
        for b in BOOLS {
            black_box(ress::tokens::boolean_literal().parse(*b).unwrap());
        }
    })
}

#[bench]
fn bools_ref(b: &mut Bencher) {
    b.iter(|| {
        for b in BOOLS {
            black_box(ress::refs::tokens::boolean_literal().parse(*b).unwrap());
        }
    })
}

#[bench]
fn null(b: &mut Bencher) {
    b.iter(|| {
        for n in NULL {
            black_box(ress::tokens::null_literal().parse(*n).unwrap());
        }
    });
}
#[bench]
fn null_ref(b: &mut Bencher) {
    b.iter(|| {
        for n in NULL {
            black_box(ress::refs::tokens::null_literal().parse(*n).unwrap());
        }
    });
}

#[bench]
fn idents(b: &mut Bencher) {
    b.iter(|| {
        for i in IDENTS {
            black_box(ress::tokens::ident().parse(*i).unwrap());
        }
    })
}

#[bench]
fn idents_ref(b: &mut Bencher) {
    b.iter(|| {
        for i in IDENTS {
            black_box(ress::refs::tokens::ident().parse(*i).unwrap());
        }
    })
}

#[bench]
fn idents_tok(b: &mut Bencher) {
    b.iter(|| {
        for i in IDENTS {
            black_box(ress::tokenizer::Tokenizer::new(i).next_());
        }
    })
}

#[bench]
pub fn token(b: &mut Bencher) {
    b.iter(|| {
        for t in TOKENS.iter() {
            black_box(ress::tokens::token().parse(*t).unwrap());
        }
    })
}

#[bench]
pub fn token_ref(b: &mut Bencher) {
    println!("[");
    b.iter(|| {
        for t in TOKENS.iter() {
            println!("{:?}", t);
            black_box(ress::refs::tokens::token().parse(*t).unwrap());
        }
        println!("];")
    })
}

#[bench]
fn scanner(b: &mut Bencher) {
    let js = include_str!("../node_modules/jquery/dist/jquery.js");
    use ress::{Item, Scanner};
    b.iter(|| {
        let s = Scanner::new(js);
        black_box(s.collect::<Vec<Item>>())
    });
}

#[bench]
fn scanner_ref(b: &mut Bencher) {
    let js = include_str!("../node_modules/jquery/dist/jquery.js");
    use ress::refs::{RefItem as Item, RefScanner as Scanner};
    b.iter(|| {
        let s = Scanner::new(js);
        black_box(s.collect::<Vec<Item>>())
    });
}
