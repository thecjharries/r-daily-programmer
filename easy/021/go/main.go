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
	"strconv"
)

func main() {
	input := 123
	permutations := permuteString(fmt.Sprint(input))
	fmt.Println(findNextLargestPermutation(input, convertPermutationsToInts(permutations)))
}

// Mostly from
// https://www.golangprograms.com/golang-program-to-print-all-permutations-of-a-given-string.html
func insertCharacterIntoEveryPositionOfString(existingString []rune, newCharacter rune) (permutations []string) {
	for index := 0; index <= len(existingString); index++ {
		permutations = append(
			permutations,
			string(existingString[:index]) + string(newCharacter) + string(existingString[index:]),
		)
	}
	return
}

// Mostly from
// https://www.golangprograms.com/golang-program-to-print-all-permutations-of-a-given-string.html
func permuteStringRecursion(remaining []rune, permutations []string) []string {
	if 0 == len(remaining) {
		return permutations
	}
	var newPermutations []string
	for _, permutation := range permutations {
		newPermutations = append(
			newPermutations,
			insertCharacterIntoEveryPositionOfString([]rune(permutation), remaining[0])...,
		)
	}
	return permuteStringRecursion(remaining[1:], newPermutations)
}

// Mostly from
// https://www.golangprograms.com/golang-program-to-print-all-permutations-of-a-given-string.html
func permuteString(stringToPermute string) []string {
	asRune := []rune(stringToPermute)
	return permuteStringRecursion(asRune[1:], []string{string(asRune[0])})
}

func convertPermutationsToInts(permutations []string) (results []int) {
	for _, permutation := range permutations {
		newInt, _ := strconv.ParseInt(permutation, 10, 0)
		results = append(results, int(newInt))
	}
	return
}

func findNextLargestPermutation(original int, permutations []int) (nextLargest int) {
	nextLargest = original
	sort.Ints(permutations)
	for _, permutation := range permutations {
		if nextLargest < permutation {
			nextLargest = permutation
			break
		}
	}
	return
}
