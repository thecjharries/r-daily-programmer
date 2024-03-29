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
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func convertFromBaseFibonacciToBase10(fibonacci string) (base10 int) {
	current, previous := 1, 1
	if '1' == fibonacci[len(fibonacci)-1] {
		base10 += 1
	}
	if 1 < len(fibonacci) && '1' == fibonacci[len(fibonacci)-2] {
		base10 += 1
	}
	for index := len(fibonacci) - 3; index >= 0; index-- {
		current, previous = current+previous, current
		if '1' == fibonacci[index] {
			base10 += current
		}
	}
	return
}

func convertFromBase10ToBaseFibonacci(base10 int) (fibonacci string) {
	if 1 == base10 {
		return "10"
	}
	fibonacciBits := []int{1, 1}
	for base10 > fibonacciBits[len(fibonacciBits)-1] {
		fibonacciBits = append(fibonacciBits, fibonacciBits[len(fibonacciBits)-1]+fibonacciBits[len(fibonacciBits)-2])
	}
	currentBase10 := base10
	for index := len(fibonacciBits) - 2; index >= 0; index-- {
		if currentBase10 >= fibonacciBits[index] {
			currentBase10 -= fibonacciBits[index]
			fibonacci += "1"
		} else {
			fibonacci += "0"
		}
	}
	return
}
