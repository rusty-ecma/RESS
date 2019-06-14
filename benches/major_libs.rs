#![cfg(test)]
#![feature(test)]
extern crate ress;
extern crate test;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use ress::Scanner;
use std::fs::read_to_string;
use std::path::PathBuf;

lazy_static! {
    static ref NG: String = get_js(Lib::Angular).unwrap();
    static ref NG_MIN: String = get_min_js(Lib::Angular).unwrap();
    static ref JQ: String = get_js(Lib::Jquery).unwrap();
    static ref JQ_MIN: String = get_min_js(Lib::Jquery).unwrap();
    static ref REACT: String = get_js(Lib::React).unwrap();
    static ref REACT_MIN: String = get_min_js(Lib::React).unwrap();
    static ref REACT_DOM: String = get_js(Lib::ReactDom).unwrap();
    static ref REACT_DOM_MIN: String = get_min_js(Lib::ReactDom).unwrap();
    static ref VUE: String = get_js(Lib::Vue).unwrap();
    static ref VUE_MIN: String = get_min_js(Lib::Vue).unwrap();
    static ref EV5: String = get_js(Lib::EveryEs5).unwrap();
    static ref EV2015: String = get_js(Lib::EveryEs2015Script).unwrap();
    static ref EVMOD: String = get_js(Lib::EveryEs2015Mod).unwrap();
}

fn angular(c: &mut Criterion) {
    c.bench_function("angular", |b| {
        b.iter(|| {
            for i in Scanner::new(&NG) {
                black_box(i.unwrap());
            }
        })
    });
}

fn angular_min(c: &mut Criterion) {
    c.bench_function("angular_min", |b| {
        b.iter(|| {
            for i in Scanner::new(&NG_MIN) {
                black_box(i.unwrap());
            }
        })
    });
}

fn jq(c: &mut Criterion) {
    c.bench_function("jq", |b| {
        b.iter(|| {
            for i in Scanner::new(&JQ) {
                black_box(i.unwrap());
            }
        })
    });
}

fn jq_min(c: &mut Criterion) {
    c.bench_function("jq_min", |b| {
        b.iter(|| {
            for i in Scanner::new(&JQ_MIN) {
                black_box(i.unwrap());
            }
        })
    });
}

fn react(c: &mut Criterion) {
    c.bench_function("react", |b| {
        b.iter(|| {
            for i in Scanner::new(&REACT) {
                black_box(i.unwrap());
            }
        })
    });
}

fn react_min(c: &mut Criterion) {
    c.bench_function("react_min", |b| {
        b.iter(|| {
            for i in Scanner::new(&REACT_MIN) {
                black_box(i.unwrap());
            }
        })
    });
}

fn react_dom(c: &mut Criterion) {
    c.bench_function("react_dom", |b| {
        b.iter(|| {
            for i in Scanner::new(&REACT_DOM) {
                black_box(i.unwrap());
            }
        })
    });
}

fn react_dom_min(c: &mut Criterion) {
    c.bench_function("react_dom_min", |b| {
        b.iter(|| {
            for i in Scanner::new(&REACT_DOM_MIN) {
                black_box(i.unwrap());
            }
        })
    });
}

fn vue(c: &mut Criterion) {
    c.bench_function("vue", |b| {
        b.iter(|| {
            for i in Scanner::new(&VUE) {
                black_box(i.unwrap());
            }
        })
    });
}

fn vue_min(c: &mut Criterion) {
    c.bench_function("vue_min", |b| {
        b.iter(|| {
            for i in Scanner::new(&VUE_MIN) {
                black_box(i.unwrap());
            }
        })
    });
}

fn everything_es5(c: &mut Criterion) {
    c.bench_function("everything_es5", |b| {
        b.iter(|| {
            black_box(Scanner::new(&EV5).collect::<Vec<_>>());
        })
    });
}

fn everything_es2015_s(c: &mut Criterion) {
    c.bench_function("everything_es2015_s", |b| {
        b.iter(|| {
            black_box(Scanner::new(&EV2015).collect::<Vec<_>>());
        })
    });
}

fn everything_es2015_m(c: &mut Criterion) {
    c.bench_function("everything_es2015_m", |b| {
        b.iter(|| {
            for i in Scanner::new(&EVMOD) {
                black_box(i.unwrap());
            }
        })
    });
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
