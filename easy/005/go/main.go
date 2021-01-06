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

func getStringInput(prompt string, source io.Reader) string {
	reader := bufio.NewReader(source)
	fmt.Println(prompt)
	input, _ := reader.ReadString('\n')
	return strings.Replace(input, "\n", "", -1)
}
