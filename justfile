default:
	@just --list --unsorted

check:
	cargo check

build:
	cargo build

test:
	cargo test

run:
	cargo run -- --number 5

clean:
	rm -fr ./target
