#![cfg(test)]
#![feature(test)]
extern crate ress;
extern crate test;
use ress::refs::RefScanner as Scanner;
use test::{Bencher, black_box};
use std::fs::read_to_string;
use std::path::PathBuf;
#[bench]
fn angular(b: &mut Bencher) {
    let js = get_js(Lib::Angular).unwrap();
    b.iter(|| {
        black_box(Scanner::new(&js).collect::<Vec<_>>());
    });
}

#[bench]
fn angular_min(b: &mut Bencher) {
    let js = get_min_js(Lib::Angular).unwrap();
    b.iter(|| {
        black_box(Scanner::new(&js).collect::<Vec<_>>());
    })
}

#[bench]
fn jq(b: &mut Bencher) {
    let js = get_js(Lib::Jquery).unwrap();
    b.iter(|| {
        black_box(Scanner::new(&js).collect::<Vec<_>>());
    });
}

#[bench]
fn jq_min(b: &mut Bencher) {
    let js = get_min_js(Lib::Jquery).unwrap();
    b.iter(|| {
        black_box(Scanner::new(&js).collect::<Vec<_>>());
    });
}

#[bench]
fn react(b: &mut Bencher) {
    let js = get_js(Lib::React).unwrap();
    b.iter(|| {
        black_box(Scanner::new(&js).collect::<Vec<_>>());
    });
}

#[bench]
fn react_min(b: &mut Bencher) {
    let js = get_min_js(Lib::React).unwrap();
    b.iter(|| {
        black_box(Scanner::new(&js).collect::<Vec<_>>());
    });
}

#[bench]
fn react_dom(b: &mut Bencher) {
    let js = get_js(Lib::ReactDom).unwrap();
    b.iter(|| {
        black_box(Scanner::new(&js).collect::<Vec<_>>());
    });
}

#[bench]
fn react_dom_min(b: &mut Bencher) {
    let js = get_min_js(Lib::ReactDom).unwrap();
    b.iter(|| {
        black_box(Scanner::new(&js).collect::<Vec<_>>());
    });
}

#[bench]
fn vue(b: &mut Bencher) {
    let js = get_js(Lib::Vue).unwrap();
    b.iter(|| {
        black_box(Scanner::new(&js).collect::<Vec<_>>());
    });
}

#[bench]
fn vue_min(b: &mut Bencher) {
    let js = get_min_js(Lib::Vue).unwrap();
    b.iter(|| {
        black_box(Scanner::new(&js).collect::<Vec<_>>());
    });
}

enum Lib {
    Jquery,
    Angular,
    React,
    ReactDom,
    Vue,
}

fn get_js(l: Lib) -> Result<String, ::std::io::Error> {
    let path = PathBuf::from(l.path());
    if !path.exists() {
        npm_install();
        if !path.exists() {
            panic!("npm install failed to make {} available", path.display());
        }
    }
    read_to_string(path)
}

fn get_min_js(l: Lib) -> Result<String, ::std::io::Error> {
    let path = PathBuf::from(l.min_path());
    if !path.exists() {
        npm_install();
        if !path.exists() {
            panic!("npm install failed to make {} available", path.display());
        }
    }
    read_to_string(path)
}

impl Lib {
    pub fn path(&self) -> String {
        match self {
            &Lib::Jquery => "node_modules/jquery/dist/jquery.js".into(),
            &Lib::Angular => "node_modules/angular/angular.js".into(),
            &Lib::React => "node_modules/react/umd/react.development.js".into(),
            &Lib::ReactDom => "node_modules/react-dom/umd/react-dom.development.js".into(),
            &Lib::Vue => "node_modules/vue/dist/vue.js".into(),
        }
    }

    pub fn min_path(&self) -> String {
        match self {
            &Lib::Jquery => "node_modules/jquery/dist/jquery.min.js".into(),
            &Lib::Angular => "node_modules/angular/angular.min.js".into(),
            &Lib::React => "node_modules/react/umd/react.production.min.js".into(),
            &Lib::ReactDom => "node_modules/react-dom/umd/react-dom.production.min.js".into(),
            &Lib::Vue => "node_modules/vue/dist/vue.min.js".into(),
        }
    }
}


fn npm_install() {
    eprintln!("Downloading required js dependencies");
    let mut c = ::std::process::Command::new("npm");
    c.arg("i");
    let out = c.output().expect("Failed to read output from npm");
    if !out.status.success() {
        panic!("{}", format!("Failed to run npm i\n{:?}", String::from_utf8_lossy(&out.stderr)));
    }
}