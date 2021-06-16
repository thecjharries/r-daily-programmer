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

// https://gist.github.com/anonymous/0ce707518d9e581499f5
var pbmDictionary = map[rune]string{
	'a': "00100\n01010\n10001\n11111\n10001\n10001\n10001",
	'b': "11110\n10001\n10001\n11110\n10001\n10001\n11110",
	'c': "01110\n10001\n10000\n10000\n10000\n10001\n01110",
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func encode(input string) string {
	exploded := make([]string, 7)
	for _, character := range input {
		pbmString, exists := pbmDictionary[character]
		if exists {
			explodedCharacter := strings.Split(pbmString, "\n")
			for index, line := range explodedCharacter {
				exploded[index] += "0" + line
			}
		}
	}
	return strings.Join(exploded, "\n")
}
