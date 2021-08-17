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

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

// Modified from https://siongui.github.io/2017/05/09/go-find-all-prime-factors-of-integer-number/
func getUniquePrimeFactors(number int) (uniquePrimeFactors []int) {
	currentNumber := number
	// Remove all possible 2s as factors
	if 0 == currentNumber%2 {
		uniquePrimeFactors = append(uniquePrimeFactors, 2)
		for 0 == currentNumber%2 {
			currentNumber /= 2
		}
	}
	// Look at odd numbers
	// Since we count up, only primes will be included
	for possiblePrime := 3; possiblePrime*possiblePrime <= currentNumber; possiblePrime += 2 {
		if 0 == currentNumber%possiblePrime {
			uniquePrimeFactors = append(uniquePrimeFactors, possiblePrime)
			for 0 == currentNumber%possiblePrime {
				currentNumber /= possiblePrime
			}
		}
	}
	// The original input was prime
	if 2 < currentNumber {
		uniquePrimeFactors = append(uniquePrimeFactors, currentNumber)
	}
	return uniquePrimeFactors
}

func sum(numbers ...int) (totalSum int) {
	for _, number := range numbers {
		totalSum += number
	}
	return totalSum
}

func isRuthAaronPair(first, second int) bool {
	return sum(getUniquePrimeFactors(first)...) == sum(getUniquePrimeFactors(second)...)
}
