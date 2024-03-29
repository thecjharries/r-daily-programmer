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
	input := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
	fmt.Println(evenToTheFrontOddToTheBack(input))
}

func evenToTheFrontOddToTheBack(input []int) []int {
	rightIndex, leftIndex := 0, len(input)-1
	for rightIndex < leftIndex {
		for 0 == input[rightIndex]%2 {
			rightIndex++
		}
		for 1 == input[leftIndex]%2 {
			leftIndex--
		}
		if rightIndex < leftIndex {
			input[rightIndex], input[leftIndex] = input[leftIndex], input[rightIndex]
		}
	}
	return input
}
