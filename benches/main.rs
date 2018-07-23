#![feature(test)]
#[macro_use]
extern crate criterion;

use criterion::Criterion;
extern crate ress;

use std::{fs::read_to_string, path::PathBuf};

fn jquery(c: &mut Criterion) {
    if let Ok(js) = get_js(Lib::Jquery) {
        c.bench_function("jquery", move |b| b.iter(|| ress::tokenize(&js)));
    }
}

fn angular1(c: &mut Criterion) {
    if let Ok(js) = get_js(Lib::Angular) {
        c.bench_function("angular1", move |b| b.iter(|| ress::tokenize(&js)));
    }
}

fn react(c: &mut Criterion) {
    if let Ok(js) = get_js(Lib::React) {
        c.bench_function("react", move |b| b.iter(|| ress::tokenize(&js)));
    }
}

fn react_dom(c: &mut Criterion) {
    if let Ok(js) = get_js(Lib::ReactDom) {
        c.bench_function("react_dom", move |b| b.iter(|| ress::tokenize(&js)));
    }
}

fn vue(c: &mut Criterion) {
    if let Ok(js) = get_js(Lib::Vue) {
        c.bench_function("vue", move |b| b.iter(|| ress::tokenize(&js)));
    }
}

criterion_group!(benches, jquery, angular1, react, react_dom, vue);
criterion_main!(benches);

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
