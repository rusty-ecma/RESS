use docopt::Docopt;
use ress::prelude::*;
#[macro_use]
extern crate serde_derive;

use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

static USAGE: &str = "
count-tokens

Usage:
    count-tokens <in-path>
";

#[derive(Deserialize)]
struct Opts {
    arg_in_path: PathBuf,
}

fn main() {
    let opts: Opts = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| {
            println!("error: {:?}", e);
            e.exit()
        });
    let js = read_to_string(opts.arg_in_path).expect("Failed to read file");
    let mut counts = get_initial_counts();

    for maybe in Scanner::new(&js) {
        let item = maybe.expect("failed to scan token");
        let key = token_type_str(&item.token);
        counts.entry(key).and_modify(|c| *c += 1);
    }
    for (key, value) in counts {
        println!("{}: {}", key, value);
    }
}

fn token_type_str(tok: &Token<&str>) -> &'static str {
    match tok {
        Token::Null => "null",
        Token::Boolean(_) => "bool",
        Token::Ident(_) => "ident",
        Token::Number(_) => "number",
        Token::String(_) => "string",
        Token::Keyword(_) => "keyword",
        Token::Punct(_) => "punct",
        Token::RegEx(_) => "regex",
        Token::Template(_) => "template",
        Token::Comment(_) => "comment",
        Token::HashbangComment(_) => "hashbang",
        Token::EoF => "eof",
    }
}

fn get_initial_counts() -> HashMap<&'static str, usize> {
    let mut counts = HashMap::new();
    counts.insert("regex", 0);
    counts.insert("ident", 0);
    counts.insert("template", 0);
    counts.insert("bool", 0);
    counts.insert("string", 0);
    counts.insert("number", 0);
    counts.insert("keyword", 0);
    counts.insert("punct", 0);
    counts.insert("comment", 0);
    counts.insert("null", 0);
    counts.insert("hashbang", 0);
    counts.insert("eof", 0);
    counts
}
