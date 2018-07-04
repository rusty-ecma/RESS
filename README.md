# RESS
> Rust EcmaScript Scanner

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
    //do something with your list of tokens
}

```
The above method could be a bit expensive, since it will attempt to parse the whole js string all at once.

The other option is to create a `Scanner`, this will create a iterator of tokens parsing them one by one from your `String`.
```rust
extern crate ress;

use ress::{Scanner};

static &str JS = include_str!("index.js");

fn main() {
    let s = Scanner::new(JS);
    for token in s {
        // do something with this token
    }
}
```
> note: The overall cost for either method is going to be the same as `tokenize` is just creating a `Scanner` and then calling `collect` on it.

In either method the major construct that you would be dealing with is a `Token` enum. This enum represents the 10 different tokens defined in the ECMAScript specification, plus `Comments`.

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

In its current state it should be able to compile any valid <=ES6 JavaScript (I believe the testing is all currently done on ES3 packages though).

## Why?
Ideally this project will be the starting point for building a full JS AST in Rust. The next step would be to build a companion crate that will raise the tokens into an AST.

One of my major motivations to get started was to help streamline the generation and formatting of JS when working with `wasm-bindgen`, though it is not currently included there.

# Performance
I am sure there are a lot of low hanging fruit in this area, on my 13" MBP Late 2013 2.4 GHz Intel Core i5
8 GB 1600 MHz DDR3 laptop running `cargo +nightly bench` provides the following info.

|Lib|Size|Time|+/-|
|---|---|---|---|
|Angular 1.5.6|1.16mb|2.81s|9.23ms|
|jquery|271.75kb|1.53s|61.42ms|
|React|59.09kb|0.23s|17.31ms|
|React-dom|641.51kb|2.47s|10.61ms|
|Vue|289.30kb|1.69s|4.98ms|

If you are interested in getting an idea about performance without waiting for `cargo bench` to complete you can run the following command.

```sh
$ cargo run --example major_libs
```