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
	"sort"
)

func main() {
	fmt.Println("hello world")
}

func greatestCommonDivisor(first, second int) int {
	if 0 == second {
		return first
	}
	return greatestCommonDivisor(second, first % second)
}

func getTotatives(number int) (totatives []int) {
	for index := 1; index < number; index++ {
		if 1 == greatestCommonDivisor(number, index) {
			totatives = append(totatives, index)
		}
	}
	return
}

func getTotient(number int) int {
	return len(getTotatives(number))
}

func getDivisors(number int) (divisors []int) {
	for index := 1; index < int(math.Sqrt(float64(number))) + 1; index++ {
		if 0 == number % index {
			divisors = append(divisors, index)
			divisors = append(divisors, number / index)
		}
	}
	sort.Ints(divisors)
	return
}

func getNumberOfDivisors(number int) int {
	return len(getDivisors(number))
}

func getSumOfDivisors(number int) int {
	totalSum := 0
	for _, element := range getDivisors(number) {
		totalSum += element
	}
	return totalSum
}
