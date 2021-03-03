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
	fmt.Println("hello world")
}

func generateCombinations(size int, elements []float64) (combinations [][]float64) {
	if 1 == size {
		for _, element := range elements {
			combinations = append(combinations, []float64{element})
		}
		return
	}
	if len(elements) == size {
		return [][]float64{elements}
	}
	for _, combination := range generateCombinations(size - 1, elements[1:]) {
		combinations = append(combinations, append([]float64{elements[0]}, combination...))
	}
	combinations = append(combinations, generateCombinations(size, elements[1:])...)
	return
}

func doesSubsetSumLessThanMax(max float64, subset ...float64) bool {
	sum := float64(0)
	for _, element := range subset {
		sum += element
	}
	return max > sum
}
