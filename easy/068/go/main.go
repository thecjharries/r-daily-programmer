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
	"strconv"
)

func main() {
	fmt.Println(getEmirpBelowMaxInclusive(100))
}

func reverseString(input string) string {
	output := []rune(input)
	for index := 0; index < len(output) / 2; index++ {
		output[index], output[len(output) - 1 - index] = output[len(output) - 1 - index], output[index]
	}
	return string(output)
}

func reverseInteger(input int) int {
	inputAsString := strconv.Itoa(input)
	reversedInputAsString := reverseString(inputAsString)
	reversedInput, _ := strconv.ParseInt(reversedInputAsString, 10, 0)
	return int(reversedInput)
}

func isIntegerPalindrome(input int) bool {
	inputAsString := strconv.Itoa(input)
	for index := 0; index < len(inputAsString) / 2; index++ {
		if inputAsString[index] != inputAsString[len(inputAsString) - 1 - index] {
			return false
		}
	}
	return true
}

func isPrime(input int) bool {
	for index := 2; index <= int(math.Sqrt(float64(input))); index++ {
		if 0 == input % index {
			return false
		}
	}
	return true
}

func isEmirp(input int) bool {
	return !isIntegerPalindrome(input) && isPrime(input) && isPrime(reverseInteger(input))
}

func getEmirpBelowMaxInclusive(maxInteger int) (emirps []int) {
	for index := 13; index <= maxInteger; index++ {
		if isEmirp(index) {
			emirps = append(emirps, index)
		}
	}
	return
}
