#!/bin/bash

set -eu -o pipefail
exe=(cargo run --example stupicat --)

root="$(cd "${0%/*}" && pwd)"
# shellcheck source=./tests/utilities.sh
source "$root/utilities.sh"

SUCCESSFULLY=0

title ""
fixture="$root/fixtures"
snapshot="$fixture/snapshots"

(with "a standard common-mark example file"
  it "succeeds" && \
    WITH_SNAPSHOT="$snapshot/stupicat-output" \
    expect_run_sh $SUCCESSFULLY "${exe[*]} $fixture/common-mark.md 2>/dev/null"
)

