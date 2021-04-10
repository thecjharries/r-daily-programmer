test:
	go test -v ./... -cover -race -coverprofile=.coverage.out

coverage: test
	go tool cover -func=.coverage.out

run:
	go run main.go
