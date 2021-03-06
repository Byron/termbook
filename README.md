[![](http://meritbadge.herokuapp.com/termbook)](https://crates.io/crates/termbook)
![Rust](https://github.com/Byron/termbook/workflows/Rust/badge.svg)

`termbook` is a command-line tool to build [`mdbook`'s][mdbook] while executing
codeblocks and collecting their output to become part of the `mdbook`.

This allows to write testable documentation for command-line interfaces.

It came to life for the documentation needs of [share-secrets-safely][sheesy].

[![asciicast](https://asciinema.org/a/163556.png)](https://asciinema.org/a/163556)

[mdbook]: https://github.com/rust-lang-nursery/mdBook
[sheesy]: https://github.com/Byron/share-secrets-safely

### Installation

Installation documentation can [be found here][installation-docs].

[installation-docs]: https://byron.github.io/termbook/cli/installation.html#installation

### Documentation

`termbook` uses `termbook` to document itself. [Click here for more!][docs]

[docs]: https://byron.github.io/termbook

### Roadmap

### Add 'replace' support

Currently we can only append the output of programs. However, if there was 'replace',
we can implement html converters, such as https://github.com/ivanceras/svgbobrus.

That way, diagrams can be written in text, either in the code-block, or in files which
are then included via 'include-file'.

#### `termbook` renderer plugin programs for `mdbook`

Provide all renderers in `termbook` as standalone `mdbook` compatible plugin
renderers, to allow easier integration with the `mdbook` binary.

Depends on https://github.com/rust-lang-nursery/mdBook/issues/631

#### `termbook` preprocessor plugin programs  for `mdbook`

Ideally, we don't have to wrap `mdbook` into `termbook`. Instead, there should
be plugin-preprocessor support for `mdbook`, similar to what's already done
for renderers.

### Maintenance Guide

#### Making a new release

 * **Assure all documentation is up-to-date and tests are green**
 * **Run cargo-clippy and fix all issues**
 * update the `version` in all `Cargo.toml` files and `git commit`
 * run `cargo publish` for the library and the CLI
 * run `git tag -s <version>`
 * run `git push --tags origin master`
 * **update the asciinema docs**
   * Set your terminal to a decent size
   * run `make asciinema-no-upload`
   * When happy with the result, run `make asciinema-upload`
   * visit the URL, configure the video, make it public, and copy the 
     markdown link into the README file.
 * **update brew file**
   * run `make update-homebrew` and commit the change
 
Documentation is updated on each push to master.
