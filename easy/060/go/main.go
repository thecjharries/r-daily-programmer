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
	number := 27
	fmt.Println(determinePolitenessSequences(number))
}

func isIntPolite(number int) bool {
	logBase2 := math.Log(float64(number)) / math.Log(2)
	return math.Ceil(logBase2) != math.Floor(logBase2) && number > 1
}

func createIntRangeInclusive(min, max int) (intRange []int) {
	intRange = make([]int, max - min + 1)
	for index := range intRange {
		intRange[index] = min + index
	}
	return
}

func determinePolitenessSequences(number int) (sequences [][]int) {
	if !isIntPolite(number) {
		return [][]int{}
	}
	for beginningIndex := 1; beginningIndex < (number / 2) + 1; beginningIndex++ {
		for endingIndex := beginningIndex + 1; endingIndex < number; endingIndex++ {
			sum := (beginningIndex + endingIndex) * (endingIndex - beginningIndex + 1) / 2
			if sum > number {
				break
			} else if sum == number {
				sequences = append(sequences, createIntRangeInclusive(beginningIndex, endingIndex))
			}
		}
	}
	return
}
