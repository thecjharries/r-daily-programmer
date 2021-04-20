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

var shiftedAlphabet = " snvfrghjokl;,mp[wtdyibcuxSNVFRGHJOKL:<MP{WTDYIBCUX"
var romanAlphabet = " abcdefghijklmnopqrstuvxyzABCDEFGHIJKLMNOPQRSTUVXYZ"

var shiftedToRomanAlphabetCache = map[string]string{}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint(shiftToRoman("Jr;;p ept;f"))
	_, _ = zPrint(shiftToRoman("Lmiyj od ,u jrtp"))
}

func shiftToRoman(input string) (result string) {
	for _, character := range input {
		characterAsString := string(character)
		translation, exists := shiftedToRomanAlphabetCache[characterAsString]
		if exists {
			result += translation
		} else {
			index := strings.Index(shiftedAlphabet, characterAsString)
			if -1 < index {
				shiftedToRomanAlphabetCache[characterAsString] = string(romanAlphabet[index])
				result += string(romanAlphabet[index])
			} else {
				result += characterAsString
			}
		}
	}
	return
}
