# Aliases for executables
CARGO ?= cargo
CODE ?= code
GH ?= gh
GIT ?= git
SED ?= sed
XDG_OPEN ?= xdg-open
RM ?= rm

# Variables
DIFFICULTY = $(lastword $(subst /, ,$(realpath ../../)))
NUMBER = $(lastword $(subst /, ,$(realpath ../)))

# Dump variables
.PHONY: debug
debug:
	@echo "Exercise is $(DIFFICULTY)-$(NUMBER)"
	@echo "CARGO is $(CARGO)"
	@echo "GIT is $(GIT)"
	@echo "SED is $(SED)"
	@echo "XDG_OPEN is $(XDG_OPEN)"
	@echo "RM is $(RM)"

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
	$(GIT) add ../README.md
	$(GIT) commit -m 'Define $(DIFFICULTY) #$(NUMBER)'

.PHONY: boot
boot: add-exercise bootstrap-feature-branch

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
	$(CODE) --reuse-window ./src/main.rs

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

# Convenience target to finish the feature branch
.PHONY: finish
finish: coverage clean
	$(GIT) push -u origin feat/$(DIFFICULTY)-$(NUMBER)
	$(GH) pr create --fill
	$(GH) pr merge --merge --delete-branch

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
