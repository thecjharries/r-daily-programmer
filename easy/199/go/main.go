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

var normalToBanner = map[rune][]string{
	'1': {"   ", "  |", "  |", "   "},
	'2': {" _ ", " _|", "|_ ", "   "},
	'3': {" _ ", " _|", " _|", "   "},
	'4': {"   ", "|_|", "  |", "   "},
	'5': {" _ ", "|_ ", " _|", "   "},
	'6': {" _ ", "|_ ", "|_|", "   "},
	'7': {" _ ", "  |", "  |", "   "},
	'8': {" _ ", "|_|", "|_|", "   "},
	'9': {" _ ", "|_|", " _|", "   "},
	'0': {" _ ", "| |", "|_|", "   "},
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func convertStringToBanner(input string) string {
	output := make([]string, 4)
	for _, character := range input {
		banner, exists := normalToBanner[character]
		if exists {
			for index, line := range banner {
				output[index] = fmt.Sprintf("%s%s", output[index], line)
			}
		}
	}
	return strings.Join(output, "\n")
}
