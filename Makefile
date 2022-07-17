.PHONY: test

test-debug:
	cargo nextest run --no-fail-fast

test-release:
	cargo nextest run --no-fail-fast --release
