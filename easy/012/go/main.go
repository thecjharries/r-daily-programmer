package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

func main() {
	input := getStringInput("String to permute?", os.Stdin)
	permuteAndPrint([]rune(input), 0)
}

var zPrintFunction = fmt.Println

func getStringInput(prompt string, source io.Reader) string {
	reader := bufio.NewReader(source)
	fmt.Println(prompt)
	input, _ := reader.ReadString('\n')
	return strings.Replace(input, "\n", "", -1)
}

func permuteAndPrint(stringToPermute []rune, startingIndex int) {
	if len(stringToPermute) <= startingIndex {
		_, _ = zPrintFunction(string(stringToPermute))
		return
	}
	for movingIndex := startingIndex; movingIndex < len(stringToPermute); movingIndex++ {
		stringToPermute[startingIndex], stringToPermute[movingIndex] = stringToPermute[movingIndex], stringToPermute[startingIndex]
		permuteAndPrint(stringToPermute, startingIndex + 1)
		stringToPermute[startingIndex], stringToPermute[movingIndex] = stringToPermute[movingIndex], stringToPermute[startingIndex]
	}
}
