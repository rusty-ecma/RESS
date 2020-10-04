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

struct Args {
    pub angular: bool,
    pub jquery: bool,
    pub react: bool,
    pub react_dom: bool,
    pub vue: bool,
    pub moment: bool,
    pub dexie: bool,
}

impl ::std::default::Default for Args {
    fn default() -> Args {
        Args {
            angular: false,
            jquery: false,
            react: false,
            react_dom: false,
            vue: false,
            moment: false,
            dexie: false,
        }
    }
}

impl Args {
    fn pristine(&self) -> bool {
        !self.angular
            && !self.jquery
            && !self.react
            && !self.react_dom
            && !self.vue
            && !self.moment
            && !self.dexie
    }
}

fn main() {
    let mut a = Args::default();
    // loop over the ags and check for
    // lib names. If they exist, run the test
    // and increment the counter
    for arg in args() {
        if arg == "jquery" || arg == "jq" {
            a.jquery = true;
        } else if arg == "angular" || arg == "ng" {
            a.angular = true;
        } else if arg == "react" {
            a.react = true;
        } else if arg == "react-dom" || arg == "rd" {
            a.react_dom = true;
        } else if arg == "vue" || arg == "v" {
            a.vue = true
        } else if arg == "moment" || arg == "mt" {
            a.moment = true;
        } else if arg == "dexie" || arg == "dx" {
            a.dexie = true;
        }
    }
    if a.jquery {
        jquery();
    }
    if a.angular {
        angular1();
    }
    if a.react {
        react();
    }
    if a.react_dom {
        react_dom();
    }
    if a.vue {
        vue();
    }
    if a.moment {
        moment();
    }
    if a.dexie {
        dexie();
    }
    if a.pristine() {
        jquery();
        angular1();
        react();
        react_dom();
        vue();
        moment();
        dexie();
    }
}

fn jquery() {
    println!("trying jquery");
    if let Ok(ref js) = get_js(Lib::Jquery) {
        test_js(js, "jquery");
    }
}

fn angular1() {
    println!("trying angular1");
    if let Ok(ref js) = get_js(Lib::Angular) {
        test_js(js, "angular");
    }
}

fn react() {
    println!("trying react");
    if let Ok(ref js) = get_js(Lib::React) {
        test_js(js, "react");
    }
}

fn react_dom() {
    println!("trying react_dom");
    if let Ok(ref js) = get_js(Lib::ReactDom) {
        test_js(js, "react-dom");
    }
}

fn vue() {
    println!("trying vue");
    if let Ok(ref js) = get_js(Lib::Vue) {
        test_js(js, "vue");
    }
}

fn moment() {
    println!("trying moment");
    if let Ok(ref js) = get_js(Lib::Moment) {
        test_js(js, "moment")
    }
}

fn dexie() {
    println!("trying dexie");
    if let Ok(ref js) = get_js(Lib::Dexie) {
        test_js(js, "dexie");
    }
}

fn test_js(text: &str, name: &str) {
    let size = text.len();
    let now = SystemTime::now();
    test(text);
    if let Ok(e) = now.elapsed() {
        report(size, e, "scanner", name)
    } else {
        println!("error capturing scanner duration for {}", name);
    }
}

fn test(text: &str) {
    let s = ress::Scanner::new(text);
    let _: Vec<_> = s.collect();
}

fn report(bytes: usize, elapsed: Duration, method: &str, name: &str) {
    let size = get_size(bytes);
    println!(
        "{} ({}) using {} in {}s {:.2}ms",
        name,
        size,
        method,
        elapsed.as_secs(),
        elapsed.subsec_millis()
    )
}

fn get_size(b: usize) -> String {
    let mut size = b as f32;
    let mut i = 0;
    while size > 1000.0 {
        if i > 4 {
            break;
        }
        size /= 1000.0;
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
    Moment,
    Dexie,
}

impl Lib {
    fn path(&self) -> String {
        match self {
            Lib::Jquery => "node_modules/jquery/dist/jquery.js".into(),
            Lib::Angular => "node_modules/angular/angular.js".into(),
            Lib::React => "node_modules/react/umd/react.development.js".into(),
            Lib::ReactDom => "node_modules/react-dom/umd/react-dom.development.js".into(),
            Lib::Vue => "node_modules/vue/dist/vue.js".into(),
            Lib::Moment => "node_modules/moment/moment.js".into(),
            Lib::Dexie => "node_modules/dexie/dist/dexie.js".into(),
        }
    }
}

fn get_js(l: Lib) -> Result<String, ::std::io::Error> {
    let path = PathBuf::from(l.path());
    if !path.exists() {
        npm_install()?;
        if !path.exists() {
            println!("cannot find {:?}", path);
        }
    }
    read_to_string(path)
}
