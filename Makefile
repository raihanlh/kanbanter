.PHONY: run-dev pwd

run-dev:
	export CARGO_MANIFEST_DIR=$(shell pwd)/src-tauri && cargo tauri dev

run-dev-reset:
	rm src-tauri/sqlite.db* || true
	export CARGO_MANIFEST_DIR=$(shell pwd)/src-tauri && cargo tauri dev

run-build:
	export CARGO_MANIFEST_DIR=$(shell pwd)/src-tauri && cargo tauri build