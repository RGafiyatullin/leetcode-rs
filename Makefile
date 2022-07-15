.PHONY: test

test:
	cargo nextest run --no-fail-fast --release
