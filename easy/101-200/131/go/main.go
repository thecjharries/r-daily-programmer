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

func reverseString(input string) string {
	result := []rune(input)
	for index := 0; index < len(result)/2; index++ {
		result[index], result[len(result)-1-index] = result[len(result)-1-index], result[index]
	}
	return string(result)
}

func uppercaseString(input string) string {
	return strings.ToUpper(input)
}

func verifyTransformation(input string, transformation func(string) string, output string) bool {
	return transformation(input) == output
}
