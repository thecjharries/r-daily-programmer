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
	"io/ioutil"
	"sort"
	"strings"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func loadDictionary(filename string) []string {
	byteContents, _ := ioutil.ReadFile(filename)
	return strings.Split(
		strings.Trim(string(byteContents), "\n"),
		"\n",
	)
}

func convertToComparableSlice(word string) (exploded []string) {
	exploded = strings.Split(strings.ToLower(word), "")
	sort.Strings(exploded)
	return
}

func findPotentialWords(input string, dictionary []string) (possibleWords []string) {
	for _, word := range dictionary {
		if 5 > len(word) || input[0] != word[0] || input[len(input)-1] != word[len(word)-1] {
			continue
		}
		currentInput := input
		possibleMatch := true
		for wordIndex := 0; wordIndex < len(word); wordIndex++ {
			inputIndex := strings.Index(currentInput, word[wordIndex:wordIndex+1])
			if -1 < inputIndex {
				currentInput = currentInput[inputIndex+1:]
			} else {
				possibleMatch = false
				break
			}
		}
		if possibleMatch {
			possibleWords = append(possibleWords, word)
		}
	}
	return
}
