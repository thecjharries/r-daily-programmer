package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

const resultFormatString string = "your name is %s, you are %s years old, and your username is %s"

func main() {
	age := getInput("Your age?", io.Reader(os.Stdin))
	fmt.Printf(resultFormatString, "", age, "")
}

func getInput(prompt string, source io.Reader) string {
	reader := bufio.NewReader(source)
	fmt.Println(prompt)
	input, _ := reader.ReadString('\n')
	return strings.Replace(input, "\n", "", -1)
}
