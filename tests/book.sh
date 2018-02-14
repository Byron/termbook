#!/bin/bash

set -eu -o pipefail
exe=${1:?First argument it the executable under test}

root="$(cd "${0%/*}" && pwd)"
exe="$(cd "${exe%/*}" && pwd)/${exe##*/}"

# shellcheck disable=1090
source "$root/book-helpers.sh"

SUCCESSFULLY=0
WITH_FAILURE=1

fixture="$root/fixtures"
snapshot="$fixture/snapshots"

title "termbook completions"

(when "using the completions subcommand"
  (with "a shell explicitly set"
    it "succeeds" && {
      expect_run_sh $SUCCESSFULLY "$exe completions bash | bash"
    }
  )
  (with "the shell derived from SHELL"
    it "succeeds" && {
      SHELL=/bin/bash expect_run_sh $SUCCESSFULLY "$exe completions | bash"
    }
  )
)

title "termbook build"
(sandboxed
  (with "rewrite enabled"
    args=("$exe" build --rewrite)

    (with "an 'include-file' codeblock"
      (with "a an non-existing file"
        make-book "$fixture/books/include-file-non-existing.md"

        it "fails with an error" && {
          WITH_SNAPSHOT="$snapshot/include-file-non-existing" \
          expect_run $WITH_FAILURE "${args[@]}" "$BOOK"
        }
      )

      (with "a an existing file relative to the book"
        make-book "$fixture/books/include-file-existing.md"
        cat <<'EOF' > "$BOOK/../outside-of-book.md"
```rust
fn included_from_file() {

}
```
EOF

        it "succeeds" && {
          expect_run $SUCCESSFULLY "${args[@]}" "$BOOK"
        }

        it "wrote the included file into the codeblock" && {
          expect_snapshot "$snapshot/include-file-existing" "$OUTPUT_DIR/markdown-rewrite"
        }
      )
    )

    (with "no specifically marked code blocks"
      make-book "$fixture/books/no-markers.md"
      it "succeeds" && {
        expect_run $SUCCESSFULLY "${args[@]}" "$BOOK"
      }

      it "wrote the original books files without any insertions" && {
        expect_snapshot "$snapshot/book-no-markers" "$OUTPUT_DIR/markdown-rewrite"
      }
    )

    (with "an 'exec' codeblock"
      (with "a non-existing program"
        make-book "$fixture/books/exec-nonexisting-program.md"

        it "fails" && {
          WITH_SNAPSHOT="$snapshot/exec-nonexisting-program"\
          expect_run $WITH_FAILURE "${args[@]}" "$BOOK"
        }
      )

      (with "no exit code specification"
        make-book "$fixture/books/exec-blank.md"

        it "succeeds as it defaults to 'expect success'" && {
          expect_run $SUCCESSFULLY "${args[@]}" "$BOOK"
        }

        it "added another marked codeblock with the output" && {
          expect_snapshot "$snapshot/book-exec-blank" "$OUTPUT_DIR/markdown-rewrite"
        }
      )

      (with "exit code specification"
        make-book "$fixture/books/exec-exit-code-error.md"

        it "succeeds as the exit code matches" && {
          expect_run $SUCCESSFULLY "${args[@]}" "$BOOK"
        }

        it "added another marked codeblock with the output" && {
          expect_snapshot "$snapshot/book-exec-exit-code-error" "$OUTPUT_DIR/markdown-rewrite"
        }
      )

      (with "exit code specification that does not match the actual exit code"
        make-book "$fixture/books/exec-exit-code-mismatch.md"

        it "fails" && {
          WITH_SNAPSHOT="$snapshot/exec-exit-code-mismatch" \
          expect_run $WITH_FAILURE "${args[@]}" "$BOOK"
        }
      )

      (with "invalid exit code specification"
        make-book "$fixture/books/exec-exit-code-invalid.md"

        it "fails" && {
          WITH_SNAPSHOT="$snapshot/exec-exit-code-invalid" \
          expect_run $WITH_FAILURE "${args[@]}" "$BOOK"
        }
      )
    )

    (with "'prepare' block"
      (with "no name"
        make-book "$fixture/books/exec-prepare-unnamed.md"

        it "fails as a name is needed" && {
          WITH_SNAPSHOT="$snapshot/exec-prepare-unnamed" \
          expect_run $WITH_FAILURE "${args[@]}" "$BOOK"
        }
      )
      (with "a name and a reference in an 'exec' block"
        make-book "$fixture/books/exec-prepare-named-and-referenced.md"

        it "succeeds" && {
          expect_run $SUCCESSFULLY "${args[@]}" "$BOOK"
        }

        it "executes the prepare step as well" && {
          expect_snapshot "$snapshot/book-exec-prepare-named-and-referenced" "$OUTPUT_DIR/markdown-rewrite"
        }
      )
      (with "having a 'use' step itself and an 'exec' block that uses it"
        make-book "$fixture/books/exec-prepare-named-with-use-and-referenced.md"

        it "succeeds" && {
          expect_run $SUCCESSFULLY "${args[@]}" "$BOOK"
        }

        it "executes the prepare step and its prepare step" && {
          expect_snapshot "$snapshot/book-exec-prepare-named-with-use-and-referenced" "$OUTPUT_DIR/markdown-rewrite"
        }
      )
    )

    (with "'use' block"
      (with "name that wasn't defined with 'prepare'"
        make-book "$fixture/books/use-unknown.md"

        it "fails" && {
          WITH_SNAPSHOT="$snapshot/use-unknown" \
          expect_run $WITH_FAILURE "${args[@]}" "$BOOK"
        }
      )
      (with "no name"
        make-book "$fixture/books/use-unnamed.md"

        it "fails as a name is needed" && {
          WITH_SNAPSHOT="$snapshot/use-unnamed" \
          expect_run $WITH_FAILURE "${args[@]}" "$BOOK"
        }
      )
    )

    (with "'hide' marker"
      (when "used on with a name"
        make-book "$fixture/books/hide-with-name.md"

        it "failse" && {
          WITH_SNAPSHOT="$snapshot/hide-with-name" \
          expect_run $WITH_FAILURE "${args[@]}" "$BOOK"
        }
      )
      (when "used on an 'exec' block"
        make-book "$fixture/books/hide-on-exec.md"

        it "succeeds" && {
          expect_run $SUCCESSFULLY "${args[@]}" "$BOOK"
        }

        it "hides the 'exec' block and its output" && {
          expect_snapshot "$snapshot/book-hide-on-exec" "$OUTPUT_DIR/markdown-rewrite"
        }
      )
      (when "used on a 'prepare' block with an non-hidden exec block"
        make-book "$fixture/books/hide-on-prepare.md"

        it "succeeds" && {
          WITH_SNAPSHOT="$snapshot/hide-on-prepare" \
          expect_run $SUCCESSFULLY "${args[@]}" "$BOOK"
        }

        it "hides the prepare block only" && {
          expect_snapshot "$snapshot/book-hide-on-prepare" "$OUTPUT_DIR/markdown-rewrite"
        }
      )
    )
  )
)
