use js_parser::{JsParser, Rule};
use pest::Parser;

#[test]
fn array_literal() {
    let expected_empty = vec![Rule::ArrayLiteral];
    let expected_one = vec![Rule::ArrayLiteral, Rule::ElementList, Rule::AssignmentExpression, ]
    super::test_many(Rule::ArrayLiteral, vec![
        "[]",
        "[1]",
        "['two', 'three']",
        "[/three/]",
    ]);
}

#[test]
fn object_literal() {
    super::test_many(Rule::PropertyAssignment, vec![
        "x: 1",
        "'y': 12",
        r#""x": "y""#,
        "z: 0,"
    ]);
    super::test_one(Rule::PropertyGetter, "get x() { return this._x }");
    super::test_one(Rule::PropertySetter, "set x(value) { this._x = value }");
    super::test_many(Rule::PropertyNameAndValueList, vec![
        "x: 'thing'",
        r#"y: "y", z: "z""#,
        "1: 2, 3: 4,",
        "name"
    ]);

    super::test_many(Rule::ObjectLiteral, vec![
        "{}",
        "{1: 2, 3: 4}",
        "{thing: 'stuff', people: 'places'}",
        "{a: thing, b: otherThing}",
        "{x(y) { return y; }, id: 1, name: 'person'}",
        "get x() { return this._x }, set x(value) { this._x = value}}",
    ]);
}

#[test]
fn numeric_literals() {
    super::test_many(Rule::NumericLiteral, vec![
        "0",
        "0.1",
        "1.1",
        "1.667e99",
        "1.556E9",
        "-100",
        "Infinity",
        "NaN",
    ]);
}