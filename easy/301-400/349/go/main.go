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
	"strconv"

	combinations "github.com/mxschmitt/golang-combinations"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func convertStringSliceToInt(input []string) []int {
	var output []int
	for _, v := range input {
		i, _ := strconv.Atoi(v)
		output = append(output, i)
	}
	return output
}

func convertIntSliceToString(input []int) []string {
	var output []string
	for _, v := range input {
		s := strconv.Itoa(v)
		output = append(output, s)
	}
	return output
}

func sum(input []int) (sum int) {
	for _, v := range input {
		sum += v
	}
	return
}

func determineChange(target int, availableChange []int, maxCoinCount int) (output []int) {
	availableChangeString := convertIntSliceToString(availableChange)
	possibleCombinations := combinations.All(availableChangeString)
	for _, combination := range possibleCombinations {
		if len(combination) > maxCoinCount {
			continue
		}
		combinationInt := convertStringSliceToInt(combination)
		if sum(combinationInt) == target {
			output = combinationInt
			break
		}
	}
	return
}
