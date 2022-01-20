DIFFICULTY = $(lastword $(subst /, ,$(realpath ../../../)))
NUMBER = $(lastword $(subst /, ,$(realpath ../)))

run:
	cargo run

test:
	cargo test

bootstrap-feature-branch:
	git flow feature start $(DIFFICULTY)-$(NUMBER)
	git add ./Makefile
	git commit -m 'Add boilerplate'
	git add ./src/main.rs
	git commit -m 'Create empty Rust file'
	sed -i 's/&REPLACE_ME&/$(DIFFICULTY)_$(NUMBER)/g' ./Cargo.toml
	git add ./Cargo.toml
	git commit -m 'Define Rust package'

finish:
	git flow feature finish

stub:
	git commit src/main.rs -m 'Stub prompt fnc'
