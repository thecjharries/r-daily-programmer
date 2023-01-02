# Aliases for executables
CARGO ?= cargo
GIT ?= git
SED ?= sed
XDG_OPEN ?= xdg-open
RM ?= rm

# Variables
DIFFICULTY = $(lastword $(subst /, ,$(realpath ../../../)))
NUMBER = $(lastword $(subst /, ,$(realpath ../)))

# Run the program
.PHONY: run
run:
	$(CARGO) run

# Run the tests
.PHONY: test
test:
	$(CARGO) test

# Get code coverage
.PHONY: coverage
coverage:
	$(CARGO) tarpaulin -v --fail-under=100

# Build coverage report
.PHONY: coverage-report
coverage-report:
	$(CARGO) tarpaulin -v --fail-under=100 --out HTML; $(XDG_OPEN) tarpaulin-report.html

.PHONY: add-exercise
add-exercise:
	git add ..
	git commit -m 'Define $(DIFFICULTY) #$(NUMBER)'

# Set up the daily exercise
.PHONY: bootstrap-feature-branch
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
.PHONY: finish
finish: coverage clean
	cd ..; $(GIT) flow feature finish

# Convenience target for common commit
# Commits a primary stub for testing
.PHONY: stub
stub:
	$(GIT) commit src/main.rs -m 'Stub prompt fnc'

# Convenience target for common commit
# Commits main method updated with prompt task
.PHONY: git-prompt-runner
git-prompt-runner:
	$(GIT) add src/main.rs
	$(GIT) commit -m 'Add prompt runner to main'

# Convenience target for making a patch commit on main.rs
.PHONY: patch
patch:
	$(GIT) add src --patch

# Remove any built artifacts
.PHONY: clean
clean:
	$(RM) -rf target
	$(RM) -rf Cargo.lock
	$(RM) -rf tarpaulin-report.html
	$(RM) -rf main
