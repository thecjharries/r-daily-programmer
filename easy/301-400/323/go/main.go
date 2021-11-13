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

func threeSum(input []int) (results [][]int) {
	sums := make(map[[3]int]struct{})
	for firstIndex, first := range input[:len(input)-2] {
		for secondIndex, second := range input[firstIndex : len(input)-1] {
			for _, third := range input[secondIndex:] {
				if first+second+third == 0 {
					newSum := []int{first, second, third}
					sort.Ints(newSum)
					_, exists := sums[[3]int{newSum[0], newSum[1], newSum[2]}]
					if !exists {
						sums[[3]int{newSum[0], newSum[1], newSum[2]}] = struct{}{}
						results = append(results, newSum)
					}
				}
			}
		}
	}
	return
}
