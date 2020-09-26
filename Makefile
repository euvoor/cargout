watch:
	cargo watch \
		--clear \
		--quiet \
		--watch \
		--shell \
			"\
				cargo run --quiet --color always -- \
				--file /data/projects/websocket-rs \
			"
