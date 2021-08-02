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
	"regexp"
	"sort"
	"strings"
	"unicode"
)

var whitespacePattern = regexp.MustCompile(`\s`)
var notLettersPattern = regexp.MustCompile(`(?i)([^a-z])`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func explodeOnWhitespace(input string) []string {
	return whitespacePattern.Split(input, -1)
}

func organizeWord(input string) string {
	// Find all the punctuation
	punctuationIndices := notLettersPattern.FindAllStringSubmatchIndex(input, -1)
	punctuationMatches := notLettersPattern.FindAllStringSubmatch(input, -1)
	// Sort the letters and convert back to string
	explodedOutput := strings.Split(strings.ToLower(notLettersPattern.ReplaceAllString(input, "")), "")
	sort.Strings(explodedOutput)
	lowerOutput := strings.Join(explodedOutput, "")
	// Reinject punctuation
	for matchIndex, indexGroup := range punctuationIndices {
		for index := 2; index < len(indexGroup); index += 2 {
			lowerOutput = fmt.Sprintf("%s%s%s", lowerOutput[:indexGroup[index]], punctuationMatches[matchIndex][index/2], lowerOutput[indexGroup[index]:])
		}
	}
	// Fix capital letters
	outputRune := []rune(lowerOutput)
	for index, character := range input {
		if unicode.IsUpper(character) {
			outputRune[index] = unicode.ToUpper(outputRune[index])
		}
	}
	return string(outputRune)
}
