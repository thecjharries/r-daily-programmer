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
	"path/filepath"
	"strings"
)

var dictionaryPath = filepath.Join("..", "selected four-letter words.txt")
var romanAlphabet = "abcdefghijklmnopqrstuvwxyz"

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

func convertDictionaryToMap(dictionary []string) (dictionaryAsMap map[string]bool) {
	dictionaryAsMap = make(map[string]bool, len(dictionary))
	for _, word := range dictionary {
		dictionaryAsMap[word] = true
	}
	return
}

func findWordLadderSiblings(word string, dictionaryAsMap map[string]bool) (siblings []string) {
	for wordIndex := 0; wordIndex < len(word); wordIndex++ {
		for alphabetIndex := 0; alphabetIndex < len(romanAlphabet); alphabetIndex++ {
			newWord := word[:wordIndex] + string(romanAlphabet[alphabetIndex]) + word[wordIndex+1:]
			_, exists := dictionaryAsMap[newWord]
			if exists {
				siblings = append(siblings, newWord)
			}
		}
	}
	return
}
