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

import "fmt"

func main() {
	needle := "world"
	haystack := "hello world"
	fmt.Println(findFirstIndexOfNaively(haystack, needle))
}

func findFirstIndexOfNaively(haystack, needle string) int {
	if len(needle) > len(haystack) {
		return -1
	}
	for leadIndex := 0; leadIndex < len(haystack) - len(needle) + 1; leadIndex++ {
		needleMatches := true
		for positionIndex := 0; positionIndex < len(needle); positionIndex++ {
			if needle[positionIndex] != haystack[leadIndex + positionIndex] {
				needleMatches = false
				break
			}
		}
		if needleMatches {
			return leadIndex
		}
	}
	return -1
}
