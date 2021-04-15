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
	"sort"
	"strings"
)

var dictionaryPath = filepath.Join("..", "enable1.txt")

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

func convertDictionaryToSortedLetterMap(dictionary []string) map[string][]string {
	sortedLetterMap := make(map[string][]string)
	for _, word := range dictionary {
		exploded := strings.Split(word, "")
		sort.Strings(exploded)
		sortedWord := strings.Join(exploded, "")
		_, exists := sortedLetterMap[sortedWord]
		if exists {
			sortedLetterMap[sortedWord] = append(sortedLetterMap[sortedWord], word)
		} else {
			sortedLetterMap[sortedWord] = []string{word}
		}
	}
	return sortedLetterMap
}
