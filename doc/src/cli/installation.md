`termbook` is a minimal clone of the `mdbook` command-line tool, and will only
be required to build the final version of your book as it adds the required
[`mdbook` preprocessor][mdbook-preprocessor].

[mdbook-preprocessor]: https://rust-lang-nursery.github.io/mdBook/for_developers/preprocessors.html

## Installation

`termbook` currently can be installed via `cargo` only, which in turn can be obtained
via [`rustup`][rustup].

Then it's as easy as

```bash
cargo install termbook-cli
```

Now you should be able to run `termbook`:

```bash,exec
termbook --help
```

[rustup]: http://rustup.rs/ 
