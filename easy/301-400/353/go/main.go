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
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func determineHammingDistance(first, second string) (distance int) {
	for i := 0; i < len(first); i++ {
		if first[i] != second[i] {
			distance++
		}
	}
	return
}

func findClosestString(input []string) (output string) {
	lowestDistance := 1<<31 - 1
	for walkingIndex, walkingElement := range input {
		currentDistance := 0
		for innerIndex := 0; innerIndex < len(input); innerIndex++ {
			if walkingIndex == innerIndex {
				continue
			}
			currentDistance += determineHammingDistance(walkingElement, input[innerIndex])
		}
		fmt.Println(walkingElement, currentDistance)
		if currentDistance < lowestDistance {
			lowestDistance = currentDistance
			output = walkingElement
		}
	}
	return
}
