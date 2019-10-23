//! This example is a quick and dirty example of
//! what someone might want to do with a JS token stream.
//! Essentially this is reading in the file and writing it out
//! with no comments. It successfully stripped all of the comments
//! out of a webpack output file though it cannot handle object literals
//! very well. It does a pretty good job of showing how you might use the Scanner.
extern crate docopt;
extern crate ress;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::{
    fs::{read_to_string, File},
    io::{BufWriter, Write},
    path::PathBuf,
    string::ToString,
};

use docopt::Docopt;

use ress::prelude::*;
type RefToken<'a> = Token<&'a str>;

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
    let s = Scanner::new(&js);
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
    for item in s {
        let item = item.unwrap();
        println!("{:?}", item);
        let token = item.token;
        if token.matches_keyword(Keyword::If(())) {
            in_if = true;
        }
        if in_if && token.matches_punct(Punct::OpenParen) {
            if_parens += 1;
        }
        if in_if && token.matches_punct(Punct::CloseParen) {
            if_parens -= 1;
        }
        if last_token.matches_keyword(Keyword::For(())) {
            in_loop = true;
        }
        if last_token.matches_keyword(Keyword::Case(())) || last_token.matches_keyword(Keyword::Default(()))
        {
            in_case = true;
        }
        if last_token.matches_punct(Punct::Colon) && in_case {
            new_line = true;
        }
        if in_loop && last_token.matches_punct(Punct::CloseParen) {
            in_loop = false;
        }
        if token.is_comment() {
            continue;
        }
        if last_token.matches_punct(Punct::OpenBrace) {
            indent += 1;
            new_line = true;
        }
        if in_if
            && if_parens == 0
            && last_token.matches_punct(Punct::CloseParen)
            && !token.is_punct()
        {
            unbraced_if = true;
            new_line = true;
            indent += 1;
        }
        if last_token.matches_punct(Punct::CloseParen) && !token.is_punct() {
            new_line = true;
        }
        if last_token.matches_punct(Punct::SemiColon) && !in_loop {
            new_line = true;
        }
        if last_token.matches_punct(Punct::CloseBrace) && !token.is_punct() {
            new_line = true;
        }
        if token.matches_punct(Punct::CloseBrace) {
            indent -= 1;
            new_line = !last_token.matches_punct(Punct::OpenBrace);
        }
        if last_token.is_comment() {
            new_line = true;
        }
        if new_line {
            out.write(format!("\n{}", "    ".repeat(indent)).as_bytes())
                .expect("error writing indent");
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
        out.write(&(token_to_string(&token)).as_bytes())
            .expect("Error writing token");
        last_token = token;
    }
}

fn space_before(last_token: &RefToken, token: &RefToken) -> bool {
    if last_token.matches_punct(Punct::Equal) || token.matches_punct(Punct::DoubleEqual) {
        return true;
    }
    if last_token.matches_punct(Punct::Period)
        && (token.is_ident() || token.matches_keyword(Keyword::This(())))
    {
        return false;
    }
    if (last_token.is_ident() || last_token.matches_keyword(Keyword::This(())))
        && token.matches_punct(Punct::Period)
    {
        return false;
    }
    if token.matches_keyword(Keyword::If(())) {
        return false;
    }
    if last_token.matches_keyword(Keyword::If(())) {
        return true;
    }
    if last_token.matches_keyword(Keyword::Return(())) && !token.is_punct() {
        return true;
    }
    if last_token.matches_keyword(Keyword::For(())) {
        return true;
    }
    if last_token.matches_keyword(Keyword::Switch(())) {
        return true;
    }
    if last_token.matches_punct(Punct::Colon) {
        return true;
    }
    if token.matches_keyword(Keyword::This(())) {
        return false;
    }
    if token.matches_punct(Punct::OpenParen) {
        return false;
    }
    if token.matches_punct(Punct::CloseParen) {
        return false;
    }
    if token.matches_punct(Punct::CloseBracket) {
        return false;
    }
    if token.matches_punct(Punct::OpenBracket) {
        return false;
    }
    if token.matches_punct(Punct::CloseBrace) {
        return false;
    }
    if last_token.matches_punct(Punct::OpenBrace) {
        return false;
    }
    if last_token.matches_punct(Punct::CloseBrace) {
        return false;
    }
    if last_token.matches_punct(Punct::CloseParen) && token.matches_punct(Punct::OpenBrace) {
        return true;
    }
    if last_token.matches_punct(Punct::OpenBracket) {
        return false;
    }
    if last_token.matches_punct(Punct::OpenParen) {
        return false;
    }
    if token.matches_punct(Punct::SemiColon) {
        return false;
    }
    if token.matches_punct(Punct::Period) {
        return false;
    }
    if last_token.matches_punct(Punct::Period) {
        return false;
    }
    if token.matches_punct(Punct::Comma) {
        return false;
    }
    if token.matches_punct(Punct::Colon) {
        return false;
    }
    if last_token.matches_punct(Punct::Bang) {
        return false;
    }
    if last_token.matches_punct(Punct::Comma) {
        return true;
    }
    if token.matches_punct(Punct::Bang) {
        return false;
    }
    if last_token.matches_keyword(Keyword::Function(())) && token.matches_punct(Punct::OpenBrace) {
        return false;
    }
    if last_token.matches_keyword(Keyword::In(()))
        || last_token.matches_ident_str("of")
        || last_token.matches_keyword(Keyword::For(()))
    {
        return true;
    }
    if token.matches_keyword(Keyword::In(())) || token.matches_ident_str("of") {
        return true;
    }
    if last_token.is_keyword() {
        return true;
    }
    if last_token.matches_punct(Punct::SemiColon) {
        return false;
    }
    if token.is_punct() || last_token.is_punct() {
        return true;
    }
    false
}

fn token_to_string(t: &RefToken) -> String {
    match t {
        &Token::Boolean(ref t) => if t == &Boolean::True { "true" } else { "false" }.to_string(),
        &Token::Comment(ref comment) => {
            if comment.is_multi_line() {
                format!("/*\n{}\n*/", comment.content)
            } else {
                format!("//{}", comment.content)
            }
        }
        &Token::Ident(ref name) => name.to_string(),
        &Token::Keyword(ref key) => key.to_string(),
        &Token::Null => "null".to_string(),
        &Token::Number(ref number) => number.to_string(),
        &Token::Punct(ref p) => p.to_string(),
        &Token::RegEx(ref regex) => match regex.flags {
            Some(ref f) => format!("/{}/{}", regex.body, f),
            None => format!("/{}/", regex.body),
        },
        &Token::String(ref s) => format!("{}", s.to_string()),
        _ => String::new(),
    }
}

#[derive(Deserialize)]
struct Opts {
    arg_in_path: PathBuf,
    arg_out_path: PathBuf,
}
