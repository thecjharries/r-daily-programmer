run:
	cargo run

test:
	cargo test

bootstrap-feature-branch:
	git flow feature start $(lastword $(subst /, ,$(realpath ../../../)))-$(lastword $(subst /, ,$(realpath ../)))
	git add ./Makefile
	git commit -m 'Add boilerplate'
	mkdir -p ./src
	touch ./src/main.rs
	git add ./src/main.rs
	git commit -m 'Create empty Rust file'
	echo '[package]\nname = "$(lastword $(subst /, ,$(realpath ../../../)))_$(lastword $(subst /, ,$(realpath ../)))"\nversion = "0.0.0"' > ./Cargo.toml
	git add ./Cargo.toml
	git commit -m 'Define Rust package'

finish:
	git flow feature finish
