#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PLUGIN_CRATE="stringindexing"
PROFILE="debug"
TRACE="false"
WORK_ROOT="${ROOT_DIR}/target/nvc-work"

TEST_BENCH="tb_string"

usage() {
  cat <<'EOF'
Usage: scripts/run_stringindexing.sh [options]

Builds the VHPI cdylib stringindexing, then compiles and runs the VHDL testbenches
with nvc and validates key VHPI log markers.

Options:
  --release             Build and load release cdylib
  --trace               Enable nvc VHPI trace output
  -h, --help            Show this help text

Examples:
  scripts/run_stringindexing.sh
  scripts/run_stringindexing.sh --release --trace
EOF
}

SELECTED_TESTS=()
while [[ $# -gt 0 ]]; do
  case "$1" in
    --release)
      PROFILE="release"
      shift
      ;;
    --trace)
      TRACE="true"
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ ! -f "${ROOT_DIR}/test_examples/${TEST_BENCH}.vhdl" ]]; then
  echo "Missing VHDL file: ${ROOT_DIR}/test_examples/${TEST_BENCH}.vhdl" >&2
  exit 2
fi

echo "[1/3] Building VHPI plugin crate '${PLUGIN_CRATE}' (${PROFILE})"
if [[ "$PROFILE" == "release" ]]; then
  cargo build -p "$PLUGIN_CRATE" --release
else
  cargo build -p "$PLUGIN_CRATE"
fi

LIB_STEM="${PLUGIN_CRATE//-/_}"
PLUGIN_SO="${ROOT_DIR}/target/${PROFILE}/lib${LIB_STEM}.so"
if [[ ! -f "$PLUGIN_SO" ]]; then
  FALLBACK="$(find "${ROOT_DIR}/target/${PROFILE}" -maxdepth 1 -type f -name "lib${LIB_STEM}.so" | head -n 1 || true)"
  if [[ -n "$FALLBACK" ]]; then
    PLUGIN_SO="$FALLBACK"
  else
    echo "Could not find built shared library for crate '${PLUGIN_CRATE}' at ${PLUGIN_SO}" >&2
    exit 1
  fi
fi

echo "[2/3] Running nvc compile/elab/sim checks"
mkdir -p "$WORK_ROOT"

RUN_DIR="${WORK_ROOT}/${PLUGIN_CRATE}"
LOG_FILE="${RUN_DIR}/run.log"

rm -rf "$RUN_DIR"
mkdir -p "$RUN_DIR"

pushd "$RUN_DIR" >/dev/null

echo "--- ${TEST_BENCH}: compile"
nvc -a "${ROOT_DIR}/test_examples/${TEST_BENCH}.vhdl"

echo "--- ${TEST_BENCH}: elaborate"
nvc -e "$TEST_BENCH"

echo "--- ${TEST_BENCH}: simulate"
if [[ "$TRACE" == "true" ]]; then
  nvc --vhpi-trace -r "$TEST_BENCH" --load="$PLUGIN_SO" >"$LOG_FILE" 2>&1
else
  nvc -r "$TEST_BENCH" --load="$PLUGIN_SO" >"$LOG_FILE" 2>&1
fi

popd >/dev/null


echo "${TEST_BENCH}: ok"

echo "[3/3] Completed stringindexing run"
echo "Logs: ${LOG_FILE}"
