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
	_, _ = zPrint(getAllUniqueStringPermutations("abbccc"))
}

func getAllStringPermutations(stringToPermute string, startingIndex int) (permutations []string) {
	if len(stringToPermute) <= startingIndex {
		return []string{stringToPermute}
	}
	stringAsRunes := []rune(stringToPermute)
	for movingIndex := startingIndex; movingIndex < len(stringAsRunes); movingIndex++ {
		stringAsRunes[startingIndex], stringAsRunes[movingIndex] = stringAsRunes[movingIndex], stringAsRunes[startingIndex]
		permutations = append(permutations, getAllStringPermutations(string(stringAsRunes), startingIndex+1)...)
		stringAsRunes[startingIndex], stringAsRunes[movingIndex] = stringAsRunes[movingIndex], stringAsRunes[startingIndex]
	}
	return
}

func getAllUniqueStringPermutations(stringToPermute string) (permutations []string) {
	nonuniquePermutations := getAllStringPermutations(stringToPermute, 0)
	permutationMap := make(map[string]bool, len(nonuniquePermutations))
	for _, permutation := range nonuniquePermutations {
		permutationMap[permutation] = true
	}
	for key := range permutationMap {
		permutations = append(permutations, key)
	}
	sort.Strings(permutations)
	return
}
