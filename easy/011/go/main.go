package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Println("hello world")
}

// https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week#Zeller's_algorithm
func zellersAlgorithm() int {
	return 0
}

// m is the shifted month (March=3,...January = 13, February=14)
func getMonthTerm(month int) int {
	return ((month + 12 - 3) % 12) + 3
}

// Y is the year minus 1 for January or February, and the year for any other month
func getYearTerm(month, year int) int {
	if 1 <= month && month <= 2 {
		return year - 1
	}
	return year
}

// c is the first 2 digits of Y
func getFirstTwoDigitsOfYear(year int) int {
	return int(math.Floor(float64(year) / 100))
}

