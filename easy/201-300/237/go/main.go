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
	"regexp"
	"strings"
)

var zPrint = fmt.Println
var dictionaryPath = filepath.Join("..", "enable1.txt")

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

func findLongestWordGivenLetters(letters string, dictionary []string) (longestWord string) {
	wordPattern := regexp.MustCompile(fmt.Sprintf("^[%s]+$", letters))
	for _, word := range dictionary {
		if len(word) > len(longestWord) && wordPattern.MatchString(word) {
			longestWord = word
		}
	}
	return
}
