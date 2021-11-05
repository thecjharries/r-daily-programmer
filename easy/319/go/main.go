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

func condenseStrings(input string) string {
	output := make([]string, 1)
	exploded := strings.Split(input, " ")
	output[0] = exploded[0]
	exploded = exploded[1:]
	for 1 < len(exploded) {
		currentWord := output[len(output)-1]
		nextWord := exploded[0]
		prefixIndex := 0
		for index := 0; index < len(currentWord) && index < len(nextWord); index++ {
			if strings.HasPrefix(nextWord, currentWord[len(currentWord)-1-index:]) {
				prefixIndex = index + 1
			}
		}
		if 0 < prefixIndex {
			output[len(output)-1] = currentWord + nextWord[prefixIndex:]
		} else {
			output = append(output, nextWord)
		}
		exploded = exploded[1:]
	}
	output = append(output, exploded...)
	return strings.Join(output, " ")
}
