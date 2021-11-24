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

func calculateConsecutiveDistanceRating(input []int) (rating int) {
	sortedInput := make([]int, len(input))
	copy(sortedInput, input)
	sort.Ints(sortedInput)
	indexMap := make(map[int]int)
	for index, value := range input {
		indexMap[value] = index
	}
	for index := 1; index < len(sortedInput); index++ {
		if sortedInput[index]-sortedInput[index-1] == 1 {
			if indexMap[sortedInput[index]] > indexMap[sortedInput[index-1]] {
				rating += indexMap[sortedInput[index]] - indexMap[sortedInput[index-1]]
			} else {
				rating += indexMap[sortedInput[index-1]] - indexMap[sortedInput[index]]
			}
		}
	}
	return
}
