# RESS
> Rusty EcmaScript Scanner

![travis](https://img.shields.io/travis/FreeMasen/RESS.svg)
![appveyor](https://img.shields.io/appveyor/ci/FreeMasen/RESS.svg)
![crates.io](https://img.shields.io/crates/v/ress.svg)
![last commit](https://img.shields.io/github/last-commit/FreeMasen/RESS.svg)

A scanner/tokenizer for JS written in Rust

## Usage
There are two interfaces for using `ress` in your Rust code.

The first is the very simple function `tokenize`, this takes in a `String` and outputs a `Vec<Token>`.

```rust
extern crate ress;

use ress::tokenize;

static &str JS = include_str!("index.js");

fn main() {
    let tokens = tokenize(JS);
    it !tokens.iter().any(|t| t.is_punct_with(";")) {
        panic!("No semi-colon!? You nave!");
    } else {
        println!("At least you are sane at one point");
    }
}

```

The other option is to create a `Scanner`, an iterator over the `Item` struct. `Item` has two fields `token` for the `Token` found and `Span` for the position in the string.
```rust
extern crate ress;

use ress::{Scanner};

static &str JS = include_str!("index.js");

fn main() {
    let s = Scanner::new(JS);
    for token in s {
        if token.is_punct_with(";") {
            panic!("A semi-colon!? Heathen!");
        }
    }
    println!("Good show! Why use something that's optional?")
}
```

In either method the major construct that you would be dealing with is a `Token` enum. This enum represents the 11 different tokens defined in the ECMAScript specification.

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
- Template
- Comment

In its current state it should be able to tokenize any valid JavaScript (I believe the testing is all currently done on ES3 packages). Keep in mind that keywords have been moving around a lot in JS between ES3 through ES2019 so you might find some items parsed as keywords in the ES2019 context that are not in the ES3 context and since my goal is keep this scanner not-context aware this should be dealt with at a higher level.

For each of the token cases there is either a struct or enum to provide additional information with the exception of `NullLiteral` and `EoF` which should be self explanatory. The more complicated items do implement `ToString` which should get you back to the original js text for that token. The `Token` enum also provides a number of helper functions for building that picture without pulling the inner data our of the enum. Using the `Punct` case as an example the helper functions look like this
```rust
fn is_punct(&self) -> bool;
fn matches_punct(&self, p: Punct) -> bool;
fn matches_punct_str(&self, s: &str) -> bool;
```
A similar set of functions are available for each case. Be aware that some `From<&str>` implementations panic if the wrong string is provided meaning these would also panic.
```rust
let p = Token::Punct(Keyword::This);
if p.matches_keyword_str("junk") {
    // panic!
}
if p.matches_keyword(Keyword::This) {
    // Don't panic!
}
```

## Why?
Wouldn't it be nice to write new JS development tools in Rust? The (clear-comments)[https://github.com/FreeMasen/RESS/blob/master/examples/clear-comments/src/main.rs] example is a proof of concept on how you might use this crate to do just that. This project will take in a JS file and output a version with all of the comments removed. An example of how you might see it in action is below (assuming you have a put file called in.js in the project root).
```sh
$ cargo run --example clear-comments -- ./in.js ./out.js
```

Ideally this project will be the starting point for building a full JS Abstract Syntax Tree (AST) in Rust. The next step would be to build a companion crate that will raise the tokens into a full (AST). And once we have an AST a program that will write out JS text from that ast, essentially coming full circle.

# Performance
I am sure there are a lot of low hanging fruit in this area.
The below stats are from running `cargo +nightly bench` on a 13" MBP Late 2013 with a 2.4GHz i5 and 8gb Ram.

|Lib           |Method   |Size     |Time  |+/-      |
|---           |---      |---      |---   |---      |
|Angular 1.5.6 |Scanner  |1.16mb   |2.86s |23.68ms  |
|              |tokenize |         |2.90s |255.72ms |
|jquery        |Scanner  |271.75kb |1.53s |152.55ms |
|              |tokenize |         |1.59s |88.92ms  |
|React         |Scanner  |59.09kb  |0.24s |56.96ms  |
|              |tokenize |59.09kb  |0.24s |27.83ms  |
|React-dom     |Scanner  |641.51kb |2.54s |267.29ms |
|              |tokenize |641.51kb |2.51s |279.62ms |
|Vue           |Scanner  |289.30kb |1.74s |238.30ms |
|              |tokenize |289.30kb |1.76s |48.03ms  |

If you are interested in getting an idea about performance without waiting for `cargo bench` to complete you can run the following command.

```sh
$ cargo run --example major_libs
```

# Contributing

[see contributing.md](https://github.com/FreeMasen/RESS/blob/master/CONTRIBUTING.md)