run: compile
	./main

compile:
	rustc ./main.rs

bootstrap-feature-branch:
	git flow feature start $(lastword $(subst /, ,$(realpath ../../)))-$(lastword $(subst /, ,$(realpath ../)))
	git add ../README.md
	git commit -m 'Copy prompt'
	git add .
	git commit -m 'Copy Go boilerplate'

git-prompt-runner:
	git add main.go main_test.go
	git commit -m 'Add prompt runner to main'

finish:
	git flow feature finish
