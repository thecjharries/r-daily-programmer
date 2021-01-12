package main

import (
	"bufio"
	"fmt"
	"io"
	"strings"
)

func main() {
	fmt.Println("hello world")
}

var zPrintFunction = fmt.Println

func getStringInput(prompt string, source io.Reader) string {
	reader := bufio.NewReader(source)
	fmt.Println(prompt)
	input, _ := reader.ReadString('\n')
	return strings.Replace(input, "\n", "", -1)
}

func permuteAndPrint(stringToPermute []rune, startingIndex int) {
	if len(stringToPermute) == startingIndex + 1 {
		_, _ = zPrintFunction(stringToPermute)
	}
	for movingIndex := startingIndex + 1; movingIndex < len(stringToPermute); movingIndex++ {
		stringToPermute[startingIndex], stringToPermute[movingIndex] = stringToPermute[movingIndex], stringToPermute[startingIndex]
		permuteAndPrint(stringToPermute, startingIndex + 1)
		stringToPermute[startingIndex], stringToPermute[movingIndex] = stringToPermute[movingIndex], stringToPermute[startingIndex]
	}
}
