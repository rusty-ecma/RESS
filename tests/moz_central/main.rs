#![cfg(all(test, feature = "moz_central"))]

use ress::*;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

#[test]
fn moz_central() {
    let _ = pretty_env_logger::try_init();
    let moz_central_path = Path::new("moz_central");
    if !moz_central_path.exists() {
        panic!("please download the JIT tests from the firefox repository. see CONTRIBUTING.md for more info");
    }
    let paths = get_paths(&moz_central_path);
    let (failures, total) = walk(&paths);
    eprintln!("completed {:?} tests", total);
    if !failures.is_empty() {
        panic!(
            "{:?} tests failed\n{:?}",
            failures.len(),
            failures.join("\n")
        );
    }
}

fn get_paths(root: &Path) -> Vec<PathBuf> {
    walkdir::WalkDir::new(root)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| {
            let entry = e.expect("bad entry");
            let path = entry.into_path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "js" {
                        Some(path)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn walk(paths: &[PathBuf]) -> (Vec<String>, usize) {
    let mut ret = Vec::new();
    let mut ct = 0;
    for path in paths {
        ct += 1;
        let js = read_to_string(&path).unwrap();
        let s = Scanner::new(js.as_str());
        for item in s {
            if let Err(e) = item {
                ret.push(format!("{:?}, path: {:?}", e, path.display()));
            }
        }
    }
    (ret, ct)
}
