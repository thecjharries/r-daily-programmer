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
	"strings"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func findCheckDigit(upc int) (checkDigit int) {
	upcString := strconv.Itoa(upc)
	if 11 != len(upcString) {
		upcString = strings.Repeat("0", 11-len(upcString)) + upcString
	}
	evenSum := 0
	oddSum := 0
	for i := 0; i < len(upcString); i++ {
		if i%2 == 0 {
			evenSum += int(upcString[i] - '0')
		} else {
			oddSum += int(upcString[i] - '0')
		}
	}
	checkDigit = ((evenSum * 3) + oddSum) % 10
	if 0 != checkDigit {
		checkDigit = 10 - checkDigit
	}
	return
}
