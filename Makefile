BIN_NAME := graphol
PREFIX ?= /usr/local
BINDIR := $(PREFIX)/bin

.PHONY: build test install uninstall

build:
	cargo build --release

test:
	cargo test

install:
	install -Dm755 target/release/$(BIN_NAME) $(BINDIR)/$(BIN_NAME)

uninstall:
	rm -f $(BINDIR)/$(BIN_NAME)
