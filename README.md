`termbook` is a command-line tool to build [`mdbook`'s][mdbook] while executing
`bash` codeblocks and collecting their output to become part of the `mdbook`.

This allows to write testable documentation for command-line interfaces.

It came to life for the documentation needs of [share-secrets-safely][sheesy]

[mdbook]: https://github.com/rust-lang-nursery/mdBook
[sheesy]: https://github.com/Byron/share-secrets-safely

### Roadmap

#### pulldown-cmark-to-cmark

A utility library which translates [`Event`][pdcm-event] back to markdown.
It's the prerequisite for writing markdown filters which can work as
[mdbook-preprocessors][mdbook-prep].

[pdcm-event]: https://docs.rs/pulldown-cmark/0.1.0/pulldown_cmark/enum.Event.html
[mdbook-prep]: https://rust-lang-nursery.github.io/mdBook/for_developers/preprocessors.html

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
