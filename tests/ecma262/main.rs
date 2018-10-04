#![cfg(test)]
extern crate ress;
extern crate pretty_env_logger;

use std::{
    path::Path,
    process::Command,
    fs::read_to_string,
};

use ress::{Scanner, Token, CommentKind,};

#[test]
fn es5() {
    println!("testing es5");
    ensure_logging();
    let js = get_js(EsVersion::Es5);
    run_test(&js);
}

#[test]
fn es2015_script() {
    println!("testing es2015 script");
    ensure_logging();
    let js = get_js(EsVersion::Es2015Script);
    // FIXME skipping first 2 lines to avoid the HTML style comment
    run_test(&js[112..]);
}

#[test]
fn es2015_module() {
    println!("testing es2015 module");
    ensure_logging();
    let js = get_js(EsVersion::Es2015Module);
    run_test(&js);
}

fn run_test(js: &str) {
    for item in Scanner::new(js) {
        match item.token {
            Token::Comment(c) => match c.kind {
                CommentKind::Single => eprintln!("----------\n{}\n----------", c.content),
                _ => ()
            },
            _ => ()
        }
    }
}

fn ensure_logging() {
    if let Err(_) = ::std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "debug");
    }
    let _ = pretty_env_logger::try_init();
}

enum EsVersion {
    Es5,
    Es2015Module,
    Es2015Script,
}

impl EsVersion {
    pub fn path(&self) -> String {
        format!("node_modules/everything.js/{}", match self {
            EsVersion::Es5 => "es5.js",
            EsVersion::Es2015Module => "es2015-module.js",
            EsVersion::Es2015Script => "es2015-script.js",
        })
    }
}

fn get_js(version: EsVersion) -> String {
    get_file(version.path())
}

fn get_file(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    if !path.exists() {
        npm_install();
        if !path.exists() {
            panic!("npm install failed to make {:?} available", path)
        }
    }
    read_to_string(path).expect(&format!("Failed to read {:?} to a string", path))
}

fn npm_install() {
    Command::new("npm").arg("install").output().expect("Failed to npm install");
}