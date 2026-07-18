#!/usr/bin/env bash
set -uo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PLUGIN_CRATE="foreignf"
PROFILE="debug"
TRACE="false"
SHOW_LOG="false"
WORK_ROOT="${ROOT_DIR}/target/nvc-work"
EXPECTED_MARKERS=(
    "foreignf plugin loaded"
  "foreignf: all checks passed \(4 mark_call invocations, 4 add_ints invocations, 6 bit_reverse invocations\)"
)

TEST_BENCH="tb_foreignf"

usage() {
  cat <<'EOF'
Usage: scripts/run_foreignf_checks.sh [options]

Builds the VHPI cdylib foreignf, then compiles and runs tb_foreignf with nvc
and validates key VHPI log markers.

Options:
  --release             Build and load release cdylib
  --trace               Enable nvc VHPI trace output
  --show-log            Print simulation logfile at the end
  -h, --help            Show this help text

Examples:
  scripts/run_foreignf_checks.sh
  scripts/run_foreignf_checks.sh --release --trace
  scripts/run_foreignf_checks.sh --show-log
EOF
}

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
    --show-log)
      SHOW_LOG="true"
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
case "${OSTYPE:-}" in
  msys*|cygwin*|win32*)
    PLUGIN_LIB_NAME="${LIB_STEM}.dll"
    ;;
  darwin*)
    PLUGIN_LIB_NAME="lib${LIB_STEM}.dylib"
    ;;
  *)
    PLUGIN_LIB_NAME="lib${LIB_STEM}.so"
    ;;
esac

PLUGIN_SO="${ROOT_DIR}/target/${PROFILE}/${PLUGIN_LIB_NAME}"
if [[ ! -f "$PLUGIN_SO" ]]; then
  FALLBACK="$(find "${ROOT_DIR}/target/${PROFILE}" -maxdepth 1 -type f -name "${PLUGIN_LIB_NAME}" | head -n 1 || true)"
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

for marker in "${EXPECTED_MARKERS[@]}"; do
  if ! grep -Eq "$marker" "$LOG_FILE"; then
    echo "${TEST_BENCH}: missing marker /${marker}/" >&2
    cat "$LOG_FILE" >&2
    exit 1
  fi
done

echo "${TEST_BENCH}: ok"

echo "[3/3] Completed ${TEST_BENCH} run"
echo "Logs: ${LOG_FILE}"

if [[ "$SHOW_LOG" == "true" ]]; then
  echo "----- begin ${LOG_FILE} -----"
  cat "$LOG_FILE"
  echo "----- end ${LOG_FILE} -----"
fi
