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
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

func main() {
	input := getStringInput("String to permute?", os.Stdin)
	fmt.Println("\nPermutations:")
	permuteAndPrint([]rune(input), 0)
}

var zPrintFunction = fmt.Println

func getStringInput(prompt string, source io.Reader) string {
	reader := bufio.NewReader(source)
	fmt.Println(prompt)
	input, _ := reader.ReadString('\n')
	return strings.Replace(input, "\n", "", -1)
}

func permuteAndPrint(stringToPermute []rune, startingIndex int) {
	if len(stringToPermute) <= startingIndex {
		_, _ = zPrintFunction(string(stringToPermute))
		return
	}
	for movingIndex := startingIndex; movingIndex < len(stringToPermute); movingIndex++ {
		stringToPermute[startingIndex], stringToPermute[movingIndex] = stringToPermute[movingIndex], stringToPermute[startingIndex]
		permuteAndPrint(stringToPermute, startingIndex + 1)
		stringToPermute[startingIndex], stringToPermute[movingIndex] = stringToPermute[movingIndex], stringToPermute[startingIndex]
	}
}
