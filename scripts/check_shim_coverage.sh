#!/usr/bin/env bash
# Check that every vhpi_* function called in the vhpi crate is
# also defined in vhpi-shim/src/lib.rs (either as a no_mangle export
# or as an internal forwarding function).
set -euo pipefail

VHPI_SRC="$(cd "$(dirname "$0")/.." && pwd)/vhpi/src"
SHIM_SRC="$(cd "$(dirname "$0")/.." && pwd)/vhpi-shim/src/lib.rs"

# Extract all vhpi_* function names referenced in the vhpi crate.
# Patterns covered:
#   vhpi_sys::vhpi_foo(   — direct namespaced call
#   use vhpi_sys::{…, vhpi_foo, …}  — use-import, then bare call
#
# Use a negative lookbehind so that identifiers like raw_vhpi_foo are
# not mistakenly matched as references to the function vhpi_foo.
used=$(
    grep -rPh --only-matching \
        '(?<![a-z_])(vhpi_[a-z_]+)\s*[(\,\}]' \
        "$VHPI_SRC" \
    | grep -o 'vhpi_[a-z_]*[a-z]' \
    | grep -v '^vhpi_sys$' \
    | sort -u
)

# Extract all function names defined in vhpi-shim.
shim_fns=$(
    grep -o 'fn vhpi_[a-z_]*[a-z]' "$SHIM_SRC" \
    | grep -o 'vhpi_[a-z_]*[a-z]' \
    | sort -u
)

missing=()
for fn in $used; do
    if ! echo "$shim_fns" | grep -qx "$fn"; then
        missing+=("$fn")
    fi
done

if [ ${#missing[@]} -eq 0 ]; then
    echo "OK: all vhpi functions used by the vhpi crate are present in vhpi-shim."
    exit 0
else
    echo "ERROR: the following functions are used by the vhpi crate but are missing from vhpi-shim:"
    for fn in "${missing[@]}"; do
        echo "  - $fn"
    done
    exit 1
fi
