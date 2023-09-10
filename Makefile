.DEFAULT_GOAL := build

.PHONY:build
release: man
	cargo build -v --release

build:
	cargo build

man:
	pandoc --standalone --to man doc/manpages/mezzotint.8.md -o doc/manpages/mezzotint.8
