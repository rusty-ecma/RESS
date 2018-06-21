use js_parser::{JsParser, Rule};
use pest::Parser;
fn keywords() -> Vec<&'static str> {
    vec!["break",
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
    "with"]
}

fn future_keywords() -> Vec<&'static str> {
    vec![
        "abstract",
     "boolean",
     "byte",
     "char",
     "class",
     "const",
     "double",
     "enum",
     "export",
     "extends",
     "final",
     "float",
     "goto",
     "implements",
     "import",
     "interface",
     "int",
     "long",
     "native",
     "package",
     "private",
     "protected",
     "public",
     "short",
     "static",
     "super",
     "synchronized",
     "throws",
     "transient",
     "volatile"
    ]
}

#[test]
fn keyword_test() {
    super::test_many(Rule::Keyword, keywords());
}

#[test]
fn future_test() {
    super::test_many(Rule::FutureReservedWord, future_keywords());
}

#[test]
fn reserved_test() {
    super::test_many(Rule::ReservedWord, future_keywords().into_iter().chain(keywords().into_iter()).chain(vec!["null", "true", "false"].into_iter()).collect())
}