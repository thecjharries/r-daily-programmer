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

func main() {
	fmt.Println("hello world")
}

func reverseString(input string) string {
	output := []rune(input)
	for index := 0; index < len(output) / 2; index++ {
		output[index], output[len(output) - 1 - index] = output[len(output) - 1 - index], output[index]
	}
	return string(output)
}

func reverseInteger(input int) int {
	inputAsString := strconv.Itoa(input)
	reversedInputAsString := reverseString(inputAsString)
	reversedInput, _ := strconv.ParseInt(reversedInputAsString, 10, 0)
	return int(reversedInput)
}
