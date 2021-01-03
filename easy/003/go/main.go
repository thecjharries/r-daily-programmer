package main

import (
	"fmt"
	"strings"
)

var alphabet string = "abcdefghijklmnopqrstuvwxyz"

func main() {
	fmt.Println("hello world")
	fmt.Println(rotateAlphabet(27))
}

func rotateAlphabet(places int) string {
	shift := places % len(alphabet)
	return alphabet[shift:] + alphabet[:shift]
}

func translateCharacter(character, newAlphabet string) string {
	characterIndex := strings.Index(alphabet, character)
	if -1 < characterIndex {
		return string(newAlphabet[characterIndex])
	}
	return ""
}
