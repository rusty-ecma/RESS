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