.PHONY: setup
setup:
	cargo install diesel_cli --no-default-features --features postgres

.PHONY: db
db:
	diesel setup

.PHONY: test
test:
	cargo test