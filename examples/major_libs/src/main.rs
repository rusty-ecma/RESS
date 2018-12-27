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
    pub refs: bool,
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
            refs: false,
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
        } else if arg == "refs" {
            a.refs = true;
        }
    }
    if a.jquery {
        jquery(a.refs);
    }
    if a.angular {
        angular1(a.refs);
    }
    if a.react {
        react(a.refs);
    }
    if a.react_dom {
        react_dom(a.refs);
    }
    if a.vue {
        vue(a.refs);
    }
    if a.moment {
        moment(a.refs);
    }
    if a.dexie {
        dexie(a.refs);
    }
    if a.pristine() {
        jquery(a.refs);
        angular1(a.refs);
        react(a.refs);
        react_dom(a.refs);
        vue(a.refs);
        moment(a.refs);
        dexie(a.refs);
    }
}

fn jquery(refs: bool) {
    println!("trying jquery");
    if let Ok(ref js) = get_js(Lib::Jquery) {
        test_js(js, "jquery", refs);
    }
}

fn angular1(refs: bool) {
    println!("trying angular1");
    if let Ok(ref js) = get_js(Lib::Angular) {
        test_js(js, "angular", refs);
    }
}

fn react(refs: bool) {
    println!("trying react");
    if let Ok(ref js) = get_js(Lib::React) {
        test_js(js, "react", refs);
    }
}

fn react_dom(refs: bool) {
    println!("trying react_dom");
    if let Ok(ref js) = get_js(Lib::ReactDom) {
        test_js(js, "react-dom", refs);
    }
}

fn vue(refs: bool) {
    println!("trying vue");
    if let Ok(ref js) = get_js(Lib::Vue) {
        test_js(js, "vue", refs);
    }
}

fn moment(refs: bool) {
    println!("trying moment");
    if let Ok(ref js) = get_js(Lib::Moment) {
        test_js(js, "moment", refs)
    }
}

fn dexie(refs: bool) {
    println!("trying dexie");
    if let Ok(ref js) = get_js(Lib::Dexie) {
        test_js(js, "dexie", refs);
    }
}

fn test_js(text: &str, name: &str, refs: bool) {
    let size = text.len();
    let now = SystemTime::now();
    if refs {
        test_ref(text, name);
    } else {
        test(text, name);
    }
    if let Ok(e) = now.elapsed() {
        report(size, e, "scanner", name)
    } else {
        println!("error capturing scanner duration for {}", name);
    }
}

fn test_ref(text: &str, name: &str) {
    let s = ress::refs::RefScanner::new(text);
    let _: Vec<ress::refs::RefItem> = s.collect();
}

fn test(text: &str, name: &str) {
    let s = ress::Scanner::new(text);
    let _: Vec<ress::Item> = s.collect();
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
    Moment,
    Dexie,
}

impl Lib {
    fn path(&self) -> String {
        match self {
            &Lib::Jquery => "node_modules/jquery/dist/jquery.js".into(),
            &Lib::Angular => "node_modules/angular/angular.js".into(),
            &Lib::React => "node_modules/react/umd/react.development.js".into(),
            &Lib::ReactDom => "node_modules/react-dom/umd/react-dom.development.js".into(),
            &Lib::Vue => "node_modules/vue/dist/vue.js".into(),
            &Lib::Moment => "node_modules/moment/moment.js".into(),
            &Lib::Dexie => "node_modules/dexie/dist/dexie.js".into(),
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
