// This example exists to allow for profiling
// applications to provide details about
// the criterion benchmarks
use ress::Tokenizer;

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

fn main() {
    for _ in 0..1000 {
        for key in KEYWORDS {
            let d = Tokenizer::new(key).next().unwrap();
            core::mem::forget(d);
        }
    }
}