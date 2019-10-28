const keywords = {
    "do": "=> Some(RawToken::Keyword(RawKeyword::Do))",
    "if": "=> Some(RawToken::Keyword(RawKeyword::If))",
    "in": "=> Some(RawToken::Keyword(RawKeyword::In))",
    "for": "=> Some(RawToken::Keyword(RawKeyword::For))",
    "new": "=> Some(RawToken::Keyword(RawKeyword::New))",
    "try": "=> Some(RawToken::Keyword(RawKeyword::Try))",
    "var": "=> Some(RawToken::Keyword(RawKeyword::Var))",
    "let": "=> Some(RawToken::Keyword(RawKeyword::Let))",
    "case": "=> Some(RawToken::Keyword(RawKeyword::Case))",
    "this": "=> Some(RawToken::Keyword(RawKeyword::This))",
    "void": "=> Some(RawToken::Keyword(RawKeyword::Void))",
    "with": "=> Some(RawToken::Keyword(RawKeyword::With))",
    "enum": "=> Some(RawToken::Keyword(RawKeyword::Enum))",
    "else": "=> Some(RawToken::Keyword(RawKeyword::Else))",
    "true": "=> Some(RawToken::Boolean(true))",
    "null": "=> Some(RawToken::Null)",
    "await": "=> Some(RawToken::Keyword(RawKeyword::Await))",
    "break": "=> Some(RawToken::Keyword(RawKeyword::Break))",
    "catch": "=> Some(RawToken::Keyword(RawKeyword::Catch))",
    "class": "=> Some(RawToken::Keyword(RawKeyword::Class))",
    "const": "=> Some(RawToken::Keyword(RawKeyword::Const))",
    "throw": "=> Some(RawToken::Keyword(RawKeyword::Throw))",
    "while": "=> Some(RawToken::Keyword(RawKeyword::While))",
    "super": "=> Some(RawToken::Keyword(RawKeyword::Super))",
    "yield": "=> Some(RawToken::Keyword(RawKeyword::Yield))",
    "false": "=> Some(RawToken::Boolean(false))",
    "delete": "=> Some(RawToken::Keyword(RawKeyword::Delete))",
    "return": "=> Some(RawToken::Keyword(RawKeyword::Return))",
    "switch": "=> Some(RawToken::Keyword(RawKeyword::Switch))",
    "typeof": "=> Some(RawToken::Keyword(RawKeyword::TypeOf))",
    "export": "=> Some(RawToken::Keyword(RawKeyword::Export))",
    "import": "=> Some(RawToken::Keyword(RawKeyword::Import))",
    "static": "=> Some(RawToken::Keyword(RawKeyword::Static))",
    "public": "=> Some(RawToken::Keyword(RawKeyword::Public))",
    "default": "=> Some(RawToken::Keyword(RawKeyword::Default))",
    "finally": "=> Some(RawToken::Keyword(RawKeyword::Finally))",
    "package": "=> Some(RawToken::Keyword(RawKeyword::Package))",
    "private": "=> Some(RawToken::Keyword(RawKeyword::Private))",
    "continue": "=> Some(RawToken::Keyword(RawKeyword::Continue))",
    "debugger": "=> Some(RawToken::Keyword(RawKeyword::Debugger))",
    "function": "=> Some(RawToken::Keyword(RawKeyword::Function))",
    "interface": "=> Some(RawToken::Keyword(RawKeyword::Interface))",
    "protected": "=> Some(RawToken::Keyword(RawKeyword::Protected))",
    "instanceof": "=> Some(RawToken::Keyword(RawKeyword::InstanceOf))",
    "implements": "=> Some(RawToken::Keyword(RawKeyword::Implements))",
};

/**
 * convert a character to its unicode escape string
 * @param {string} c Single character
 * @returns {string}
 */
function toEscapedUnicode(c) {
    let dig = c.charCodeAt(0);
    let num = `0000${dig.toString(16)}`.substr(-4);
    return `\\u${num}`;
}

function getPrefix(word) {
    if (word.length > 32) {
        return `${word.length} if &ident[0..32] == br"${word.substr(0,32)}" && &ident[32..] == br"${word.substr(32)}"`;
    }
    return `${word.length} if ident == br"${word}"`;
}

function toNewTest(key, escape, i, expr) {
    let test = '';
    for (let j = 0; j < key.length;j++) {
        if (j === i) { 
            test += escape;
        } else {
            test += key[j];
        }
    }
    return toFinalNewTest(test, expr);
}

function toFinalNewTest(test, expr) {
    let prefix = getPrefix(test);
    return `${prefix} ${expr},\n`
}

function main() {
    let newTree = '';
    for (let key in keywords) {
        let expr = keywords[key];
        let fullEscape = "";
        for (let i = 0; i < key.length;i++) {
            let escape = toEscapedUnicode(key[i]);
            fullEscape += escape;
            newTree += toNewTest(key, escape, i, expr);
        }
        
        newTree += toFinalNewTest(fullEscape, expr);
    }
    let sig = `use super::tokens::{RawToken, RawKeyword};

pub fn check_complicated_keyword(ident: &[u8]) -> Option<RawToken> {
    match ident.len() {
        `;
    let matchBody = newTree.split('\n').map(line => {
        let size = parseInt(line.split(' ')[0]);
        return [size, line];
    }).sort((lhs, rhs) => lhs[0] - rhs[0])
        .map(pair => pair[1]).join('\n        ');
    let end = '_ => None,\n    }\n}\n'
    let test = `
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_unicode_escaped_keyword() {
        let escaped_keywords = vec![
            (Some(RawToken::Keyword(RawKeyword::Yield)), r"\\u0079ield"),
            (Some(RawToken::Keyword(RawKeyword::Private)), r"privat\\u0065"),
            (Some(RawToken::Keyword(RawKeyword::Static)), r"\\u0073\\u0074\\u0061\\u0074\\u0069\\u0063"),
            (None, r"yield"),
        ];
        for (target, test) in escaped_keywords {
            assert_eq!(target, check_complicated_keyword(test.as_bytes()))
        }
    }
}`
    const fs = require('fs');
    fs.writeFile('./src/tokenizer/keyword_escape.rs', sig + matchBody + end + test, (e) => {
        if (e) {
            throw e
        }
        console.log('success');
    });
}

main();