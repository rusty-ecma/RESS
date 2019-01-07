#![cfg(test)]
extern crate ress;

use ress::*;
#[test]
fn template_tail_error() {
    let js = "function getRawDirName (dir) {
  return dir.rawName || `${dir.name}.${Object.keys(dir.modifiers || {}).join('.')}`
}";
    let tokens = tokenize(js);
    assert_eq!(tokens, vec![
        Token::Keyword(Keyword::Function),
        Token::ident("getRawDirName"),
        Token::Punct(Punct::OpenParen),
        Token::ident("dir"),
        Token::Punct(Punct::CloseParen),
        Token::punct("{"),
        Token::keyword("return"),
        Token::ident("dir"),
        Token::punct("."),
        Token::ident("rawName"),
        Token::punct("||"),
        Token::template_head(""),
        Token::ident("dir"),
        Token::punct("."),
        Token::ident("name"),
        Token::template_middle("."),
        Token::ident("Object"),
        Token::punct("."),
        Token::ident("keys"),
        Token::punct("("),
        Token::ident("dir"),
        Token::punct("."),
        Token::ident("modifiers"),
        Token::punct("||"),
        Token::punct("{"),
        Token::punct("}"),
        Token::punct(")"),
        Token::punct("."),
        Token::ident("join"),
        Token::punct("("),
        Token::single_quoted_string("."),
        Token::punct(")"),
        Token::template_tail(""),
        Token::punct("}"),
        Token::EoF,
    ]);
}

#[test]
fn nested_templates() {
    let js = "`${dir.name}.${`${Object.keys(dir.modifiers || {}).join('.')}`}${{}}` + `${}`";
    let tokens = tokenize(js);
    assert_eq!(tokens, vec![
        Token::template_head(""),
        Token::ident("dir"),
        Token::punct("."),
        Token::ident("name"),
        Token::template_middle("."),
        Token::template_head(""),
        Token::ident("Object"),
        Token::punct("."),
        Token::ident("keys"),
        Token::punct("("),
        Token::ident("dir"),
        Token::punct("."),
        Token::ident("modifiers"),
        Token::punct("||"),
        Token::punct("{"),
        Token::punct("}"),
        Token::punct(")"),
        Token::punct("."),
        Token::ident("join"),
        Token::punct("("),
        Token::single_quoted_string("."),
        Token::punct(")"),
        Token::template_tail(""),
        Token::template_middle(""),
        Token::punct("{"),
        Token::punct("}"),
        Token::template_tail(""),
        Token::punct("+"),
        Token::template_head(""),
        Token::template_tail(""),
        Token::EoF,
    ]);
}