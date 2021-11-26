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
	"strconv"
	"strings"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func isValid(number string) bool {
	return !strings.HasPrefix(number, "0")
}

func buildLetterSet(words []string) (uniqueLetters []string) {
	letters := make(map[string]struct{})
	for _, word := range words {
		for index := 0; index < len(word); index++ {
			_, exists := letters[string(word[index])]
			if !exists {
				letters[string(word[index])] = struct{}{}
				uniqueLetters = append(uniqueLetters, string(word[index]))
			}
		}
	}
	return
}

func buildLetterMapping(letters, values []string) map[string]string {
	mapping := make(map[string]string)
	for index, letter := range letters {
		mapping[letter] = values[index]
	}
	return mapping
}

func convertWordToNumber(word string, mapping map[string]string) int {
	wordNumber := strings.Map(func(r rune) rune {
		return []rune(mapping[string(r)])[0]
	}, word)
	number, _ := strconv.Atoi(wordNumber)
	return number
}

func isMappingValid(words []string, mapping map[string]string) bool {
	if !isValid(words[len(words)-1]) {
		return false
	}
	workingSum := 0
	for _, word := range words[:len(words)-1] {
		if !isValid(word) {
			return false
		}
		workingSum += convertWordToNumber(word, mapping)
	}
	return workingSum == convertWordToNumber(words[len(words)-1], mapping)
}
