extern crate ress;
#[macro_use]
extern crate proptest;

proptest! {
    #[test]
    fn function_idents(s in "function [a-zA-Z_&][a-zA-Z_]+") {
        ress::tokenize(&s);
    }
}