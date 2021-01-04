package main

import (
	"fmt"
	"math/rand"
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
	fmt.Println("THIS IS NOT CRYPTOGRAPHICALLY SECURE")
	fmt.Println(rand.Intn(1))
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

func randomIntInRange(min, max int) int {
	return rand.Intn(max - min + 1) + min
}
