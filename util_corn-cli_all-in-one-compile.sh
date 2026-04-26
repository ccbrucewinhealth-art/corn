#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROFILE="release"
OUT_DIR="${ROOT_DIR}/bin/deploy"

parse_args() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --debug) PROFILE="debug"; shift ;;
      --out-dir) OUT_DIR="$2"; shift 2 ;;
      *) shift ;;
    esac
  done
}

load_env() {
  if [[ -f "${ROOT_DIR}/../.env" ]]; then
    set +e
    set -a; source "${ROOT_DIR}/../.env" >/dev/null 2>&1; set +a
    set -e
  elif [[ -f "${ROOT_DIR}/.env" ]]; then
    set +e
    set -a; source "${ROOT_DIR}/.env" >/dev/null 2>&1; set +a
    set -e
  fi
}

write_log() {
  local level="$1"; shift
  printf '[%s] %s\n' "$level" "$*"
}

run_main() {
  mkdir -p "$OUT_DIR"
  write_log INFO "all-in-one compile cron-cli profile=${PROFILE}"
  if [[ "$PROFILE" == "debug" ]]; then
    cargo build -p cron-cli --target-dir "${ROOT_DIR}/bin/target"
    cp bin/target/debug/cron-cli "$OUT_DIR/" || true
  else
    cargo build -p cron-cli --release --target-dir "${ROOT_DIR}/bin/target"
    cp bin/target/release/cron-cli "$OUT_DIR/" || true
  fi
  write_log INFO "cron-cli output at ${OUT_DIR}"
}

main() {
  parse_args "$@"
  load_env
  run_main
}

main "$@"
