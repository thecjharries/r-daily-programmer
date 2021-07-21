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
	_, _ = zPrint(cullNonUniqueNumbers([]int{3, 1, 3, 4, 4, 1, 4, 5, 2, 1, 4, 4, 4, 4, 1, 4, 3, 2, 5, 5, 2, 2, 2, 4, 2, 4, 4, 4, 4, 1}))
}

func cullNonUniqueNumbers(input []int) (output []int) {
	uniqueNumbers := make(map[int]struct{})
	for _, element := range input {
		uniqueNumbers[element] = struct{}{}
	}
	for key, _ := range uniqueNumbers {
		output = append(output, key)
	}
	sort.Ints(output)
	return
}
