`termbook` is a command-line tool and library designed to make your 
[`mdbook` documentation][mdbook-book] executable.

Thus you are advised to use the `mdbook` command-line tool for everything that
`termbook` does not support, as it is essentially nothing more than an `mdbook`
clone with a custom preprocessor. The latter is not easily supported by `mdbook`
just yet.

## Features 

`termbook` can do a few unique things for you

 * **execute codeblocks**
   * This makes your documentation executable, with the output captured in their
    own codeblocks.
   * That way your documentation never goes out of sync with reality,
    and allows you to build documentation as part of your test suite.
 * **playback books to the terminal**
   * This can be seen as 'eye-candy' generator, which is meant to be recorded with
     [asciinema][asciinema] or similar tools.
   * Generate videos without ever having to type out commands yourself, and keep them
     consistent with your documentation.

[asciinema]: https://asciinema.org
[mdbook-book]: https://rust-lang-nursery.github.io/mdBook/
