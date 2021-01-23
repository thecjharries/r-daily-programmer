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
)

var knownPrimesSeed = []int{2}

func main() {
	fmt.Println(findPrimesBelow(2000))
}

func isPrime(input int, knownPrimes []int) bool {
	maxPrime := int(math.Ceil(math.Sqrt(float64(input))))
	for _, checkPrime := range knownPrimes {
		if 0 == input % checkPrime {
			return false
		}
		if maxPrime < checkPrime {
			return true
		}
	}
	return true
}

func findPrimesBelow(maxInt int) []int {
	primes := knownPrimesSeed
	for checkInt := primes[0] + 1; checkInt < maxInt; checkInt++ {
		if isPrime(checkInt, primes) {
			primes = append(primes, checkInt)
		}
	}
	return primes
}
