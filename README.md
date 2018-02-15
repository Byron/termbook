[![Build Status](https://travis-ci.org/Byron/termbook.svg?branch=master)](https://travis-ci.org/Byron/termbook)
[![dependency status](https://deps.rs/repo/github/byron/termbook/status.svg)](https://deps.rs/repo/github/byron/termbook)

`termbook` is a command-line tool to build [`mdbook`'s][mdbook] while executing
codeblocks and collecting their output to become part of the `mdbook`.

This allows to write testable documentation for command-line interfaces.

It came to life for the documentation needs of [share-secrets-safely][sheesy].

[mdbook]: https://github.com/rust-lang-nursery/mdBook
[sheesy]: https://github.com/Byron/share-secrets-safely

### Installation

Installation documentation can [be found here][installation-docs].

[installation-docs]: https://byron.github.io/termbook/cli/installation.html#installation

### Documentation

`termbook` uses `termbook` to document itself. [Click here for more!][docs]

[docs]: https://byron.github.io/termbook

### Roadmap

#### `termbook interactive` and `asciinema`

Pretty-print (chapters of) an `mdbook` to the terminal, similar to [`mdcat`][mdcat],
with customizable settings for the speed of printing, and how `bash` scripts should be
'typed'. The goal is to make recording demonstrations as pretty and informative as the
mdbook itself, and make recording that using asciinema easy.

That should make recordings reproducible.

 * [ ] Chapter Regex
 * [ ] asciinema integration

[mdcat]: https://github.com/lunaryorn/mdcat

#### `termbook` renderer plugin programs for `mdbook`

Provide all renderers in `termbook` as standalone `mdbook` compatible plugin
renderers, to allow easier integration with the `mdbook` binary.

#### `termbook` preprocessor plugin programs  for `mdbook`

Ideally, we don't have to wrap `mdbook` into `termbook`. Instead, there should
be plugin-preprocessor support for `mdbook`, similar to what's already done
for renderers.

### Maintenance Guide

#### Making a new release

 * **Assure all documentation is up-to-date and tests are green**
 * update the `version` in all `Cargo.toml` files and `git commit`
 * run `cargo publish` for the library and the CLI
 * run `git tag -s <version>`
 * run `git push --tags origin master`
 
documentation is updated on each push to master.
