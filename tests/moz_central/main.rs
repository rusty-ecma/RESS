#![cfg(all(test, feature = "moz_central"))]
extern crate flate2;
extern crate ress;
extern crate tar;

use flate2::read::GzDecoder;
use ress::*;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

#[test]
fn moz_central() {
    let _ = pretty_env_logger::try_init();
    let moz_central_path = Path::new("./moz-central");
    if !moz_central_path.exists() {
        get_moz_central_test_files(&moz_central_path);
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

fn get_moz_central_test_files(path: &Path) {
    let mut response = reqwest::get(
        "https://hg.mozilla.org/mozilla-central/archive/tip.tar.gz/js/src/jit-test/tests/",
    )
    .expect("Failed to get zip of moz-central");
    let mut buf = Vec::new();
    response
        .copy_to(&mut buf)
        .expect("failed to copy to BzDecoder");
    let gz = GzDecoder::new(buf.as_slice());
    let mut t = tar::Archive::new(gz);
    t.unpack(path).expect("Failed to unpack gz");
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
        eprintln!("\n{} file-> {}", ct, path.display());
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
