help:
	@echo make rust-version
	@echo make format
	@echo make lint
	@echo make test
	@echo make run
	@echo make release
	@echo make all

rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run -- --number 5

release:
	cargo build --release

all: format lint test run

clean:
	rm -fr ./target
