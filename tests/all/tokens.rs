use js_parser::{JsParser, Rule};
use pest::Parser;

fn rule_token_pairs() -> Vec<(Rule, &'static str)> {
    vec![
        (Rule::FalseTok, "false"),
        (Rule::TrueTok, "true"),
        (Rule::NullTok, "null"),
        (Rule::BreakTok, "break"),
        (Rule::ContinueTok, "continue"),
        (Rule::DebuggerTok, "debugger"),
        (Rule::InTok, "in"),
        (Rule::InstanceOfTok, "instanceof"),
        (Rule::DeleteTok, "delete"),
        (Rule::FunctionTok, "function"),
        (Rule::NewTok, "new"),
        (Rule::ThisTok, "this"),
        (Rule::TypeofTok, "typeof"),
        (Rule::VoidTok, "void"),
        (Rule::IfTok, "if"),
        (Rule::ElseTok, "else"),
        (Rule::DoTok, "do"),
        (Rule::WhileTok, "while"),
        (Rule::ForTok, "for"),
        (Rule::VarTok, "var"),
        (Rule::ReturnTok, "return"),
        (Rule::CaseTok, "case"),
        (Rule::DefaultTok, "default"),
        (Rule::SwitchTok, "switch"),
        (Rule::ThrowTok, "throw"),
        (Rule::CatchTok, "catch"),
        (Rule::FinallyTok, "finally"),
        (Rule::TryTok, "try"),
        (Rule::WithTok, "with"),
    ]
}
#[test]
fn tokens() {
    let pairs = rule_token_pairs();
    for pair in pairs {
        super::test_one(pair.0, pair.1);
    }
}