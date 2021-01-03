package main

import "fmt"

var alphabet string = "abcdefghijklmnopqrstuvwxyz"

func main() {
	fmt.Println("hello world")
	fmt.Println(rotateAlphabet(27))
}

func rotateAlphabet(places int) string {
	shift := places % len(alphabet)
	return alphabet[shift:] + alphabet[:shift]
}
