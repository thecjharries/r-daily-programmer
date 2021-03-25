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
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strings"
)

func main() {
	fmt.Println("hello world")
}

var removeWrappingSpaceRegexp *regexp.Regexp = regexp.MustCompile(`^\s*(.*)\s*$`)

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

func findLengthOfLongestLine(stringSlice []string) int {
	maxLength := 0
	for _, line := range stringSlice {
		if len(line) > maxLength {
			maxLength = len(line)
		}
	}
	return maxLength
}

func leftJustifyStringSlice(stringSlice []string) []string {
	var result []string
	for _, line := range stringSlice {
		result = append(result, removeWrappingSpaceRegexp.ReplaceAllString(line, "$1"))
	}
	return result
}

func rightJustifyStringSlice(stringSlice []string) []string {
	var result []string
	maxLength := findLengthOfLongestLine(stringSlice)
	for _, line := range stringSlice {
		result = append(
			result,
			strings.Repeat(" ", maxLength-len(line))+
				removeWrappingSpaceRegexp.ReplaceAllString(line, "$1"),
		)
	}
	return result
}
