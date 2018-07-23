//! This example is primarily for illustrating the
//! project's performance w/o waiting for the current
//! set of benches. It simply pulls down some major
//! JS libraries and attempts to tokenize them with
//! both methods and then reports the size, time and method
//! for each lib.
extern crate ress;
use std::{
    env::args,
    fs::read_to_string,
    path::PathBuf,
    time::{Duration, SystemTime},
};

fn main() {
    let mut i = 0;
    for arg in args() {
        i += 1;
        if arg == "jquery" || arg == "jq" {
            println!("trying jquery");
            jquery();
        } else if arg == "angular" || arg == "ng" {
            println!("trying angular1");
            angular1();
        } else if arg == "react" {
            println!("trying react");
            react();
        } else if arg == "react-dom" || arg == "rd" {
            println!("trying react_dom");
            react_dom();
        } else if arg == "vue" {
            println!("trying vue");
            vue();
        }
    }
    if i == 0 {
        println!("trying jquery");
        jquery();
        println!("trying angular1");
        angular1();
        println!("trying react");
        react();
        println!("trying react_dom");
        react_dom();
        println!("trying vue");
        vue();
    }
}

fn jquery() {
    if let Ok(ref js) = get_js(Lib::Jquery) {
        test_js(js, "jquery");
    }
}

fn angular1() {
    if let Ok(ref js) = get_js(Lib::Angular) {
        test_js(js, "angular");
    }
}

fn react() {
    if let Ok(ref js) = get_js(Lib::React) {
        test_js(js, "react");
    }
}

fn react_dom() {
    if let Ok(ref js) = get_js(Lib::ReactDom) {
        test_js(js, "react-dom");
    }
}

fn vue() {
    if let Ok(ref js) = get_js(Lib::Vue) {
        test_js(js, "vue");
    }
}

fn test_js(text: &str, name: &str) {
    let size = text.len();
    let now = SystemTime::now();
    let _ = ress::tokenize(text);
    if let Ok(e) = now.elapsed() {
        report(size, e, "tokenize", name)
    } else {
        println!("error capturing tokenize duration for {}", name);
    }

    let now = SystemTime::now();
    let s = ress::Scanner::new(text);
    let _: Vec<ress::Item> = s.collect();
    if let Ok(e) = now.elapsed() {
        report(size, e, "scanner", name)
    } else {
        println!("error capturing scanner duration for {}", name);
    }
}

fn report(bytes: usize, elapsed: Duration, method: &str, name: &str) {
    let size = get_size(bytes);
    println!("{} ({}) using {} in {}s {:.2}ms",
             name,
             size,
             method,
             elapsed.as_secs(),
             elapsed.subsec_millis())
}

fn get_size(b: usize) -> String {
    let mut size = b as f32;
    let mut i = 0;
    while size > 1000 as f32 {
        if i > 4 {
            break;
        }
        size = size / 1000.0;
        i += 1;
    }
    let bytes = match i {
        0 => "b",
        1 => "kb",
        2 => "mb",
        3 => "gb",
        _ => "tb",
    };
    format!("{:.2}{}", size, bytes)
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
