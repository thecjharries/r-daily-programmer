package main

import (
	"fmt"
	"regexp"
)

func main() {
	fmt.Println("hello world")
}

func removeCharactersFromString(charactersToRemove, input string) string {
	charactersToRemovePattern := regexp.MustCompile(fmt.Sprintf("[%s]", charactersToRemove))
	return charactersToRemovePattern.ReplaceAllString(input, "")
}
