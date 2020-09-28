.PHONY: watch

RUST_LOG ?= cargout=error

watch:
	cargo watch \
		--clear \
		--quiet \
		--watch \
		--shell \
			"\
				RUST_LOG=$(RUST_LOG) cargo run --quiet --color always -- \
					--file /data/projects/websocket-rs \
			"

build:
	cargo build --release
	rm -f /data/local/bin/cargout
	ln -s $(shell pwd)/target/release/cargout /data/local/bin/cargout
