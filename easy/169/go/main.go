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

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func rotate2dArray90(input [][]int) (output [][]int) {
	output = make([][]int, len(input))
	for rowIndex := 0; rowIndex < len(input); rowIndex++ {
		output[rowIndex] = make([]int, len(input[rowIndex]))
		for columnIndex := 0; columnIndex < len(input[rowIndex]); columnIndex++ {
			output[rowIndex] = append(output[rowIndex], input[len(input[rowIndex])-rowIndex-1][columnIndex])
		}
	}
	return
}
