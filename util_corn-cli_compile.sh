#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROFILE="release"

parse_args() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --debug) PROFILE="debug"; shift ;;
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
  write_log INFO "compile cron-cli profile=${PROFILE}"
  if [[ "$PROFILE" == "debug" ]]; then
    cargo build -p cron-cli --target-dir "${ROOT_DIR}/bin/target"
  else
    cargo build -p cron-cli --release --target-dir "${ROOT_DIR}/bin/target"
  fi
  write_log INFO "compile cron-cli done"
}

main() {
  parse_args "$@"
  load_env
  run_main
}

main "$@"
