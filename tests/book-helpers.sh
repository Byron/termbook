#!/bin/bash

# shellcheck disable=1090
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/utilities.sh"

function book_sandbox () {
  MDBOOK_BUILD__BUILD_DIR="$(mktemp -t book-build.XXXX -d)"
  export MDBOOK_BUILD__BUILD_DIR OUTPUT_DIR="$MDBOOK_BUILD__BUILD_DIR"
}

function make-book () {
  local index="${1:?}"
  local basename="${index##*/}"
  basename="${basename%.*}"
  
  mkdir "$basename"
  cat <<'EOF' > "$basename/book.toml"
[book]
authors = []
multilingual = false
src = "."
EOF

  cat <<'EOF' > "$basename/SUMMARY.md"
# Summary

 - [Introduction](./index.md)
EOF

  cp "$index" "$basename/index.md"

  export BOOK="$basename"
}

function sandboxed () {
  sandbox "book_sandbox"
}
