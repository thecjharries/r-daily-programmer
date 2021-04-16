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

var novelPath = filepath.Join("..", "enable1.txt")

var promptPattern = regexp.MustCompile(`[".,:;!?()[\]{}]|[^".,:;!?()[\]{}\s]+`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func loadFileIntoString(filename string) string {
	byteContents, _ := ioutil.ReadFile(filename)
	return strings.Trim(string(byteContents), "\n")
}

func countPatternsInString(input string) map[string]int {
	results := promptPattern.FindAllString(input, -1)
	wordCount := make(map[string]int)
	for _, result := range results {
		_, exists := wordCount[result]
		if exists {
			wordCount[result] += 1
		} else {
			wordCount[result] = 1
		}
	}
	return wordCount
}
