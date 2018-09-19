# Contributing to RESS

If you are interested in contributing to RESS know that I would be happy for the help!

Feel free to open issues and/or pull requests for anything that you see that might be an improvement.

I do not work on this full time, please be patient if I am not able to respond quickly.

For any PRs know that the code must pass travis and appveyor tests before they will be reviewed/merged. These test include the following commands you could use to check your version.
```sh
$ npm i
$ cargo test
$ cargo run --example major_libs --release
```

This will run all of the project's unit tests as well as a test against some major js libraries, namely Angular-js, Jquery, React, React-Dom and Vue.

If you are interested in becoming a maintainer send me an email and we can talk more about what that looks like.

# What you might help with
- If you have been using RESS and would love to see a helper function to the `Token` enum or `Item` struct, that sounds great!
- If you have more experience with parsing and/or `combine` and see an area for performance improvements, I would maybe do a back-flip.