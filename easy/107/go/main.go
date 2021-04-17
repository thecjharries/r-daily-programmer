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

var romanAlphabet = "abcdefghijklmnopqrstuvwxyz"

func main() {
	_, _ = zPrint("hello world")
}

func iteratePossiblePlaintext(number, workingPlaintext string) []string {
	if 0 == len(number) {
		return []string{workingPlaintext}
	}
	var newPossibilities []string
	var newWorkingPlaintext string
	var currentCode int
	currentCode, _ = strconv.Atoi(string(number[0]))
	newWorkingPlaintext = workingPlaintext + string(romanAlphabet[currentCode]-1)
	newPossibilities = append(newPossibilities, iteratePossiblePlaintext(number[1:], newWorkingPlaintext)...)
	if 1 < len(number) {
		currentCode, _ = strconv.Atoi(number[0:2])
		if currentCode < 27 {
			newWorkingPlaintext = workingPlaintext + string(romanAlphabet[currentCode]-1)
			newPossibilities = append(newPossibilities, iteratePossiblePlaintext(number[2:], newWorkingPlaintext)...)
		}
	}
	return newPossibilities
}
