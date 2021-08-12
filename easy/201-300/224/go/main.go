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
	"math/rand"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func shuffleIntSlice(input []int) (output []int) {
	inputReduced := make([]int, len(input))
	copy(inputReduced, input)
	for 0 < len(inputReduced) {
		index := rand.Intn(len(inputReduced))
		output = append(output, inputReduced[index])
		inputReduced = append(inputReduced[:index], inputReduced[index+1:]...)
	}
	return
}

func shuffleIntSliceFisherYaters(input []int) []int {
	output := make([]int, len(input))
	copy(output, input)
	for index := len(output) - 1; 0 < index; index-- {
		swappedIndex := rand.Intn(index)
		output[index], output[swappedIndex] = output[swappedIndex], output[index]
	}
	return output
}
