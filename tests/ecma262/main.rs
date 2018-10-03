#![cfg(test)]
extern crate ress;
extern crate env_logger;
use std::{
    path::Path,
    process::Command,
    fs::read_to_string,
};

use ress::Scanner;

#[test]
fn es5() {
    let _ = env_logger::try_init();
    println!("testing es5");
    let js = get_js(EsVersion::Es5);
    let _: Vec<_> = Scanner::new(js).collect();
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