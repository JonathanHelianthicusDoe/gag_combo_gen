.PHONY: debug release native

debug:
	cargo build

release:
	cargo rustc --release -- -C target-feature=+crt-static

native:
	cargo rustc --release -- -C target-cpu=native
