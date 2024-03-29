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
	total := 100
	items := []int{5, 75, 25}
	fmt.Printf("Discovered indices: ")
	fmt.Print(findFirstTwoItemsThatSumToTotal(total, items))
	fmt.Printf("\n")
}

func findFirstTwoItemsThatSumToTotal(total int, items []int) [2]int {
	for first := 0; first < len(items); first++ {
		for second := first + 1; second < len(items); second++ {
			if total == items[first]+items[second] {
				return [2]int{first, second}
			}
		}
	}
	return [2]int{}
}
