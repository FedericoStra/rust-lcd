.PHONY: all build install

all:
	@echo "Run 'sudo make install' to install 'rust-lcd'"

build:
	cargo build --release

install: build
	install -o root -g root -m 4755 -t /usr/local/bin target/release/rust-lcd
