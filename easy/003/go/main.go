package main

import (
	"fmt"
	"strings"
)

var alphabet string = "abcdefghijklmnopqrstuvwxyz"

type CaesarCipher interface {
	Encrypt(input string, places int) string
	Decrypt(input string, places int) string
}

func main() {
	fmt.Println("hello world")
	translateString("test", "alphabet")
}

func rotateAlphabet(places int) string {
	shift := ((places % len(alphabet)) + len(alphabet)) % len(alphabet)
	return alphabet[shift:] + alphabet[:shift]
}

func translateCharacter(character, newAlphabet string) string {
	characterIndex := strings.Index(alphabet, character)
	if -1 < characterIndex {
		return string(newAlphabet[characterIndex])
	}
	return ""
}

func translateString(input, newAlphabet string) string {
	result := ""
	for _, character := range input {
		result += translateCharacter(string(character), newAlphabet)
	}
	return result
}
