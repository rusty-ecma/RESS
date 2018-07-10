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
    string::ToString,
};

use docopt::Docopt;

use ress::{Token, TokenData, Scanner, BooleanLiteral, Keyword, Punct};

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
    let mut last_token = TokenData::EoF;
    let mut new_line = false;
    let mut in_loop = false;
    let mut in_case = false;
    let mut in_if = false;
    let mut if_parens = 0;
    let mut unbraced_if = false;
    for token in s {
        let data = token.data;
        if data.is_keyword_with("if") {
            in_if = true;
        }
        if in_if && data.is_punct_with("(") {
            if_parens += 1;
        }
        if in_if && data.is_punct_with(")") {
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
        if data.is_comment() {
            continue;
        }
        if last_token.is_punct_with("{") {
            indent += 1;
            new_line = true;
        }
        if in_if && if_parens == 0 && last_token.is_punct_with(")") && !data.is_punct() {
            unbraced_if = true;
            new_line = true;
            indent += 1;
        }
        if last_token.is_punct_with(")") && !data.is_punct() {
            new_line = true;
        }
        if last_token.is_punct_with(";") && !in_loop {
            new_line = true;
        }
        if last_token.is_punct_with("}") && !data.is_punct() {
            new_line = true;
        }
        if data.is_punct_with("}") {
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

        if space_before(&last_token, &data) {
            out.write(" ".as_bytes()).expect("error writing space");
        }
        out.write(&(token_to_string(&data)).as_bytes()).expect("Error writing token");
        last_token = data;
    }
}

fn space_before(last_token: &TokenData, token: &TokenData) -> bool {
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

fn token_to_string(t: &TokenData) -> String {
    match t {
        &TokenData::Boolean(ref t) => if t == &BooleanLiteral::True {
            "true"
        } else {
            "false"
        }.to_string(),
        &TokenData::Comment(ref comment) => if comment.is_multi_line() {
            format!("/*\n{}\n*/", comment.content)
        } else {
            format!("//{}", comment.content)
        },
        &TokenData::Ident(ref name) => name.to_string(),
        &TokenData::Keyword(ref key) => key.to_string(),
        &TokenData::Null => "null".to_string(),
        &TokenData::Numeric(ref number) => {
            String::new()
        },
        &TokenData::Punct(ref c) => c.to_string(),
        &TokenData::RegEx(ref regex) => match regex.flags {
            Some(ref f) => format!("/{}/{}", regex.body, f),
            None => format!("/{}/", regex.body),
        },
        &TokenData::String(ref s) => format!("{}", s.into_simple()),
        &TokenData::Template(ref tokens) => {
            let mut open = false;
            let mut ret = String::from("`");
            for token in tokens {
                if let TokenData::String(ref s) = token {
                    if open {
                        open = false;
                        ret.push_str(&format!("}}{}", &s.content))
                    } else {
                        open = true;
                        ret.push_str(&format!("{}${{", &s.content))
                    }
                } else {
                    ret.push_str(&token_to_string(token))
                }
            }
            ret.push('`');
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