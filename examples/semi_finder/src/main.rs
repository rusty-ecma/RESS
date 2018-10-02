extern crate ress;
extern crate walkdir;

use ress::{Punct, Scanner};
use walkdir::WalkDir;

use std::{collections::HashMap, env::args, fs::read_to_string, path::PathBuf};

fn main() {
    let mut args = args();
    let _ = args.next();
    let start = args
        .next()
        .expect("No directory provided as starting location.");
    if let Err(issues) = check_files(start) {
        for (path, indexes) in issues {
            println!("Issues found in {:?} at indexes:", path);
            println!("\t{:?}\n", indexes)
        }
    } else {
        println!("Good to go, no semicolons found");
    }
}

fn check_files(start: String) -> Result<(), HashMap<PathBuf, Vec<usize>>> {
    // We are going to store the location of any semi-colons we have found
    let mut ret: HashMap<PathBuf, Vec<usize>> = HashMap::new();
    // loop over the directories in our path
    // set the min_depth to 1, so we will skip the
    // path passed in as `start`
    for entry in WalkDir::new(start).min_depth(1) {
        // If the entry doesn't error
        if let Ok(entry) = entry {
            // capture the path of this entry
            let path = entry.path();
            //if the path ends with js, we want to check for semicolons
            if path.extension() == Some(::std::ffi::OsStr::new("js")) {
                println!("checking: {:?}", path);
                // if we can read the file to a string
                if let Ok(js) = read_to_string(path) {
                    // pas the text off to our check_js fn
                    if let Err(indexes) = check_js(&js) {
                        // if we found any semicolons, add them to our hashmap
                        // println!("found {} semicolons", indexes.len());
                        ret.insert(path.to_path_buf(), indexes);
                    }
                }
            }
        }
    }
    // if we found any semi-colons, send them up to the caller
    if ret.len() > 0 {
        Err(ret)
    } else {
        Ok(())
    }
}

fn check_js(js: &str) -> Result<(), Vec<usize>> {
    // Create a scanner with the text
    let s = Scanner::new(js);
    // filter out any tokens that are not semi-colons
    // then collect them all into a `Vec` of the start index
    // for the semi-colon
    let semis: Vec<usize> = s
        .filter_map(|item| {
            // If this token matches the `Punct::SemiColon`
            if item.token.matches_punct(Punct::SemiColon) {
                // we want to return the first position of this token
                // since semi-colons are only 1 character wide we would
                // only need this part of the `Span`
                Some(item.span.start)
            } else {
                None
            }
        }).collect();
    // If we have anything in the result of the `filter_map`
    // we will return an error
    if semis.len() > 0 {
        Err(semis)
    } else {
        Ok(())
    }
}
