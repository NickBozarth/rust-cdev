.PHONY: all
all: build

.PHONY: build
build:
	cargo build --target aarch64-unknown-none --release
