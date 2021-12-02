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

func findRepeatedNumbers(input string) (repeated map[string]int) {
	discovery := make(map[string]int)
	repeated = make(map[string]int)
	for length := len(input) - 2; length > 1; length-- {
		for index := 0; index < len(input)-length; index++ {
			currentNumber := input[index : index+length]
			_, exists := discovery[currentNumber]
			if exists {
				discovery[currentNumber] += 1
				repeated[currentNumber] = discovery[currentNumber]
			} else {
				discovery[currentNumber] = 1
			}
		}
	}
	return
}
