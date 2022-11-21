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

func determineBaumTerm(input int) int {
	if 0 >= input {
		return 1
	}
	asBinary := strconv.FormatInt(int64(input), 2)
	currentRunLength := 0
	for _, character := range asBinary {
		if '0' == character {
			currentRunLength++
		} else {
			if 1 == currentRunLength%2 {
				return 0
			}
			currentRunLength = 0
		}
	}
	if 1 == currentRunLength%2 {
		return 0
	}
	return 1
}

func buildBaumSequence(max int) (sequence []int) {
	for index := 0; index <= max; index++ {
		sequence = append(sequence, determineBaumTerm(index))
	}
	return
}
