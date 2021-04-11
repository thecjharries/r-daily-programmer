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

const errorUnknownDayOfWeek string = "Unknown day integer!"

func main() {
	fmt.Println("hello world")
}

// https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week#Zeller's_algorithm
func zellersAlgorithm(day, month, year int) int {
	m := getMonthTerm(month)
	convertedYear := getYearTerm(month, year)
	c := getFirstTwoDigitsOfYear(convertedYear)
	y := getLastTwoDigitsOfYear(convertedYear)
	return ((int(math.Floor(13*float64(m+1)/5))+
		int(math.Floor(float64(y)/4))+
		int(math.Floor(float64(c)/4))+
		day+
		y-
		(2*c))%7 + 7) % 7
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

// y is the last 2 digits of Y
func getLastTwoDigitsOfYear(year int) int {
	return year - int(math.Floor(float64(year)/100)*100)
}

func getDayOfWeek(day int) string {
	switch day {
	case 1:
		return "Sunday"
	case 2:
		return "Monday"
	case 3:
		return "Tuesday"
	case 4:
		return "Wednesday"
	case 5:
		return "Thursday"
	case 6:
		return "Friday"
	case 0:
		return "Saturday"
	default:
		panic(errorUnknownDayOfWeek)
	}
}
