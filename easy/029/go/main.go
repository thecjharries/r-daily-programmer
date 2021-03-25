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
	"math"
)

func main() {
	fmt.Println("hello world")
}

func splitStringInTwoEqualPieces(input string) (string, string) {
	firstEnd := int(math.Floor(float64(len(input)) / 2))
	secondBeginning := int(math.Ceil(float64(len(input)) / 2))
	return input[:firstEnd], input[secondBeginning:]
}

func isPalindrome(input string) bool {
	first, second := splitStringInTwoEqualPieces(input)
	for index := 0; index < len(first); index++ {
		if first[index] != second[len(second)-index-1] {
			return false
		}
	}
	return true
}
