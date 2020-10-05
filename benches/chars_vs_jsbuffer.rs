#![cfg(test)]
extern crate ress;

#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

fn ascii_string() -> String {
    string_from_range(0..256)
}
fn non_ascii_string() -> String {
    string_from_range(0x7FF..0x110000)
}
fn string_from_range(r: std::ops::Range<u32>) -> String {
    let mut ret = String::new();
    for i in r {
        if let Some(ch) = std::char::from_u32(i) {
            ret.push(ch);
        }
    }
    ret
}
fn chars_ascii_chars(c: &mut Criterion) {
    let s = ascii_string();
    chars(c, &s, "chars_ascii_chars");
}
fn chars_non_ascii_chars(c: &mut Criterion) {
    let mut s = non_ascii_string();
    chars(c, &s, "chars_non_ascii_chars");
}
fn jsb_ascii_chars(c: &mut Criterion) {
    let s = ascii_string();
    js_buffer(c, &s, "jsb_ascii_chars");
}
fn jsb_non_ascii_chars(c: &mut Criterion) {
    let s = non_ascii_string();
    js_buffer(c, &s, "jsb_non_ascii_chars")
}
fn chars(c: &mut Criterion, s: &str, name: &str) {
    c.bench_function(name, |b| {
        b.iter(|| {
            let mut chs = s.chars();
            while let Some(ch) = chs.next() {
                black_box(ch);
            }
        });
    });
}
fn js_buffer(c: &mut Criterion, s: &str, name: &str) {
    c.bench_function(name, |b| {
        b.iter(|| {
            let mut chs = ress::JSBuffer::new(s.as_bytes());
            while let Some(ch) = chs.next_char() {
                black_box(ch);
            }
        });
    });
}

criterion_group!(
    benches,
    chars_ascii_chars,
    chars_non_ascii_chars,
    jsb_ascii_chars,
    jsb_non_ascii_chars,
);
criterion_main!(benches);
