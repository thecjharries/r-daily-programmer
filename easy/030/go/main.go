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

func main() {
	getFirstTwoElementsThatSumToTarget([]int{1, 2, 3, 4}, 4)
}

// Returns true if a and b sum to the target
func doesInputSumToTarget(a, b, target int) bool {
	return target == a+b
}

func getFirstTwoElementsThatSumToTarget(input []int, target int) (found [2]int) {
	for firstIndex, first := range input {
		for _, second := range input[firstIndex+1:] {
			if doesInputSumToTarget(first, second, target) {
				return [2]int{first, second}
			}
		}
	}
	return
}
