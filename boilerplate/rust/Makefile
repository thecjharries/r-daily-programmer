# Aliases for executables
CARGO := cargo
GIT := git
SED := sed

# Variables
DIFFICULTY = $(lastword $(subst /, ,$(realpath ../../../)))
NUMBER = $(lastword $(subst /, ,$(realpath ../)))

run:
	$(CARGO) run

test:
	$(CARGO) test

bootstrap-feature-branch:
	$(GIT) flow feature start $(DIFFICULTY)-$(NUMBER)
	$(GIT) add ./Makefile
	$(GIT) commit -m 'Add boilerplate'
	$(GIT) add ./src/main.rs
	$(GIT) commit -m 'Create empty Rust file'
	$(SED) -i 's/&REPLACE_ME&/$(DIFFICULTY)_$(NUMBER)/g' ./Cargo.toml
	$(GIT) add ./Cargo.toml
	$(GIT) commit -m 'Define Rust package'

finish:
	$(GIT) flow feature finish

stub:
	$(GIT) commit src/main.rs -m 'Stub prompt fnc'

git-prompt-runner:
	$(GIT) add src/main.rs
	$(GIT) commit -m 'Add prompt runner to main'
