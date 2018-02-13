[![Build Status](https://travis-ci.org/Byron/termbook.svg?branch=master)](https://travis-ci.org/Byron/termbook)
[![dependency status](https://deps.rs/repo/github/byron/termbook/status.svg)](https://deps.rs/repo/github/byron/termbook)

`termbook` is a command-line tool to build [`mdbook`'s][mdbook] while executing
`bash` codeblocks and collecting their output to become part of the `mdbook`.

This allows to write testable documentation for command-line interfaces.

It came to life for the documentation needs of [share-secrets-safely][sheesy]

[mdbook]: https://github.com/rust-lang-nursery/mdBook
[sheesy]: https://github.com/Byron/share-secrets-safely

### Roadmap

#### `termbook` and `termbook-cli` - minimal viable product

Termbook will be tested on a high-level comprised mostly of journey tests. The CLI
will support

 * `build`ing an `mdbook` 
   * It's similar to `mdbook build`, but with a preprocessor to execute bash scripts
     and collect their output.
   * this can also be run on CI to not only create the book, but to assure your bash
     code examples are still working.
     
#### `termbook interactive` and `asciinema`

Pretty-print (chapters of) an `mdbook` to the terminal, similar to [`mdcat`][mdcat],
with customizable settings for the speed of printing, and how `bash` scripts should be
'typed'. The goal is to make recording demonstrations as pretty and informative as the
mdbook itself, and make recording that using asciinema easy.

That should make recordings reproducible.

[mdcat]: https://github.com/lunaryorn/mdcat

### Maintenance Guide

#### Making a new release

 * **Assure all documentation is up-to-date and tests are green**
 * update the `version` in all `Cargo.toml` files and `git commit`
 * run `cargo publish` for the library and the CLI
 * run `git tag -s <version>`
 * run `git push --tags origin master`
