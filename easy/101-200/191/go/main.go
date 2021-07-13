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
	"strings"
)

var wordPattern = regexp.MustCompile(`\w+`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func parseWords(input string) []string {
	return wordPattern.FindAllString(strings.ToLower(input), -1)
}

func countWords(words []string) map[string]int {
	count := make(map[string]int)
	for _, word := range words {
		currentCount, exists := count[word]
		if exists {
			count[word] = currentCount + 1
		} else {
			count[word] = 1
		}
	}
	return count
}
