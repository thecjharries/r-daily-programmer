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

var rule90 = map[string]string{
	"111": "0",
	"101": "0",
	"010": "0",
	"000": "0",
	"110": "1",
	"100": "1",
	"011": "1",
	"001": "1",
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func walkRule90(start string, iterations int) (results []string) {
	results = append(results, start)
	for iterationIndex := 0; iterationIndex < iterations; iterationIndex++ {
		currentState := "0" + results[iterationIndex] + "0"
		fmt.Println(currentState)
		nextState := ""
		for stateEndIndex := 3; stateEndIndex <= len(currentState); stateEndIndex++ {
			character, exists := rule90[currentState[stateEndIndex-3:stateEndIndex]]
			if exists {
				nextState += character
				fmt.Println(currentState[stateEndIndex-3:stateEndIndex], character)
			}
		}
		results = append(results, nextState)
	}
	return
}
