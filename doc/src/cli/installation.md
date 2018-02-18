`termbook` is a minimal clone of the `mdbook` command-line tool, and will only
be required to build the final version of your book as it adds the needed
[`mdbook` preprocessor][mdbook-preprocessor].

[mdbook-preprocessor]: https://rust-lang-nursery.github.io/mdBook/for_developers/preprocessors.html

## Installation

### Via HomeBrew (OSX and Linux)

This is by far the most straight-forward way of installing termbook. Just execute
the following code.

```bash
brew tap byron/termbook https://github.com/byron/termbook.git
brew install termbook
```

### Via Github-Releases

At the [github releases page][gh-releases] you will find precompiled binaries
for all common platforms. 

Just decompress the respective archive and copy the `termbook` binary into your
`$PATH`, usually this will be `/usr/local/bin`.

_Windows is notably absent, but could be provided if 
there is demand._

[gh-releases]: https://github.com/Byron/termbook/releases

### Via Cargo

`termbook` can be installed via `cargo` only, which in turn can be obtained
via [`rustup`][rustup].

Then it's as easy as

```bash
cargo install termbook-cli
```

[rustup]: http://rustup.rs/ 

### After the installation...

Now you should be able to run `termbook`:

```bash,exec
termbook --help
```

