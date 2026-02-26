#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PLUGIN_CRATE="dumper"
PROFILE="debug"
TRACE="false"
WORK_ROOT="${ROOT_DIR}/target/nvc-work"
EXPECTED_MARKERS=(
  "dumper plugin loaded"
  "start of simulation"
  "end of simulation"
)

TEST_BENCHES=(
  tb_simple
  tb_string
  tb_arrays
  tb_enums
  tb_physical_time
  tb_complex
)

usage() {
  cat <<'EOF'
Usage: scripts/run_vhpi_dumper.sh [options]

Builds the VHPI example application dumper, then compiles and runs VHDL testbenches
with nvc in isolated work directories and validates key VHPI log markers.

Options:
  --test <tb_name>      Run one testbench (may be passed multiple times)
  --release             Build and load release cdylib
  --trace               Enable nvc VHPI trace output
  -h, --help            Show this help text

Examples:
  scripts/run_vhpi_dumper.sh
  scripts/run_vhpi_dumper.sh --test tb_simple --test tb_complex
  scripts/run_vhpi_dumper.sh --release --trace
EOF
}

SELECTED_TESTS=()
while [[ $# -gt 0 ]]; do
  case "$1" in
    --test)
      SELECTED_TESTS+=("$2")
      shift 2
      ;;
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


if [[ ${#SELECTED_TESTS[@]} -gt 0 ]]; then
  TEST_BENCHES=("${SELECTED_TESTS[@]}")
fi

for tb in "${TEST_BENCHES[@]}"; do
  if [[ ! -f "${ROOT_DIR}/test_examples/${tb}.vhdl" ]]; then
    echo "Missing VHDL file: ${ROOT_DIR}/test_examples/${tb}.vhdl" >&2
    exit 2
  fi
done

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

for tb in "${TEST_BENCHES[@]}"; do
  RUN_DIR="${WORK_ROOT}/${PLUGIN_CRATE}/${tb}"
  LOG_FILE="${RUN_DIR}/run.log"

  rm -rf "$RUN_DIR"
  mkdir -p "$RUN_DIR"

  pushd "$RUN_DIR" >/dev/null

  echo "--- ${tb}: compile"
  nvc -a "${ROOT_DIR}/test_examples/${tb}.vhdl"

  echo "--- ${tb}: elaborate"
  nvc -e "$tb"

  echo "--- ${tb}: simulate"
  if [[ "$TRACE" == "true" ]]; then
    nvc --vhpi-trace -r "$tb" --load="$PLUGIN_SO" >"$LOG_FILE" 2>&1
  else
    nvc -r "$tb" --load="$PLUGIN_SO" >"$LOG_FILE" 2>&1
  fi

  popd >/dev/null

  for marker in "${EXPECTED_MARKERS[@]}"; do
    if ! grep -Eq "$marker" "$LOG_FILE"; then
      echo "${tb}: missing marker /${marker}/" >&2
      cat "$LOG_FILE" >&2
      exit 1
    fi
  done

  echo "${tb}: ok"
done

echo "[3/3] Completed ${#TEST_BENCHES[@]} dumper runs"
echo "Logs: ${WORK_ROOT}/${PLUGIN_CRATE}/<testbench>/run.log"
