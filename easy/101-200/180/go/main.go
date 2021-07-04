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
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func getNthLookAndSayIteration(n, seed int) int {
	if 2 > n {
		return seed
	}
	seedAsString := strconv.Itoa(seed)
	nextIterationAsString := ""
	currentNumber := rune(seedAsString[0])
	count := 1
	for _, element := range seedAsString[1:] {
		if currentNumber == element {
			count++
		} else {
			nextIterationAsString += fmt.Sprintf("%d%s", count, string(currentNumber))
			currentNumber = element
			count = 1
		}
	}
	nextIterationAsString += fmt.Sprintf("%d%s", count, string(currentNumber))
	nextIteration, _ := strconv.Atoi(nextIterationAsString)
	return getNthLookAndSayIteration(n-1, nextIteration)
}
