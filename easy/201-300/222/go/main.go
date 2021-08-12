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
	"unicode"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func getLetterValue(input rune) int {
	switch unicode.ToLower(input) {
	case 'a':
		return 1
	case 'b':
		return 2
	case 'c':
		return 3
	case 'd':
		return 4
	case 'e':
		return 5
	case 'f':
		return 6
	case 'g':
		return 7
	case 'h':
		return 8
	case 'i':
		return 9
	case 'j':
		return 10
	case 'k':
		return 11
	case 'l':
		return 12
	case 'm':
		return 13
	case 'n':
		return 14
	case 'o':
		return 15
	case 'p':
		return 16
	case 'q':
		return 17
	case 'r':
		return 18
	case 's':
		return 19
	case 't':
		return 20
	case 'u':
		return 21
	case 'v':
		return 22
	case 'w':
		return 23
	case 'x':
		return 24
	case 'y':
		return 25
	case 'z':
		return 26
	default:
		return 0
	}
}

func balanceWord(word string) ([3]string, int) {
	leftSum, rightSum := 0, 0
	leftWeight, rightWeight := 0, 0
	leftIndex, rightIndex := 0, len(word)-1
	for leftIndex < rightIndex {
		if leftWeight < rightWeight {
			leftSum += getLetterValue(rune(word[leftIndex]))
			leftWeight += leftSum
			leftIndex++
		} else {
			rightSum += getLetterValue(rune(word[rightIndex]))
			rightWeight += rightSum
			rightIndex--
		}
	}
	if leftIndex == rightIndex && leftWeight == rightWeight {
		return [3]string{word[:leftIndex], word[leftIndex : rightIndex+1], word[rightIndex+1:]}, leftWeight
	}
	return [3]string{}, -1
}
