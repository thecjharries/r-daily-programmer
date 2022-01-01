bootstrap-feature-branch:
	git flow feature start $(lastword $(subst /, ,$(realpath ../../)))-$(lastword $(subst /, ,$(realpath ../)))
	git add ../README.md
	git commit -m 'Copy prompt'

main: clean
	rustc main.rs
	./main

clean:
	rm -f main
