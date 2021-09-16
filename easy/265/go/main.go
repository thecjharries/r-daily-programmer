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

	"github.com/gitchander/permutation"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func generatePermutations(max int) (permutations [][]int) {
	var input []int
	for index := 0; index <= max; index++ {
		input = append(input, index)
	}
	perms := permutation.New(permutation.IntSlice(input))
	for perms.Next() {
		singlePerm := make([]int, len(input))
		copy(singlePerm, input)
		permutations = append(permutations, singlePerm)
	}
	return
}

func getSpecificPerm(max, permNumber int) []int {
	return generatePermutations(max)[permNumber-1]
}
