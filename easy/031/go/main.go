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

import "fmt"

const base26Alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"

func main() {
	fmt.Println("hello world")
}

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
