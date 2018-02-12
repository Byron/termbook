#!/bin/bash

set -eu -o pipefail
exe=${1:?First argument it the executable under test}

root="$(cd "${0%/*}" && pwd)"
exe="$(cd "${exe%/*}" && pwd)/${exe##*/}"

# shellcheck source=./tests/book-helpers.sh
source "$root/book-helpers.sh"

SUCCESSFULLY=0

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
    (with "no specifically marked code blocks"
      make-book "$fixture/books/no-markers.md"
      it "succeeds" && {
        expect_run $SUCCESSFULLY "${args[@]}" $BOOK
      }
      
      it "wrote the original books files without any insertions" && {
        expect_snapshot "$snapshot/book-no-markers" "$OUTPUT_DIR/markdown-rewrite" 
      }
    )
    
    (with "an 'exec' codeblock with no output specification"
      make-book "$fixture/books/exec-blank.md"
      
      it "succeeds" && {
        expect_run $SUCCESSFULLY "${args[@]}" $BOOK
      }
      
      it "added another marked codeblock with the output" && {
        expect_snapshot "$snapshot/book-exec-blank" "$OUTPUT_DIR/markdown-rewrite" 
      }
    )
  )
)

