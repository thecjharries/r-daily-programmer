test:
	go test -v ./... -cover -race -coverprofile=.coverage.out

coverage: test
	go tool cover -func=.coverage.out

run:
	go run main.go

bootstrap-feature-branch:
	git flow feature start $(lastword $(subst /, ,$(realpath ../../)))-$(lastword $(subst /, ,$(realpath ../)))
	git add ../README.md
	git commit -m 'Copy prompt'
	git add .
	git commit -m 'Copy Go boilerplate'
