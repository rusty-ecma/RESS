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
    let failures = walk(&moz_central_path);
    if !failures.is_empty() {
        panic!("{} tests failed\n{}", failures.len(), failures.join("\n"));
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

fn walk(path: &Path) -> Vec<String> {
    let mut ret = vec![];
    let files: Vec<PathBuf> = path
        .read_dir()
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    files.iter().enumerate().for_each(|(_i, path)| {
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "js" {
                    let js = read_to_string(&path).unwrap();
                    for item in Scanner::new(js.as_str()) {
                        if let Err(e) = item {
                            ret.push(format!("{}, path: {}", e, path.display()));
                        }
                    }
                }
            }
        } else {
            ret.extend(walk(&path));
        }
    });
    ret
}
