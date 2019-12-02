fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let path = args.next().expect("First argument must be a file path");
    let path = std::path::Path::new(&path);
    if !path.exists() {
        panic!("First argument must be a file path");
    }
    let js = std::fs::read_to_string(path).expect("Couldn't read the path provide");
    for item in ress::Scanner::new(&js) {
        println!("{:?}", item.expect("failed to lex token"));
    }
}
