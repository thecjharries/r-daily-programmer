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
	values := []int{31, -41, 59, 26, -53, 58, 97, -93, -23, 84}
	fmt.Println(kadanesAlgorithm(values))
}

// https://en.wikipedia.org/wiki/Maximum_subarray_problem#Kadane's_algorithm
func kadanesAlgorithm(values []int) []int {
	var maxSum, maxStartIndex, maxEndIndex, currentSum, currentStartIndex int
	currentSum = int(math.Inf(-1))
	for currentEndIndex, currentValue := range values {
		if currentSum <= 0 {
			if currentSum < currentValue {
				currentSum = currentValue
				currentStartIndex = currentEndIndex
				maxStartIndex, maxEndIndex = currentStartIndex, currentEndIndex+1
			}
		} else {
			currentSum += currentValue
		}
		if maxSum <= currentSum {
			maxSum = currentSum
			maxStartIndex, maxEndIndex = currentStartIndex, currentEndIndex+1
		}
	}
	return values[maxStartIndex:maxEndIndex]
}
