#!/usr/bin/env bash
# Check that no public functions in the vhpi crate expose vhpi-sys types
# in their argument lists or return types.
#
# "Public" here means `pub fn` or `pub(super) fn` — not `pub(crate) fn`.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VHPI_SRC="$ROOT/vhpi/src"

# For each .rs file, use awk to collect pub fn signatures (which may span
# multiple lines) and report any that reference vhpi_sys::.
#
# Strategy:
#   • When a line matches `pub fn` (excluding pub(crate)), start accumulating.
#   • Keep accumulating lines until we see `{` or `;` (end of signature).
#   • Strip everything from the first `{` or `;` onwards (the body / where
#     clause brace).
#   • If the accumulated signature contains `vhpi_sys::`, report it.
violations=$(
    awk '
        # Start of a public function signature (skip pub(crate) and pub(super))
        /pub fn / || /pub\(super\) fn / {
            # Skip crate-private items
            if (/pub\(crate\)/) next
            in_sig   = 1
            sig      = $0
            start    = FNR
        }

        # Accumulate continuation lines
        in_sig && FNR != start {
            sig = sig " " $0
        }

        # End of signature: line contains { or ;
        in_sig && /[{;]/ {
            # Drop everything from the first brace/semicolon onwards
            sub(/[{;].*/, "", sig)
            if (index(sig, "vhpi_sys::") > 0) {
                print FILENAME ":" start ": " sig
            }
            in_sig = 0
            sig    = ""
        }
    ' "$VHPI_SRC"/*.rs
)

if [ -z "$violations" ]; then
    echo "OK: no public functions expose vhpi-sys types."
    exit 0
else
    echo "ERROR: the following public functions expose vhpi-sys types:"
    while IFS= read -r line; do
        echo "  $line"
    done <<< "$violations"
    exit 1
fi
