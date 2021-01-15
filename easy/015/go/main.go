package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	fmt.Println("hello world")
}

// https://stackoverflow.com/a/16615559
func readFileIntoStringSlice(filename string) []string {
	var result []string
	file, _ := os.Open(filename)
	defer (func() { _ = file.Close() })()
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		result = append(result, scanner.Text())
	}
	return result
}

func findLengthOfLongestLine(stringSlice []string) int{
	maxLength := 0
	for _, line := range stringSlice {
		if len(line) > maxLength {
			maxLength = len(line)
		}
	}
	return maxLength
}
