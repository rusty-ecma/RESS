use ress::prelude::*;
use walkdir::WalkDir;

use std::{env::args, fs::read_to_string};

fn main() {
    let mut args = args();
    let _ = args.next();
    let start = args
        .next()
        .expect("No directory provided as starting location.");
    println!("static REGEXES: &[&str] = &[");
    let mut set = std::collections::HashSet::new();
    for path in WalkDir::new(start) {
        if let Ok(entry) = path {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "js" {
                        if let Ok(js) = read_to_string(path) {
                            let s = Scanner::new(&js);
                            for item in s {
                                if let Ok(item) = item {
                                    if item.token.is_regex() {
                                        let s = js[item.span.start..item.span.end].to_string();
                                        if set.insert(s) {
                                            println!(
                                                "    r#\"{}\"#,",
                                                &js[item.span.start..item.span.end]
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("];");
}
