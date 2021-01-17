package main

import (
	"fmt"
	"math"
	"regexp"
	"strings"
)

var removeWrappingSpaceRegexp *regexp.Regexp = regexp.MustCompile(`^\s*(.*)\s*$`)

func main() {
	fmt.Println("hello world")
}

func getStarsForLine(lineNumber int) string {
	count := int(math.Pow(2, math.Max(0, float64(lineNumber - 1))))
	return strings.Repeat("@", count)
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

func rightJustifyStringSlice(stringSlice []string) []string {
	var result []string
	maxLength := findLengthOfLongestLine(stringSlice)
	for _, line := range stringSlice {
		result = append(
			result,
			strings.Repeat(" ", maxLength - len(line)) +
				removeWrappingSpaceRegexp.ReplaceAllString(line, "$1"),
		)
	}
	return result
}

// https://stackoverflow.com/a/19239850
func reverseStringSlice(stringSlice []string) []string {
	result := stringSlice
	for lead, tail := 0, len(result) - 1; lead < tail; lead, tail = lead + 1, tail - 1 {
		result[lead], result[tail] = result[tail], result[lead]
	}
	return result
}
