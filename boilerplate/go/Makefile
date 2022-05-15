test:
	go test -v ./... -cover -race -coverprofile=.coverage.out

coverage: test
	go tool cover -func=.coverage.out

coverage-report: coverage
	go tool cover -html=.coverage.out

coverage-percentage: test
	test "$(shell go tool cover -func=.coverage.out | grep total | awk '{ print substr($$3, 1, length($$3)-1); }')" = "100.0"

run:
	go run main.go

bootstrap-feature-branch:
	git flow feature start $(lastword $(subst /, ,$(realpath ../../)))-$(lastword $(subst /, ,$(realpath ../)))
	git add ../README.md
	git commit -m 'Copy prompt'
	git add .
	git commit -m 'Copy Go boilerplate'

git-prompt-runner:
	git add main.go main_test.go
	git commit -m 'Add prompt runner to main'

clean:
	rm -rf .coverage.out

finish: coverage-percentage clean
	git flow feature finish


stub:
	git commit main.go -m 'Stub prompt fnc'
