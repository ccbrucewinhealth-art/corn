#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
INTERVAL_SEC=5
COMMAND="cargo run -p corn -- list"

parse_args() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --interval) INTERVAL_SEC="$2"; shift 2 ;;
      --cmd) COMMAND="$2"; shift 2 ;;
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
  write_log INFO "loop start interval=${INTERVAL_SEC}s cmd=${COMMAND}"
  while true; do
    if eval "$COMMAND"; then
      write_log INFO "loop run ok"
    else
      write_log ERROR "loop run failed"
    fi
    sleep "$INTERVAL_SEC"
  done
}

main() {
  parse_args "$@"
  load_env
  run_main
}

main "$@"
