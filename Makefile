.PHONY: run-dev pwd

run-dev:
	export CARGO_MANIFEST_DIR=$(shell pwd)/src-tauri && cargo tauri dev