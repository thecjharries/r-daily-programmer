// Copyright 2021 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package main

import (
	"fmt"
	"math"
	"regexp"
	"strings"
)

var removeWrappingSpaceRegexp *regexp.Regexp = regexp.MustCompile(`^\s*(.*)\s*$`)

func main() {
	count := 5
	lines := generateLines(count)
	fmt.Printf("%s\n", strings.Join(lines, "\n"))
	fmt.Printf("%s\n", strings.Join(reverseStringSlice(lines), "\n"))
	fmt.Printf("%s\n", strings.Join(rightJustifyStringSlice(lines), "\n"))
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

func generateLines(maxLine int) []string {
	var result []string
	for lineNumber := 1; lineNumber <= maxLine; lineNumber++ {
		result = append(result, getStarsForLine(lineNumber))
	}
	return result
}
