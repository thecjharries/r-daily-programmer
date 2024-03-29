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
	input := []int{4, 3, 2, 1, 5, 7, 6, 8, 9}
	windowSize := 3
	fmt.Println(getSlidingWindowMinimums(input, windowSize))
}

func getSlidingWindowMinimums(values []int, windowSize int) (minimums []int) {
	for windowStartIndex := 0; windowStartIndex < len(values)-windowSize+1; windowStartIndex++ {
		minimum := values[windowStartIndex]
		for windowIndex := 1; windowIndex < windowSize; windowIndex++ {
			if minimum > values[windowStartIndex+windowIndex] {
				minimum = values[windowStartIndex+windowIndex]
			}
		}
		minimums = append(minimums, minimum)
	}
	return
}
