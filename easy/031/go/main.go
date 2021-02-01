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
	"math"
	"strings"
)

const base26Alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"

func main() {
	fmt.Printf("%s * %s = %s\n", "CSGHJ", "CBA", multiplyBase26("CSGHJ", "CBA"))
}

// Converts an int to the provided base26 interpretation
func base10ToBase26(input int) (base26 string) {
	if len(base26Alphabet) > input {
		return string(base26Alphabet[input])
	}
	var quotient, remainder int
	quotient = input
	for 0 != quotient {
		remainder = quotient % 26
		quotient /= 26
		base26 = string(base26Alphabet[remainder]) + base26
	}
	return
}

// Converts a string using the provided base26 interpretation into an int
func base26ToBase10(input string) (base10 int) {
	base10 = 0
	for index, base26 := range input {
		base10 += int(math.Pow(26, float64(len(input) - index - 1))) *
			strings.Index(base26Alphabet, string(base26))
	}
	return base10
}

// Multiplies two strings of the provided base26 interpretation
func multiplyBase26(a, b string) string {
	return base10ToBase26(
		base26ToBase10(a) * base26ToBase10(b),
	)
}
