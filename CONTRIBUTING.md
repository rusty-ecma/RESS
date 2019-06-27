# Contributing to RESS

If you are interested in contributing to RESS know that your help would be appreciated!

Feel free to open issues and/or pull requests for anything that you see that might be an improvement.
Please note that [ressa](https://github.com/freemasen/ressa) and [resast](https://github.com/freemasen/resast) may already have an issue opened.

I do not work on this full time, please be patient if I am not able to respond quickly.

The primary development branch is the `next` branch. It would be ideal to create any pull requests against that branch over `master` or one of the other feature branches that might have been missed when cleaning up.

For any PRs know that the code must pass travis and appveyor tests before they will be reviewed/merged. These test include the following commands you could use to check your version.
```sh
$ npm i
$ cargo test
$ cargo run --example major_libs --release
```
The release flag in the above is due to the fact that this example is a naive benchmark to validate that changes haven't completely ruined the performance. Feel free to leave this flag off when you are testing for a PR.

This will run all of the project's unit tests as well as a test against some major js libraries, namely [Angular-js](angularjs.org), [Jquery](jquery.com), [React/React-Dom](reactjs.org), [Vue](vuejs.org), [Moment.js](momentjs.com) and [Dexie](dexie.org).

If you are interested in becoming a maintainer send me an email and we can talk more about what that looks like.

There are a few things you might need to know to get started. First, the tests and benchmarks require that `npm` is installed to pull down the javascript they evaluate so you'll need [node.js](https://nodejs.org/en/) installed. 

Because the benchmarks use Criterion, it can be difficult to use them with profiling so each of the single token benchmarks is extracted out as an example (you can find these in the examples/instruments folder). For the major_libs benchmark, you can use the example with the same name. These are helpful for working with tools like [`cargo instruments`](https://crates.io/crates/cargo-instruments).

The overall code layout works like this.

- lib.rs
  - `Scanner`: The primary interface for this crate
    - Mostly this is a wrapper around Tokenizer that handles detecting regexes and calculating line/column numbers
  - `ScannerState`: This is used for caching the state and resetting it. See the `Scanner::get_state` and `Scanner::set_state` methods
- erros.rs
  - This is where the error structs live. If you add a new error type to the `Tokenizer` you will need to add a Into/From implementation here
- look_behnid: 
  - `LookBehind`: This is ring like structure that is used to keep the look behind tokens.
    - For regex detection we only care are the last token we have seen and the three toknes before an open parentheses, so the Scanner keeps two of these on hand.
    - The basic idea here is to just use a 3 element array and keep track of where we last put an element to be able to calculate which is `last`, `two` or `three`. 
  - `MetaToken`: a cheaper token variant which only holds the bare minimum of information for regex detection
- tokenizer
  - mod.rs
    - `RawItem`: a cheaper version of the `Item` struct from above, it has only as much information as the `Tokenizer` can determine; a `RawToken` and the byte index of the start and end.
    - `Tokenizer`: This is the primary export of this module. This struct will perform the actual seperation and classification of tokens
    - One note about the matching logic, matching on the length of a byte array or string a bunch of times with an if clause is cheaper than matching on the strings directly. Until [phf](https://github.com/sfackler/rust-phf) can handle byte slices, this is the fastest method available
  - bufer.rs
    - `JSBuffer`: Mostly a reimplementation of [std::Chars](https://doc.rust-lang.org/std/str/struct.Chars.html)
      - For most look_ahead operations there is `look_ahead_matches` which takes a byte slice, however if you are looking for a single byte character the `look_ahead_byte_matche` is slightly faster
      - `at_new_line` the `cmp` operation on u8 is faster than matching or `eq` so checking if something is smaller than a target is faster than doing bounds checks between `||`s
    - tokens.rs
      - `RawToken`: This is a token more tailored to directing the Scanner about how to construct a `tokens::Token`
        - The three cases that can have new lines carry some extra information with them, the `new_line_count` and the `last_len` (length of the last line)
      - `CommentKind`: empty version of `tokens::Comment`
      - `StingKind`: empty version of `tokens::StringLit`
      - `TemplateKind`: empty version of `tokens::Template`
  - unicode.rs
    - bounds checks on `char`s is more effective than binary search (which the two unicode implemtations I could find use) so these function bodies are generated using the approprate table
    - The generation code may become available in the future but right now it isn't very effective
    - `is_ident_start`: check if a `char` has the attribute of ident_start
    - `is_id_continue`: check if a `char` has the attribute of ident_continue
    - `is_other_whitesapce`: the ECMA spec says that any Zs category character is valid whitespace. This function will test any exotic whitespaces 