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
)

func main() {
	fmt.Println(generateBinaryRotationSequence(54321))
}

func rotateNumber(number int64) int64 {
	leadingDigit := strconv.FormatInt(number & 1, 2)
	shifted := strconv.FormatInt(number >> 1, 2)
	newNumber, _ := strconv.ParseInt(leadingDigit + shifted, 2, 64)
	return newNumber
}

func generateBinaryRotationSequence(number int64) (sequence []int64) {
	currentNumber := number
	sequence = append(sequence, currentNumber)
	newNumber := rotateNumber(currentNumber)
	for currentNumber != newNumber {
		sequence = append(sequence, newNumber)
		currentNumber = newNumber
		newNumber = rotateNumber(currentNumber)
	}
	return
}
