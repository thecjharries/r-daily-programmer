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

func computeDivisors(number int) (divisors []int) {
	divisors = []int{1, number}
	for possibleDivisor := 2; possibleDivisor <= number/2; possibleDivisor++ {
		if 0 == number%possibleDivisor {
			divisors = append(divisors, possibleDivisor)
		}
	}
	return
}

func sum(numbers ...int) (totalSum int) {
	for _, number := range numbers {
		totalSum += number
	}
	return
}

func determineNumberClass(number int) string {
	comparison := 2*number - sum(computeDivisors(number)...)
	if 0 == comparison {
		return "perfect"
	} else if 0 < comparison {
		return fmt.Sprintf("deficient %d", comparison)
	} else {
		return fmt.Sprintf("abundant %d", -1*comparison)
	}
}
