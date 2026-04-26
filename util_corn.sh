#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
SUB_COMMAND="list"
TARGET=""

parse_args() {
  if [[ $# -gt 0 ]]; then
    SUB_COMMAND="$1"
    shift
  fi
  if [[ $# -gt 0 ]]; then
    TARGET="$1"
    shift
  fi
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
  write_log INFO "execute corn sub_command=${SUB_COMMAND} target=${TARGET:-N/A}"
  if [[ -n "$TARGET" ]]; then
    cargo run -p corn -- "$SUB_COMMAND" "$TARGET"
  else
    cargo run -p corn -- "$SUB_COMMAND"
  fi
  write_log INFO "execute corn done"
}

main() {
  parse_args "$@"
  load_env
  run_main
}

main "$@"
