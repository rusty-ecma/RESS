#![cfg(test)]
#![feature(test)]
extern crate ress;
extern crate test;

#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use ress::Scanner;
use std::fs::read_to_string;
use std::path::PathBuf;

fn angular(c: &mut Criterion) {
    run_bench(c, Lib::Angular, "angular", false);
}

fn angular_min(c: &mut Criterion) {
    run_bench(c, Lib::Angular, "angular_min", true);
}

fn jq(c: &mut Criterion) {
    run_bench(c, Lib::Jquery, "jq", false);
}

fn jq_min(c: &mut Criterion) {
    run_bench(c, Lib::Jquery, "jq_min", true);
}

fn react(c: &mut Criterion) {
    run_bench(c, Lib::React, "react", false);
}

fn react_min(c: &mut Criterion) {
    run_bench(c, Lib::React, "react_min", true);
}

fn react_dom(c: &mut Criterion) {
    run_bench(c, Lib::ReactDom, "react_dom", false);
}

fn react_dom_min(c: &mut Criterion) {
    run_bench(c, Lib::ReactDom, "react_dom_min", true);
}

fn vue(c: &mut Criterion) {
    run_bench(c, Lib::Vue, "vue", false);
}

fn vue_min(c: &mut Criterion) {
    run_bench(c, Lib::Vue, "vue_min", true);
}

fn everything_es5(c: &mut Criterion) {
    run_bench(c, Lib::EveryEs5, "everything_es5", false);
}

fn everything_es2015_s(c: &mut Criterion) {
    run_bench(c, Lib::EveryEs2015Script, "everything_es2015_s", false);
}

fn everything_es2015_m(c: &mut Criterion) {
    run_bench(c, Lib::EveryEs2015Mod, "everything_es2015_m", false);
}

enum Lib {
    Jquery,
    Angular,
    React,
    ReactDom,
    Vue,
    EveryEs5,
    EveryEs2015Script,
    EveryEs2015Mod,
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
            Lib::Jquery => "node_modules/jquery/dist/jquery.js",
            Lib::Angular => "node_modules/angular/angular.js",
            Lib::React => "node_modules/react/umd/react.development.js",
            Lib::ReactDom => "node_modules/react-dom/umd/react-dom.development.js",
            Lib::Vue => "node_modules/vue/dist/vue.js",
            Lib::EveryEs5 => "node_modules/everything.js/es5.js",
            Lib::EveryEs2015Script => "node_modules/everything.js/es2015-script.js",
            Lib::EveryEs2015Mod => "node_modules/everything.js/es2015-module.js",
        }
        .into()
    }

    pub fn min_path(&self) -> String {
        match self {
            &Lib::Jquery => "node_modules/jquery/dist/jquery.min.js".into(),
            &Lib::Angular => "node_modules/angular/angular.min.js".into(),
            &Lib::React => "node_modules/react/umd/react.production.min.js".into(),
            &Lib::ReactDom => "node_modules/react-dom/umd/react-dom.production.min.js".into(),
            &Lib::Vue => "node_modules/vue/dist/vue.min.js".into(),
            _ => String::new(),
        }
    }
}

fn npm_install() {
    eprintln!("Downloading required js dependencies");
    let mut c = ::std::process::Command::new("npm");
    c.arg("i");
    let out = c.output().expect("Failed to read output from npm");
    if !out.status.success() {
        panic!(
            "{}",
            format!(
                "Failed to run npm i\n{:?}",
                String::from_utf8_lossy(&out.stderr)
            )
        );
    }
}

#[inline(always)]
fn run_bench(c: &mut Criterion, lib: Lib, name: &str, min: bool) {
    let js = if min {
        get_min_js(lib).unwrap()
    } else {
        get_js(lib).unwrap()
    };
    run_bench_(c, &js, name)
}

#[inline(always)]
fn run_bench_(c: &mut Criterion, js: &str, name: &str) {
    let mut group = c.benchmark_group(name);
    group.throughput(criterion::Throughput::Bytes(js.len() as u64));
    group.bench_function(name, |b| {
        b.iter(|| {
            for i in Scanner::new(&js) {
                black_box(i.unwrap());
            }
        })
    });
    group.finish();
}

criterion_group!(
    benches,
    angular,
    angular_min,
    jq,
    jq_min,
    react,
    react_min,
    react_dom,
    react_dom_min,
    vue,
    vue_min,
    everything_es5,
    everything_es2015_s,
    everything_es2015_m
);
criterion_main!(benches);
