.DEFAULT_GOAL = help
SHELL=bash

## Run all the tests
all-tests:
	cargo test
	cargo test -p iwlib_sys

## Pre-check before publishing to crate
check:
	make all-tests
	cargo fmt --all -- --check
	cargo clippy -- -D warnings
	cargo check

## Check memory leaks using valgrind
valgrind:
	cargo valgrind

## Run the binary
run:
	cargo run

## Watch and run build
watch-build:
	cargo watch -x build

## Watch and run binary
watch-run:
	cargo watch -x run

## Show help screen.
help:
	@echo "Please use \`make <target>' where <target> is one of\n\n"
	@awk '/^[a-zA-Z\-\_0-9]+:/ { \
		helpMessage = match(lastLine, /^## (.*)/); \
		if (helpMessage) { \
			helpCommand = substr($$1, 0, index($$1, ":")); \
			helpMessage = substr(lastLine, RSTART + 3, RLENGTH); \
			printf "%-30s %s\n", helpCommand, helpMessage; \
		} \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST)
