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

func main() {
	elements := []float64{18.1, 55.1, 91.2, 74.6, 73.0, 85.9, 73.9, 81.4, 87.1, 49.3, 88.8, 5.7, 26.3, 7.1, 58.2, 31.7, 5.8, 76.9, 16.5, 8.1, 48.3, 6.8, 92.4, 83.0, 19.6}
	maxSum := 98.2
	size := 3
	fmt.Println(ullmansPuzzle(elements, maxSum, size))
}

func doesSubsetSumLessThanMax(maxSum float64, subset []float64) bool {
	sum := float64(0)
	for _, element := range subset {
		sum += element
	}
	return maxSum > sum
}

func ullmansPuzzle(elements []float64, maxSum float64, size int) (bool, []float64) {
	sorted := make([]float64, len(elements))
	copy(sorted, elements)
	sort.Float64s(sorted)
	if doesSubsetSumLessThanMax(maxSum, sorted[0:size]) {
		return true, sorted[0:size]
	}
	return false, nil
}
