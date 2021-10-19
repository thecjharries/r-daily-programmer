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
	"strings"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func removeExtraneousParentheses(input string) (output string) {
	var openIndices, removeIndices []int
	previousOpen, previousClose := -3, -3
	for index, character := range input {
		if '(' == character {
			openIndices = append(openIndices, index)
			fmt.Println(openIndices)
		} else if ')' == character {
			if openIndices[len(openIndices)-1] == index-1 {
				removeIndices = append(removeIndices, index, index-1)
			} else if openIndices[len(openIndices)-1] == previousOpen-1 && index == previousClose+1 {
				removeIndices = append(removeIndices, openIndices[len(openIndices)-1], index)
			} else {
				fmt.Println(openIndices[len(openIndices)-1], previousOpen, index, previousClose)
			}
			previousOpen = openIndices[len(openIndices)-1]
			previousClose = index
			openIndices = openIndices[:len(openIndices)-1]
		}
	}
	fmt.Println(removeIndices)
	output = input
	for _, removeIndex := range removeIndices {
		output = output[:removeIndex] + " " + output[removeIndex+1:]
	}
	return strings.ReplaceAll(output, " ", "")
}
