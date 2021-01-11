package main

import "fmt"

func main() {
	fmt.Println("hello world")
}

// https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week#Disparate_variation
func disparateGaussAlgorithm() int {
	return 0
}

// m is the shifted month (March=1,...,February=12)
func getMonthTerm(month int) int {
	return ((month + 12 - 3) % 12) + 1
}

