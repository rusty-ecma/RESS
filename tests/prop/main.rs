extern crate ress;
#[macro_use]
extern crate proptest;

proptest! {
    #[test]
    fn function_idents(s in r#"function [a-zA-Z_$\u2118\u212E\u309B\u309C\u1885\u1886][a-zA-Z_]+"#) {
        ress::tokenize(&s).unwrap();
    }
}
