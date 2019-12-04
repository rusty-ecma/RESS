#![allow(clippy::all)]

#[inline]
pub(crate) fn is_id_start(c: char) -> bool {
    if c >= 'a' && c <= 'z' {
        true
    } else if c >= 'A' && c <= 'Z' {
        true
    } else if c == '\\' || c == '_' || c == '$' {
        true
    } else if c < '\u{AA}' {
        false
    } else if c == '\u{2118}'
        || c == '\u{212E}'
        || c == '\u{309B}'
        || c == '\u{309C}'
        || c == '\u{1885}'
        || c == '\u{1886}'
    {
        true
    } else {
        unic_ucd_ident::is_id_start(c)
    }
}
#[inline]
pub(crate) fn is_id_continue(c: char) -> bool {
    if c >= 'a' && c <= 'z' {
        true
    } else if c >= 'A' && c <= 'Z' {
        true
    } else if c >= '0' && c <= '9' {
        true
    } else if c == '\\' || c == '_' || c == '$' {
        true
    } else if c < '\u{AA}' {
        false
    } else if c == '\u{2118}'
        || c == '\u{212E}'
        || c == '\u{309B}'
        || c == '\u{309C}'
        || c == '\u{1885}'
        || c == '\u{1886}'
        || c == '\u{1369}'
        || c == '\u{136A}'
        || c == '\u{136B}'
        || c == '\u{136C}'
        || c == '\u{136D}'
        || c == '\u{136E}'
        || c == '\u{136F}'
        || c == '\u{1370}'
        || c == '\u{1371}'
        || c == '\u{B7}'
        || c == '\u{387}'
        || c == '\u{19DA}'
    {
        true
    } else {
        unic_ucd_ident::is_id_continue(c)
    }
}
