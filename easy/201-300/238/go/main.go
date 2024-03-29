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
	"math/rand"
	"strings"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func getRandomVowel() string {
	return string("aeiou"[rand.Intn(5)])
}

func getRandomConsonant() string {
	return string("bcdfghjklmnpqrstvwxyz"[rand.Intn(21)])
}

func createWordFromForm(form string) (result string) {
	for _, element := range form {
		switch element {
		case 'c':
			result += getRandomConsonant()
		case 'C':
			result += strings.ToUpper(getRandomConsonant())
		case 'v':
			result += getRandomVowel()
		case 'V':
			result += strings.ToUpper(getRandomVowel())
		}
	}
	return
}
