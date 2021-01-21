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
