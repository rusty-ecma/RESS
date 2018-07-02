extern crate js_parser;
// extern crate reqwest;

use js_parser::tokenize;

fn main() {
    // let c = reqwest::Client::new();
    // let mut jq_res = c.get("https://code.jquery.com/jquery-3.3.1.js")
    //     .send()
    //     .unwrap();
    let jq = ::std::fs::read_to_string("./tests/jquery.js").unwrap();
    // let s = js_parser::Scanner::new(jq.clone());
    // for token in s {
    //     println!("{:?}", token);
    // }
    println!("{:#?}", tokenize(&jq));
}

