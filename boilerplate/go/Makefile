# Aliases for executables
AWK ?= awk
GIT ?= git
GO ?= go
GREP ?= grep
RM ?= rm
TEST ?= test

test:
	$(GO) test -v ./... -cover -race -coverprofile=.coverage.out

coverage: test
	$(GO) tool cover -func=.coverage.out

coverage-report: coverage
	$(GO) tool cover -html=.coverage.out

coverage-percentage: test
	$(TEST) "$(shell $(GO) tool cover -func=.coverage.out | $(GREP) total | $(AWK) '{ print substr($$3, 1, length($$3)-1); }')" = "100.0"

run:
	$(GO) run main.go

bootstrap-feature-branch:
	$(GIT) flow feature start $(lastword $(subst /, ,$(realpath ../../)))-$(lastword $(subst /, ,$(realpath ../)))
	$(GIT) add ../README.md
	$(GIT) commit -m 'Copy prompt'
	$(GIT) add .
	$(GIT) commit -m 'Copy Go boilerplate'

git-prompt-runner:
	$(GIT) add main.go main_test.go
	$(GIT) commit -m 'Add prompt runner to main'

clean:
	rm -rf .coverage.out

finish: coverage-percentage clean
	$(GIT) flow feature finish


stub:
	$(GIT) commit main.go -m 'Stub prompt fnc'
