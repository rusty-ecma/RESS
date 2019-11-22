#![allow(clippy::all)]

#[inline]
pub(crate) fn is_other_whitespace(c: char) -> bool {
    if c < '\u{2000}' {
        false
    } else if c >= '\u{2000}' && c <= '\u{200A}' {
        true
    } else if c < '\u{202F}' {
        false
    } else if c == '\u{202F}' {
        true
    } else if c < '\u{205F}' {
        false
    } else if c == '\u{205F}' {
        true
    } else if c < '\u{3000}' {
        false
    } else if c == '\u{3000}' {
        true
    } else {
        false
    }
}
