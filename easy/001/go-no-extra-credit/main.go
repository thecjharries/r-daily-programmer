package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

const resultFormatString string = "your name is %s, you are %s years old, and your username is %s"
var prompts = [3]string{
	"Your name?",
	"Your age?",
	"Your username?",
}


func main() {
	fmt.Println(buildResult(io.Reader(os.Stdin)))
}

func getInput(prompt string, source io.Reader) string {
	reader := bufio.NewReader(source)
	fmt.Println(prompt)
	input, _ := reader.ReadString('\n')
	return strings.Replace(input, "\n", "", -1)
}

func buildResult(source io.Reader) string {
	var results [3]string
	for index, prompt := range prompts {
		results[index] = getInput(prompt, source)
	}
	return fmt.Sprintf(resultFormatString, results[0], results[1], results[2])
}
