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
	"sort"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func cannibalizeNumbers(queryNumber int, numbers []int) (result int) {
	sortedNumbers := make([]int, len(numbers))
	copy(sortedNumbers, numbers)
	sort.Ints(sortedNumbers)
	bottomIndex, topIndex := 0, len(sortedNumbers)-1
	for bottomIndex < topIndex {
		if queryNumber <= sortedNumbers[topIndex] {
			result++
		} else {
			for queryNumber > sortedNumbers[topIndex] && bottomIndex < topIndex {
				bottomIndex++
				sortedNumbers[topIndex]++
			}
			if queryNumber == sortedNumbers[topIndex] {
				result++
			}
		}
		topIndex--
	}
	return
}
