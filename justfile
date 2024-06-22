default:
	@just --list --unsorted

check:
	cargo check

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

run:
	cargo run -- --number 5

clean:
	rm -fr ./target
