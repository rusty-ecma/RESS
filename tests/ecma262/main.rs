#![cfg(test)]
extern crate pretty_env_logger;
extern crate ress;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use std::{fs::read_to_string, path::Path, process::Command};

use ress::Scanner;
mod es2015m;
mod es2015s;
mod es5;

#[test]
fn es5_test() {
    println!("testing es5");
    ensure_logging();
    let js = get_js(EsVersion::Es5);
    for (i, (lhs, rhs)) in Scanner::new(&js).zip(es5::ES5.iter()).enumerate() {
        let lhs = lhs.unwrap();
        debug!("{:?}:{:?}", lhs.token, rhs);
        assert_eq!((i, &lhs.token), (i, rhs));
    }
}

#[test]
fn es2015_script_test() {
    println!("testing es2015 script");
    ensure_logging();
    let js = get_js(EsVersion::Es2015Script);
    for (i, (lhs, rhs)) in Scanner::new(&js).zip(es2015s::TOKENS.iter()).enumerate() {
        let lhs = lhs.unwrap();
        debug!("{:?}:{:?}", lhs.token, rhs);
        assert_eq!((i, &lhs.token), (i, rhs));
    }
}

#[test]
fn es2015_module_test() {
    ensure_logging();
    debug!("testing es2015 module");
    let js = get_js(EsVersion::Es2015Module);
    for (i, (lhs, rhs)) in Scanner::new(&js).zip(es2015m::TOKENS.iter()).enumerate() {
        let lhs = lhs.unwrap();
        debug!("{:?}:{:?}", lhs.token, rhs);
        assert_eq!((i, &lhs.token), (i, rhs));
    }
}

fn ensure_logging() {
    let _ = pretty_env_logger::try_init();
}

enum EsVersion {
    Es5,
    Es2015Module,
    Es2015Script,
}

impl EsVersion {
    pub fn path(&self) -> String {
        format!(
            "node_modules/everything.js/{}",
            match self {
                EsVersion::Es5 => "es5.js",
                EsVersion::Es2015Module => "es2015-module.js",
                EsVersion::Es2015Script => "es2015-script.js",
            }
        )
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
    Command::new("npm")
        .arg("install")
        .output()
        .expect("Failed to npm install");
}
