# Aliases for executables
CARGO := cargo
GIT := git
SED := sed

# Variables
DIFFICULTY = $(lastword $(subst /, ,$(realpath ../../../)))
NUMBER = $(lastword $(subst /, ,$(realpath ../)))

# Run the program
run:
	$(CARGO) run

# Run the tests
test:
	$(CARGO) test

# Set up the daily exercise
bootstrap-feature-branch:
	$(GIT) flow feature start $(DIFFICULTY)-$(NUMBER)
	$(GIT) add ./Makefile
	$(GIT) commit -m 'Add boilerplate'
	$(GIT) add ./src/main.rs
	$(GIT) commit -m 'Create empty Rust file'
	$(SED) -i 's/&REPLACE_ME&/$(DIFFICULTY)_$(NUMBER)/g' ./Cargo.toml
	$(GIT) add ./Cargo.toml
	$(GIT) commit -m 'Define Rust package'

# Shortcut to finish the exercise branch
# Probably better as an alias; this works just fine
finish:
	$(GIT) flow feature finish

# Convenience target for common commit
# Commits a primary stub for testing
stub:
	$(GIT) commit src/main.rs -m 'Stub prompt fnc'

# Convenience target for common commit
# Commits main method updated with prompt task
git-prompt-runner:
	$(GIT) add src/main.rs
	$(GIT) commit -m 'Add prompt runner to main'
