#![cfg(test)]
extern crate pretty_env_logger;
extern crate ress;
#[macro_use]
extern crate log;

use std::{fs::read_to_string, path::Path, process::Command};

use ress::{Scanner};

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
    run_test(&js);
}

#[test]
fn es2015_module() {
    ensure_logging();
    debug!("testing es2015 module");
    let js = get_js(EsVersion::Es2015Module);
    run_test(&js);
}

fn run_test(js: &str) {
    let mut s = Scanner::new(js);
    let mut i = 0;
    while let Some(item) = s.next() {
        debug!("{}, {:?}", i, item.token);
        i += 1;
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
