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
	"time"
)

const (
	// Smallest ASCII character
	asciiLowerBound int = 0
	// Largest ASCII character
	asciiUpperBound int = 255
	// Smallest ASCII character we want (!)
	characterLowerBound int = 33
	// Largest ASCII character we want (~)
	characterUpperBound int = 126
)

func main() {
	rand.Seed(time.Now().UnixNano())
	fmt.Println("THIS IS NOT CRYPTOGRAPHICALLY SECURE")
	stringLength := 24
	stringCount := 5
	for index := 0; index <= stringCount; index++ {
		fmt.Println(generateRandomCharactersInBoundsOfLength(
			characterLowerBound,
			characterUpperBound,
			stringLength,
		))
	}
}

func min(numbers ...int) int {
	smallest := numbers[0]
	for _, number := range numbers {
		if number < smallest {
			smallest = number
		}
	}
	return smallest
}

func max(numbers ...int) int {
	largest := numbers[0]
	for _, number := range numbers {
		if number > largest {
			largest = number
		}
	}
	return largest
}

func randomIntInRange(minInt, maxInt int) int {
	return rand.Intn(maxInt-minInt+1) + minInt
}

func randomAsciiCharacterInRange(minInt, maxInt int) string {
	return string(
		rune(randomIntInRange(
			max(minInt, asciiLowerBound),
			min(maxInt, asciiUpperBound),
		)),
	)
}

func generateRandomCharactersInBoundsOfLength(lowerBound, upperBound, length int) string {
	result := ""
	for index := 0; index < length; index++ {
		result += randomAsciiCharacterInRange(lowerBound, upperBound)
	}
	return result
}
