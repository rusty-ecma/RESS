const keywords = {
    "do": "=> Some(RawToken::Keyword(Keyword::Do))",
    "if": "=> Some(RawToken::Keyword(Keyword::If))",
    "in": "=> Some(RawToken::Keyword(Keyword::In))",
    "for": "=> Some(RawToken::Keyword(Keyword::For))",
    "new": "=> Some(RawToken::Keyword(Keyword::New))",
    "try": "=> Some(RawToken::Keyword(Keyword::Try))",
    "var": "=> Some(RawToken::Keyword(Keyword::Var))",
    "let": "=> Some(RawToken::Keyword(Keyword::Let))",
    "case": "=> Some(RawToken::Keyword(Keyword::Case))",
    "this": "=> Some(RawToken::Keyword(Keyword::This))",
    "void": "=> Some(RawToken::Keyword(Keyword::Void))",
    "with": "=> Some(RawToken::Keyword(Keyword::With))",
    "enum": "=> Some(RawToken::Keyword(Keyword::Enum))",
    "else": "=> Some(RawToken::Keyword(Keyword::Else))",
    "true": "=> Some(RawToken::Boolean(true))",
    "null": "=> Some(RawToken::Null)",
    "await": "=> Some(RawToken::Keyword(Keyword::Await))",
    "break": "=> Some(RawToken::Keyword(Keyword::Break))",
    "catch": "=> Some(RawToken::Keyword(Keyword::Catch))",
    "class": "=> Some(RawToken::Keyword(Keyword::Class))",
    "const": "=> Some(RawToken::Keyword(Keyword::Const))",
    "throw": "=> Some(RawToken::Keyword(Keyword::Throw))",
    "while": "=> Some(RawToken::Keyword(Keyword::While))",
    "super": "=> Some(RawToken::Keyword(Keyword::Super))",
    "yield": "=> Some(RawToken::Keyword(Keyword::Yield))",
    "false": "=> Some(RawToken::Boolean(false))",
    "delete": "=> Some(RawToken::Keyword(Keyword::Delete))",
    "return": "=> Some(RawToken::Keyword(Keyword::Return))",
    "switch": "=> Some(RawToken::Keyword(Keyword::Switch))",
    "typeof": "=> Some(RawToken::Keyword(Keyword::TypeOf))",
    "export": "=> Some(RawToken::Keyword(Keyword::Export))",
    "import": "=> Some(RawToken::Keyword(Keyword::Import))",
    "static": "=> Some(RawToken::Keyword(Keyword::Static))",
    "public": "=> Some(RawToken::Keyword(Keyword::Public))",
    "default": "=> Some(RawToken::Keyword(Keyword::Default))",
    "finally": "=> Some(RawToken::Keyword(Keyword::Finally))",
    "package": "=> Some(RawToken::Keyword(Keyword::Package))",
    "private": "=> Some(RawToken::Keyword(Keyword::Private))",
    "continue": "=> Some(RawToken::Keyword(Keyword::Continue))",
    "debugger": "=> Some(RawToken::Keyword(Keyword::Debugger))",
    "function": "=> Some(RawToken::Keyword(Keyword::Function))",
    "interface": "=> Some(RawToken::Keyword(Keyword::Interface))",
    "protected": "=> Some(RawToken::Keyword(Keyword::Protected))",
    "instanceof": "=> Some(RawToken::Keyword(Keyword::InstanceOf))",
    "implements": "=> Some(RawToken::Keyword(Keyword::Implements))",
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
    let sig = `use super::tokens::RawToken;
use crate::tokens::Keyword;

pub fn check_complicated_keyword(ident: &[u8]) -> Option<RawToken> {
    match ident.len() {
        `;
    let matchBody = newTree.split('\n').map(line => {
        let size = parseInt(line.split(' ')[0]);
        return [size, line];
    }).sort((lhs, rhs) => lhs[0] - rhs[0])
        .map(pair => pair[1]).join('\n        ');
    let end = '_ => None,\n    }\n}\n'
    const fs = require('fs');
    fs.writeFile('./src/tokenizer/keyword_escape.rs', sig + matchBody + end, (e) => {
        if (e) {
            throw e
        }
        console.log('success');
    });
}

main();