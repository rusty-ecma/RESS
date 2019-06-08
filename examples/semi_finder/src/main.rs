extern crate ress;
extern crate walkdir;

use ress::{Punct, RefToken, Scanner};
use walkdir::WalkDir;

use std::{collections::HashMap, env::args, fs::read_to_string, path::PathBuf};

fn main() {
    // get the command line arguments that started this process
    let mut args = args();
    // discard the first argument, this will be the path to our
    // executable
    let _ = args.next();
    // The next argument will be the path to check
    // panic and display an error to the user if no path
    // was provided
    let start = args
        .next()
        .expect("No directory provided as starting location.");
    // Pass the argument off to our `check_files` function
    let issues = check_files(start);
    // If no issues were found
    if issues.is_empty() {
        // Print the success message
        println!("Good to go, no semicolons found");
    } else {
        // Otherwise loop over the hashmap and
        // tell the user where we found semi-colons that need to be
        // removed
        for (path, indexes) in issues {
            println!("Issues found in {:?} at indexes:", path);
            println!("\t{:?}\n", indexes)
        }
    }
}

fn check_files(start: String) -> HashMap<PathBuf, Vec<usize>> {
    // We are going to store the location of any semi-colons we have found
    let mut ret: HashMap<PathBuf, Vec<usize>> = HashMap::new();
    // loop over the directories in our path
    // set the min_depth to 1, so we will skip the
    // path passed in as `start`
    for entry in WalkDir::new(start).min_depth(1) {
        match entry {
            Ok(entry) => {
                // If the entry doesn't error
                // capture the path of this entry
                let path = entry.path();
                //if the path ends with js, we want to check for semicolons
                if path.extension() == Some(::std::ffi::OsStr::new("js")) {
                    // if we can read the file to a string
                    // pass the text off to our check_js fn
                    // if we can't we'll just skip it for now
                    if let Ok(js) = read_to_string(path) {
                        let indexes = check_js(&js);
                        // if we found any semicolons, add them to our hashmap
                        if !indexes.is_empty() {
                            ret.insert(path.to_path_buf(), indexes);
                        }
                    }
                }
            }
            Err(e) => eprintln!("failed to get a directory entry: {:?}", e),
        }
    }
    ret
}

fn check_js(js: &str) -> Vec<usize> {
    // Create a scanner with the text then
    // filter out any tokens that are not semi-colons
    // then collect them all into a `Vec` of the start indexes
    Scanner::new(js)
        .filter_map(|item| {
            let item = item.unwrap();
            // If this token matches the `Punct::SemiColon`
            if let RefToken::Punct(ref inner) = item.token {
                match inner {
                    // we want to return the first position of this token
                    // since semi-colons are only 1 character wide we would
                    // only need this part of the `Span`
                    Punct::SemiColon => Some(item.span.start),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect()
}
