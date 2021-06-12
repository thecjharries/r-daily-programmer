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
)

var promptWordPattern = regexp.MustCompile(`[a-zA-Z0-9]+`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func findAllWords(haystack string) []string {
	matches := promptWordPattern.FindAllString(haystack, -1)
	return matches
}

func getWordAtIndex(index int, haystack []string) string {
	if 0 <= index-1 && len(haystack) > index-1 {
		return haystack[index-1]
	}
	return ""
}
