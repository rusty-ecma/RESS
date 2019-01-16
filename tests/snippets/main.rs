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

#[test]
fn template_middle_2() {
    let js = "let bitval = `(${prefix}.const ${format(val, i)})`";
    let expect = vec![
        Token::keyword("let"),
        Token::ident("bitval"),
        Token::punct("="),
        Token::template_head("("),
        Token::ident("prefix"),
        Token::template_middle(".const "),
        Token::ident("format"),
        Token::punct("("),
        Token::ident("val"),
        Token::punct(","),
        Token::ident("i"),
        Token::punct(")"),
        Token::template_tail(")"),
        Token::EoF,
    ];
    assert_eq!(expect, tokenize(js));
}

#[test]
fn deeply_nested_template() {
    let js = r#"`
 (func ${name} (param $barrierValue i32) (result i32)
   (local $n i32)
   (local $tmp ${prefix})
   (set_local $n (i32.const ${ITERATIONS}))
   (loop $outer
    (if (get_local $n)
        (block
         ${isMaster ? `;; Init
(${prefix}.atomic.store${tag} ${loc} (${prefix}.const ${distribute(initial)}))` : ``}
         ${barrier}

${(() => {
    let s = `;; Do\n`;
    for (let i=0; i < NUMVALS; i++) {
        let bitval = `(${prefix}.const ${format(val, i)})`
        // The load must be atomic though it would be better if it were relaxed,
        // we would avoid fences in that case.
        if (op.match(/cmpxchg/)) {
            s += `(loop $doit
                   (set_local $tmp (${prefix}.atomic.load${tag} ${loc}))
                   (br_if $doit (i32.eqz
                                 (${prefix}.eq
                                  (get_local $tmp)
                                  (${op} ${loc} (get_local $tmp) (${prefix}.or (get_local $tmp) ${bitval}))))))
            `;
        } else {
            s += `(drop (${op} ${loc} ${bitval}))
            `;
       }
     }
    return s
})()}
         (loop $wait_done
          (br_if $wait_done (${prefix}.ne (${prefix}.atomic.load${tag} ${loc}) (${prefix}.const ${distribute(expected)}))))
         ${barrier}
         (set_local $n (i32.sub (get_local $n) (i32.const 1)))
         (br $outer))))
  (get_local $barrierValue))`"#;
    let s = Scanner::new(js);
    for item in s {
        println!("{:?}", item);
    }
}