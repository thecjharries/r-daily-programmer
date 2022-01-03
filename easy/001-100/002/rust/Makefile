run:
	cargo run

test:
	cargo test

bootstrap-feature-branch:
	git flow feature start $(lastword $(subst /, ,$(realpath ../../../)))-$(lastword $(subst /, ,$(realpath ../)))
	git add ./Makefile
	git commit -m 'Add boilerplate'

finish:
	git flow feature finish
