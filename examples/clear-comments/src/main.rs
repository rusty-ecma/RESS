//! This example is a quick and dirty example of
//! what someone might want to do with a JS token stream.
//! Essentially this is reading in the file and writing it out
//! with no comments. It successfully stripped all of the comments
//! out of a webpack output file though it cannot handle object literals
//! very well. It does a pretty good job of how you might use the Scanner.
extern crate docopt;
extern crate ress;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::{
    path::PathBuf,
    fs::{read_to_string, File},
    io::{Write, BufWriter},
};

use docopt::Docopt;

use ress::{Token, Scanner};

const USAGE: &'static str = "
clear-comments

Usage:
    clear-comments <in-path> <out-path>
";

fn main() {
    let opts: Opts = Docopt::new(USAGE)
                        .and_then(|d| d.deserialize())
                        .unwrap_or_else(|e| {
                            println!("error: {:?}", e);
                            e.exit()
                        });
    let js = if let Ok(s) = read_to_string(opts.arg_in_path) {
        s
    } else {
        eprintln!("Unable to read in-path");
        ::std::process::exit(1);
    };
    let s = Scanner::new(js);
    let mut indent = 0;
    let f = File::create(&opts.arg_out_path).expect("Error opening outfile");
    let mut out = BufWriter::new(f);
    let mut last_token = Token::EoF;
    let mut new_line = false;
    let mut in_loop = false;
    let mut in_case = false;
    let mut in_if = false;
    let mut if_parens = 0;
    let mut unbraced_if = false;
    for token in s {
        if last_token.is_keyword_with("if") {
            in_if = true;
        }
        if in_if && token.is_punct_with("(") {
            if_parens += 1;
        }
        if in_if && token.is_punct_with(")") {
            if_parens -= 1;
        }
        if last_token.is_keyword_with("for") {
            in_loop = true;
        }
        if last_token.is_keyword_with("case") || last_token.is_keyword_with("default") {
            in_case = true;
        }
        if last_token.is_punct_with(":") && in_case {
            new_line = true;
        }
        if in_loop && last_token.is_punct_with(")") {
            in_loop = false;
        }
        if token.is_comment() {
            continue;
        }
        if last_token.is_punct_with("{") {
            indent += 1;
            new_line = true;
        }
        if in_if && if_parens == 0 && last_token.is_punct_with(")") && !token.is_punct() {
            unbraced_if = true;
            new_line = true;
            indent += 1;
        }
        if last_token.is_punct_with(")") && !token.is_punct() {
            new_line = true;
        }
        if last_token.is_punct_with(";") && !in_loop {
            new_line = true;
        }
        if last_token.is_punct_with("}") && !token.is_punct() {
            new_line = true;
        }
        if token.is_punct_with("}") {
            indent -= 1;
            new_line = !last_token.is_punct_with("{");
        }
        if last_token.is_comment() {
            new_line = true;
        }
        if new_line {
            out.write(format!("\n{}","    ".repeat(indent)).as_bytes()).expect("error writing indent");
            new_line = false;
            in_if = false;
            if_parens = 0;
            if unbraced_if {
                indent -= 1;
                unbraced_if = false;
            }
        }

        if space_before(&last_token, &token) {
            out.write(" ".as_bytes()).expect("error writing space");
        }
        out.write(&(token_to_string(&token)).as_bytes()).expect("Error writing token");
        last_token = token;
    }
}

fn space_before(last_token: &Token, token: &Token) -> bool {
    if last_token.is_punct_with("=") || token.is_punct_with("=") {
        return true;
    }
    if last_token.is_punct_with(".") && (token.is_ident() || token.is_keyword_with("this")) {
        return false;
    }
    if (last_token.is_ident() || last_token.is_keyword_with("this")) && token.is_punct_with(".") {
        return false
    }
    if token.is_keyword_with("if") {
        return false;
    }
    if last_token.is_keyword_with("if") {
        return true;
    }
    if last_token.is_keyword_with("for") {
        return true;
    }
    if last_token.is_keyword_with("switch") {
        return true;
    }
    if last_token.is_punct_with(":") {
        return true;
    }
    if token.is_keyword_with("this") {
        return false;
    }
    if token.is_punct_with("(") {
        return false;
    }
    if token.is_punct_with(")") {
        return false;
    }
    if token.is_punct_with("]") {
        return false;
    }
    if token.is_punct_with("[") {
        return false;
    }
    if token.is_punct_with("}") {
        return false;
    }
    if last_token.is_punct_with("{") {
        return false;
    }
    if last_token.is_punct_with("}") {
        return false;
    }
    if last_token.is_punct_with(")") && token.is_punct_with("{") {
        return true;
    }
    if last_token.is_punct_with("[") {
        return false;
    }
    if last_token.is_punct_with("(") {
        return false;
    }
    if token.is_punct_with(";") {
        return false;
    }
    if token.is_punct_with(".") {
        return false;
    }
    if last_token.is_punct_with(".") {
        return false;
    }
    if token.is_punct_with(",") {
        return false;
    }
    if token.is_punct_with(":") {
        return false;
    }
    if last_token.is_punct_with("!") {
        return false;
    }
    if last_token.is_punct_with(",") {
        return true;
    }
    if token.is_punct_with("!") {
        return false;
    }
    if last_token.is_keyword_with("function") && token.is_punct_with("{") {
        return false;
    }
    if last_token.is_keyword_with("in") ||
        last_token.is_keyword_with("of") ||
        last_token.is_keyword_with("for") {
        return true;
    }
    if token.is_keyword_with("in") {
        return true;
    }
    if last_token.is_keyword() {
        return true;
    }
    if last_token.is_punct_with(";") {
        return false;
    }
    if token.is_punct() || last_token.is_punct() {
        return true;
    }
    false
}

fn token_to_string(t: &Token) -> String {
    match t {
        &Token::Boolean(ref t) => if *t {
            "true"
        } else {
            "false"
        }.to_string(),
        &Token::Comment(ref info) => if info.contains('\n') {
            format!("/*\n{}\n*/", info)
        } else {
            format!("//{}", info)
        },
        &Token::Ident(ref name) => name.to_string(),
        &Token::Keyword(ref key) => key.to_string(),
        &Token::Null => "null".to_string(),
        &Token::Numeric(ref number) => number.to_string(),
        &Token::Punct(ref c) => c.to_string(),
        &Token::RegEx(ref body, ref flags) => match flags {
            Some(ref f) => format!("/{}/{}", body, f),
            None => format!("/{}/", body),
        },
        &Token::String(ref s) => format!("'{}'", s),
        &Token::Template(ref tokens) => {
            let mut open = false;
            let mut ret = String::new();
            for token in tokens {
                if let Token::String(ref s) = token {
                    if open {
                        open = false;
                        ret.push_str(&format!("}}{}", s))
                    } else {
                        open = true;
                        ret.push_str(&format!("{}${{", s))
                    }
                } else {
                    ret.push_str(&token_to_string(token))
                }
            }
            ret
        },
        _ => String::new()
    }
}

#[derive(Deserialize)]
struct Opts {
    arg_in_path: PathBuf,
    arg_out_path: PathBuf,
}

