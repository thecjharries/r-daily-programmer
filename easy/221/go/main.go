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

func convertToWordSnake(input []string) string {
	outputRows := make([]string, 1)
	currentSpacing := 0
	for index, word := range input {
		if 0 == index%2 {
			outputRows[len(outputRows)-1] = strings.Repeat(" ", currentSpacing) + word
			currentSpacing += len(word) - 1
		} else {
			for _, character := range word[1:] {
				outputRows = append(outputRows, strings.Repeat(" ", currentSpacing)+string(character))
			}
		}
	}
	return strings.Join(outputRows, "\n")
}
