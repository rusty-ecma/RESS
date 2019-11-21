#![cfg(all(test, feature = "moz_central"))]

use indicatif::{ProgressBar, ProgressStyle};

use zip::read::ZipArchive;
use ress::*;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

#[test]
fn moz_central() {
    let _ = pretty_env_logger::try_init();
    let moz_central_path = Path::new("./moz-central");
    get_moz_central_test_files(&moz_central_path);
    if !moz_central_path.exists() {
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
    if !path.exists() {
        std::fs::create_dir_all(path).expect("failed to create root path");
    }
    let mut response = reqwest::get(
        "https://hg.mozilla.org/mozilla-central/archive/tip.zip/js/src/jit-test/tests/",
    )
    .expect("Failed to get zip of moz-central");
    
    let mut buf = Vec::new();
    response
        .copy_to(&mut buf)
        .expect("failed to copy to BzDecoder");
    let cur = std::io::Cursor::new(buf);
    let mut z = ZipArchive::new(cur).expect("failed to create ZipArchive");
    for i in 0..z.len() {
        let mut file = z.by_index(i).expect(&format!("failed to open file {} in zip archive", i));
        if file.is_dir() {
            std::fs::create_dir_all(path.join(file.sanitized_name())).expect(&format!("failed to create folder {}", file.name()));
        } else {
            let dest_path = path.join(file.sanitized_name());
            if !dest_path.exists() {
                if let Some(parent) = dest_path.parent() {
                    std::fs::create_dir_all(parent).expect(&format!("failed to create dir for {}", parent.display()));
                }
            }
            let mut dest = std::fs::File::create(&dest_path).expect(&format!("failed to create file {}", file.name()));
            std::io::copy(&mut file, &mut dest).expect(&format!("failed to copy from zip to disk: {}", file.name()));
        }
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
    let pb = ProgressBar::new(paths.len() as u64);
    let sty = ProgressStyle::default_bar()
        .template("{bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("█▓▒░  ");
    pb.set_style(sty.clone());
    for path in paths {
        ct += 1;
        pb.println(&format!("{}", path.display()));
        pb.inc(1);
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
