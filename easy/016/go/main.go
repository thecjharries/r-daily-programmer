package main

import (
	"fmt"
	"regexp"
)

var specialCharactersPattern = regexp.MustCompile(`([\^\-\]])`)

func main() {
	fmt.Println("hello world")
}

func removeCharactersFromString(charactersToRemove, input string) string {
	sanitizedCharactersToRemove := specialCharactersPattern.ReplaceAllString(charactersToRemove, "\\$1")
	charactersToRemovePattern := regexp.MustCompile(fmt.Sprintf("[%s]", sanitizedCharactersToRemove))
	return charactersToRemovePattern.ReplaceAllString(input, "")
}
