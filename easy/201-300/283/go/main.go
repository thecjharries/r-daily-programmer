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
)

var notLettersPattern = regexp.MustCompile(`[^a-z]`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func checkIfAnagram(first, second string) bool {
	firstExploded := strings.Split(notLettersPattern.ReplaceAllString(strings.ToLower(first), ""), "")
	secondExploded := strings.Split(notLettersPattern.ReplaceAllString(strings.ToLower(second), ""), "")
	if len(firstExploded) != len(secondExploded) {
		return false
	}
	sort.Strings(firstExploded)
	sort.Strings(secondExploded)
	for index := 0; index < len(firstExploded); index++ {
		if firstExploded[index] != secondExploded[index] {
			return false
		}
	}
	return true
}
