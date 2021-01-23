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

// https://stackoverflow.com/a/19239850
func reverseSort(slice []int) []int {
	result := slice
	for lead, tail := 0, len(result) - 1; lead < tail; lead, tail = lead + 1, tail -1 {
		result[lead], result[tail] = result[tail], result[lead]
	}
	return result
}

func reverseSortByBlock(slice []int, blockSize int) []int {
	var result []int
	for index := 0; index < len(slice); index += blockSize {
		sliceCap := int(math.Min(float64(index + blockSize), float64(len(slice))))
		result = append(result, reverseSort(slice[index:sliceCap])...)
	}
	return result
}
