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

var sevenSegmentToDigit = map[string]string{
	"   \n  |\n  |": "1",
	" _ \n _|\n|_ ": "2",
	" _ \n _|\n _|": "3",
	"   \n|_|\n  |": "4",
	" _ \n|_ \n _|": "5",
	" _ \n|_ \n|_|": "6",
	" _ \n  |\n  |": "7",
	" _ \n|_|\n|_|": "8",
	" _ \n|_|\n _|": "9",
	" _ \n| |\n|_|": "0",
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func convertSevenSegmentToInt(input string) (converted int) {
	exploded := strings.Split(input, "\n")
	output := ""
	for characterIndex := 0; characterIndex < len(exploded[0]); characterIndex += 3 {
		currentExploded := make([]string, len(exploded))
		for rowIndex := 0; rowIndex < len(exploded); rowIndex++ {
			currentExploded[rowIndex] = exploded[rowIndex][characterIndex : characterIndex+3]
		}
		output += sevenSegmentToDigit[strings.Join(currentExploded, "\n")]
	}
	converted, _ = strconv.Atoi(output)
	return
}
