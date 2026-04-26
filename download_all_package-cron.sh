#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
OUT_DIR="${ROOT_DIR}/../package/rust/modules"

parse_args() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --out-dir) OUT_DIR="$2"; shift 2;;
      *) shift;;
    esac
  done
}

load_env() {
  if [[ -f "${ROOT_DIR}/../.env" ]]; then
    set -a; source "${ROOT_DIR}/../.env"; set +a
  elif [[ -f "${ROOT_DIR}/.env" ]]; then
    set -a; source "${ROOT_DIR}/.env"; set +a
  fi
}

write_log() {
  local level="$1"; shift
  printf '[%s] %s\n' "$level" "$*"
}

run_main() {
  mkdir -p "$OUT_DIR"
  write_log INFO "download crates to ${OUT_DIR}"
  cargo fetch
  write_log INFO "cargo fetch done"
}

main() {
  parse_args "$@"
  load_env
  run_main
}

main "$@"
