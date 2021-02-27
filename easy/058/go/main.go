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

const base36Digits string = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
const base27Digits string = "0123456789ABCDEFGHIJKLMNOPQ"

func main() {
	fmt.Println("hello world")
}

func convertNumberToBase(number int, baseDigits string) (numberInBase string) {
	numberBeingConverted := number
	for 0 < numberBeingConverted {
		numberInBase = string(baseDigits[numberBeingConverted % len(baseDigits)]) + numberInBase
		numberBeingConverted /= len(baseDigits)
	}
	return
}

func isNumberPalindromicInBase(number int, baseDigits string) bool {
	numberInBase := convertNumberToBase(number, baseDigits)
	for index := 0; index < len(numberInBase) / 2; index++ {
		if numberInBase[index] != numberInBase[len(numberInBase) - index - 1] {
			return false
		}
	}
	return true
}
