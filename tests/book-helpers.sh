#!/bin/bash

# shellcheck disable=1090
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/utilities.sh"

function book_sandbox () {
  MDBOOK_BUILD__BUILD_DIR="$(mktemp -t book-build.XXXX -d)"
  export MDBOOK_BUILD__BUILD_DIR OUTPUT_DIR="$MDBOOK_BUILD__BUILD_DIR"
}

function sandboxed () {
  sandbox "book_sandbox"
}
