# RESS
> Rusty EcmaScript Scanner

[![travis](https://img.shields.io/travis/FreeMasen/RESS.svg)](https://travis-ci.org/FreeMasen/RESS)
[![appveyor](https://img.shields.io/appveyor/ci/FreeMasen/RESS.svg)](https://ci.appveyor.com/project/FreeMasen/sitebuilder)
[![crates.io](https://img.shields.io/crates/v/ress.svg)](https://crates.io/crates/ress)
[![last commit master](https://img.shields.io/github/last-commit/FreeMasen/RESS.svg)](https://github.com/FreeMasen/RESS/commits/master)

A scanner/tokenizer for JS written in Rust

## Usage
The primary way to interact with ress is through the `Scanner` struct which implements `Iterator` over the `Item` struct. `Item` has three fields `token` for the `Token` found, `span` which represents the start and end of the byte position in the original string and `location` which represents start and end character position with a line and column. It's definition looks like this.

```rust
Item {
    token: Token::Punct(Punct::Bang),
    span: Span {
        start: 0,
        end: 1,
    },
    location: SourceLocation {
        start: Position {
            line: 1,
            column: 1,
        },
        end: Position {
            line: 1,
            column: 2,
        }
    }
}
```

Note: the EcmaScript spec allows for 4 new line characters, only two of which are normally rendered by modern text editors the location line numbers will count these unrendered lines.

Here is an example that check some JS text for the existence of a semicolon and panics if one is found.

```rust
extern crate ress;

use ress::{Scanner};

static JS: &str = include_str!("index.js");

fn main() {
    let s = Scanner::new(JS);
    for token in s {
        let token = token.unwrap().token;
        if token.matches_punct_str(";") {
            panic!("A semi-colon!? Heathen!");
        }
    }
    println!("Good show! Why use something that's optional?")
}
```
By far the most important part of `Item` is the `Token` enum, which will represent the 11 different types of token's supported by the [ECMAScript specification](https://tc39.es/ecma262/#sec-ecmascript-language-lexical-grammar).

### ES Tokens
- Boolean Literal
- End of File
- Identifier
- Keyword
- Null Literal
- Numeric Literal
- Punctuation
- String Literal
- Regular Expression Literal
- Template String
- Comment

Keep in mind that keywords have been moving around a lot in JS between ES3 through ES2019 so you might find some items parsed as keywords in the ES2019 context that are not in the ES3 context, this should be dealt with at a higher level. A good example of this is `yield` which is sometimes a keyword and sometimes an identifier, this package will always parse this as a Keyword. As of the writing of this readme `ress` supports all tokens in the [Stage 2 and Stage 3 ECMAScript Proposals](https://github.com/tc39/proposals) with the exception of the `#!` comments.

For each of the token cases there is either a struct or enum to provide additional information with the exception of `NullLiteral` and `EoF` which should be self explanatory. The more complicated items do implement `ToString` which should get you back to the original js text for that token. The `Token` enum also provides a number of helper functions for building that picture without pulling the inner data our of the enum. Using the `Punct` case as an example the helper functions look like this.

```rust
fn is_punct(&self) -> bool;
fn matches_punct(&self, p: Punct) -> bool;
fn matches_punct_str(&self, s: &str) -> bool;
```
A similar set of functions are available for each case.

Like all `Iterators` the `Scanner` has a `next` method, It also has a `look_ahead` method that will allow you to parse the next value without advancing. Using this method can be a convenient way to get the next token without performing a mutable borrow, however you will be incurring the cost of parsing that token twice. All `Iterators` can be converted into a `Peekable` Iterator with a `peek` method, this will allow you to look ahead while only paying the cost once however `peek` performs a mutable borrow which means it needs to be in a different scope than a call to `next`.
```rust
// look_ahead
let js = "function() { return; }";
let mut s = Scanner::new(js);
let current = s.next();
let next = s.look_ahead();
let new_current = s.next();
assert_eq!(next, new_current);
// peekable (fails to compile)
let p = Scanner::new(js).peekable();
let current = s.next(); // <-- first mutable borrow
let next = p.peek(); // <-- second mutable borrow
```

For more intense lookahead scenarios `Scanner` makes available the `get_state` and `set_state` methods. These methods will allow you to capture a snapshot of the current position and any context, and then later reset to that position and context.

```rust
let js = "function() {
    return 0;
};";
let mut s = Scanner::new(js);
let start = s.get_state();
assert_eq!(s.next().unwrap().unwrap().token, Token::Keyword(Keyword::Function));
assert_eq!(s.next().unwrap().unwrap().token, Token::Punct(Punct::OpenParen));
assert_eq!(s.next().unwrap().unwrap().token, Token::Punct(Punct::CloseParen));
s.set_state(start);
assert_eq!(s.next().unwrap().unwrap().token, Token::Keyword(Keyword::Function));
```


## Why?
Wouldn't it be nice to write new JS development tools in Rust? The (clear-comments)[https://github.com/FreeMasen/RESS/blob/master/examples/clear-comments/src/main.rs] example is a proof of concept on how you might use this crate to do just that. This example will take in a JS file and output a version with all of the comments removed. An example of how you might see it in action is below (assuming you have a file called in.js in the project root).

```sh
$ cargo run --example clear-comments -- ./in.js ./out.js
```

# Performance
The below stats are from running `cargo +nightly bench` on a MBP (2.9 GHz i9-8850H & 16bg RAM).

| Lib         | Size     | Time      | +/-        |
| ----------- | -------- | --------- | ---------- |
| Angular 1.5 |   1.16mb | 18.991 ms |   4.393 ms |
| jquery      | 271.75kb |  7.218 ms | 577.236 μs |
| React       |  59.09kb |  1.976 ms | 116.139 μs |
| React-dom   | 641.51kb | 16.880 ms |   3.614 ms |
| Vue         | 289.30kb |  9.675 ms |   1.402 ms |

If you are interested in getting an idea about performance without waiting for `cargo bench` to complete you can run the following command.

```sh
$ cargo run --example major_libs
```

# Contributing

[see contributing.md](https://github.com/FreeMasen/RESS/blob/master/CONTRIBUTING.md)