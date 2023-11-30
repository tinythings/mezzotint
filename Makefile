.DEFAULT_GOAL := build

.PHONY:build
release: man
	cargo build -v --release

build:
	cargo build

man:
	pandoc --standalone --to man docs/manpages/mezzotint.8.md -o docs/manpages/mezzotint.8
