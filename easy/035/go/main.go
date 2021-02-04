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
	"strings"
)

func main() {
	triangle := intToRightTriangle(11)
	for _, line := range triangle {
		fmt.Println(line)
	}
}

func intToRightTriangle(input int) (triangle []string) {
	var currentRow []string
	lineNumber := 1
	for index := 1; index <= input; index++ {
		currentRow = append(currentRow, strconv.Itoa(index))
		if lineNumber == len(currentRow) {
			triangle = append([]string{strings.Join(currentRow, " ")}, triangle...)
			currentRow = nil
			lineNumber++
		}
	}
	return
}
