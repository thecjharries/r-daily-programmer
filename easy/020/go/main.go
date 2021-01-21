package main

import (
	"fmt"
	"math"
)

var knownPrimesSeed = []int{2}

func main() {
	fmt.Println("hello world")
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
