.PHONY: test-debug test-release all

all: test-release test-debug

test-debug:
	cargo nextest run --no-fail-fast

test-release:
	cargo nextest run --no-fail-fast --release
