#![feature(test)]

extern crate test;

use test::Bencher;
extern crate ress;

use std::{fs::read_to_string, path::PathBuf};

#[bench]
fn jquery_tokenize(b: &mut Bencher) {
    if let Ok(ref js) = get_js(Lib::Jquery) {
        b.iter(|| ress::tokenize(js));
    }
}
#[bench]
fn jquery_scanner(b: &mut Bencher) {
    if let Ok(ref js) = get_js(Lib::Jquery) {
        b.iter(move || {
                   let s = ress::Scanner::new(js.as_str());
                   let _: Vec<ress::Item> = s.collect();
               });
    }
}

#[bench]
fn angular1_tokenize(b: &mut Bencher) {
    if let Ok(ref js) = get_js(Lib::Angular) {
        b.iter(|| ress::tokenize(js));
    }
}
#[bench]
fn angular1_scanner(b: &mut Bencher) {
    if let Ok(ref js) = get_js(Lib::Angular) {
        b.iter(move || {
                   let s = ress::Scanner::new(js.as_str());
                   let _: Vec<ress::Item> = s.collect();
               });
    }
}

#[bench]
fn react_tokenize(b: &mut Bencher) {
    if let Ok(ref js) = get_js(Lib::React) {
        b.iter(|| ress::tokenize(js));
    }
}
#[bench]
fn react_scanner(b: &mut Bencher) {
    if let Ok(ref js) = get_js(Lib::React) {
        b.iter(move || {
                   let s = ress::Scanner::new(js.as_str());
                   let _: Vec<ress::Item> = s.collect();
               });
    }
}

#[bench]
fn react_dom_tokenize(b: &mut Bencher) {
    if let Ok(ref js) = get_js(Lib::ReactDom) {
        b.iter(|| ress::tokenize(js));
    }
}
#[bench]
fn react_dom_scanner(b: &mut Bencher) {
    if let Ok(ref js) = get_js(Lib::ReactDom) {
        b.iter(move || {
                   let s = ress::Scanner::new(js.as_str());
                   let _: Vec<ress::Item> = s.collect();
               });
    }
}

#[bench]
fn vue_tokenize(b: &mut Bencher) {
    if let Ok(ref js) = get_js(Lib::Vue) {
        b.iter(|| ress::tokenize(js));
    }
}
#[bench]
fn vue_scanner(b: &mut Bencher) {
    if let Ok(ref js) = get_js(Lib::Vue) {
        b.iter(move || {
                   let s = ress::Scanner::new(js.as_str());
                   let _: Vec<ress::Item> = s.collect();
               });
    }
}

fn npm_install() -> Result<(), ::std::io::Error> {
    let mut c = ::std::process::Command::new("npm");
    c.arg("i");
    c.output()?;
    Ok(())
}

enum Lib {
    Jquery,
    Angular,
    React,
    ReactDom,
    Vue,
}

impl Lib {
    fn path(&self) -> String {
        match self {
            &Lib::Jquery => "node_modules/jquery/dist/jquery.js".into(),
            &Lib::Angular => "node_modules/angular/angular.js".into(),
            &Lib::React => "node_modules/react/umd/react.development.js".into(),
            &Lib::ReactDom => "node_modules/react-dom/umd/react-dom.development.js".into(),
            &Lib::Vue => "node_modules/vue/dist/vue.js".into(),
        }
    }
}

fn get_js(l: Lib) -> Result<String, ::std::io::Error> {
    let path = PathBuf::from(l.path());
    if !path.exists() {
        npm_install()?;
    }
    read_to_string(path)
}
