package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"sort"
	"strings"
)

func main() {
	fmt.Println("hello world")
	input := getStringInput("Enter digits or letters to sort", os.Stdin)
	fmt.Println(sortInput(input))
}

func getStringInput(prompt string, source io.Reader) string {
	reader := bufio.NewReader(source)
	fmt.Println(prompt)
	input, _ := reader.ReadString('\n')
	return strings.Replace(input, "\n", "", -1)
}

func sortInput(input string) []string {
	exploded := strings.Split(input, "")
	sort.Strings(exploded)
	return exploded
}
