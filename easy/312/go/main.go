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

var englishToLeet = map[string]string{
	"A": "4",
	"B": "6",
	"E": "3",
	"I": "1",
	"L": "1",
	"M": "(V)",
	"N": "(\\)",
	"O": "0",
	"S": "5",
	"T": "7",
	"V": "\\/",
	"W": "`//",
}

var leetToEnglish = map[string]string{
	"4":    "A",
	"6":    "B",
	"3":    "E",
	"1":    "I",
	"(V)":  "M",
	"(\\)": "N",
	"0":    "O",
	"5":    "S",
	"7":    "T",
	"\\/":  "V",
	"`//":  "W",
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func translate(input string, dictionary map[string]string) (result string) {
	currentInput := input
	for 0 < len(currentInput) {
		updated := false
		for key, value := range dictionary {
			if strings.HasPrefix(strings.ToUpper(currentInput), key) {
				result += value
				currentInput = currentInput[len(key):]
				updated = true
				break
			}
		}
		if updated {
			continue
		}
		result += currentInput[:1]
		currentInput = currentInput[1:]
	}
	return
}
