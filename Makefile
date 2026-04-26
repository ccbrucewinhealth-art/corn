SHELL := /bin/bash

.PHONY: \
	build build-corn build-corn-cli \
	compile-corn compile-corn-cli \
	allinone-corn allinone-corn-cli \
	run run-corn run-corn-cli \
	run-corn-script loop-corn \
	debug-run debug-run-corn debug-run-corn-cli \
	svc cli fmt check run-unit-test docker-build

# ========= Build（編譯） =========
build:
	cargo build --workspace

build-corn:
	cargo build -p corn

build-corn-cli:
	cargo build -p cron-cli

compile-corn:
	./util_corn_compile.sh

compile-corn-cli:
	./util_corn-cli_compile.sh

allinone-corn:
	./util_corn_all-in-one-compile.sh

allinone-corn-cli:
	./util_corn-cli_all-in-one-compile.sh

# ========= Run（執行） =========
run: run-corn

run-corn:
	cargo run -p corn -- start

run-corn-cli:
	cargo run -p cron-cli -- jobs

run-corn-script:
	./util_corn.sh list

loop-corn:
	./util_corn-loop-exec.sh

# ========= Debug Run（除錯執行） =========
debug-run: debug-run-corn

debug-run-corn:
	RUST_LOG=debug cargo run -p corn -- svc --bind 0.0.0.0:8080

debug-run-corn-cli:
	RUST_LOG=debug cargo run -p cron-cli -- jobs

# ========= 既有別名（向後相容） =========
svc:
	$(MAKE) run-corn

cli:
	$(MAKE) run-corn-cli

fmt:
	cargo fmt --all

check:
	cargo check --workspace

run-unit-test:
	cargo test --workspace

docker-build:
	docker build -t corn:0.1.0 .
