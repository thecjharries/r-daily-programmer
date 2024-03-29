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

var romanNumeralsDecreasingOrder = []rune("MDCLXVI")

func main() {
	fmt.Println(isFirstLessThanSecondWithoutPrefixes("X", "C"))
}

func getNumeralPosition(numeral rune) int {
	for index, element := range romanNumeralsDecreasingOrder {
		if numeral == element {
			return index
		}
	}
	return -1
}

func isFirstLessThanSecondWithoutPrefixes(first, second string) bool {
	for index := 0; index < len(first) && index < len(second); index++ {
		firstIndex := getNumeralPosition(rune(first[index]))
		secondIndex := getNumeralPosition(rune(second[index]))
		if firstIndex > secondIndex {
			return true
		} else if firstIndex < secondIndex {
			return false
		}
	}
	return len(first) < len(second)
}
