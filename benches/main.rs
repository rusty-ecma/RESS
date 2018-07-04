#![feature(test)]

extern crate test;

use test::Bencher;
extern crate ress;
extern crate reqwest;
#[bench]
fn jquery(b: &mut Bencher) {
    let js = &get_js("https://code.jquery.com/jquery-3.3.1.js").unwrap();
    b.iter(||ress::tokenize(js));
}

#[bench]
fn angular1(b: &mut Bencher) {
    let js = &get_js("https://ajax.googleapis.com/ajax/libs/angularjs/1.5.6/angular.js").unwrap();
    b.iter(||ress::tokenize(js));
}

#[bench]
fn react(b: &mut Bencher) {
    let js = &get_js("https://unpkg.com/react@16/umd/react.development.js").unwrap();
    b.iter(||ress::tokenize(js));
}

#[bench]
fn react_dom(b: &mut Bencher) {
    let js = &get_js("https://unpkg.com/react-dom@16/umd/react-dom.development.js").unwrap();
    b.iter(||ress::tokenize(js));
}

#[bench]
fn vue(b: &mut Bencher) {
    let js = &get_js("https://cdn.jsdelivr.net/npm/vue@2.5.16/dist/vue.js").unwrap();
    b.iter(||ress::tokenize(js));
}


fn get_js(url: &str) -> Result<String, String> {
    let c = reqwest::Client::new();
    match c.get(url.clone()).send() {
        Ok(mut res) => match res.text() {
            Ok(js) => Ok(js.to_string()),
            Err(e) => Err(format!("Error getting js: {:?}", e))
        },
        Err(e) => Err(format!("Error getting js: {:?}", e)),
    }
}