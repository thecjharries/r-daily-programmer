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

var normalDayCount = [12]int{31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31}
var leapDayCount = [12]int{31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31}

func main() {
	fmt.Println(getDayNumber(1,1, true))
	fmt.Println(getDayNumber(1,12, true))
	fmt.Println(getDayNumber(1,1, false))
	fmt.Println(getDayNumber(1,12, false))
}

func getDaysInMonthSlice(isLeapYear bool) [12]int {
	if isLeapYear {
		return leapDayCount
	}
	return normalDayCount
}

func getSumOfDaysBefore(month int, isLeapYear bool) int {
	monthSlice := getDaysInMonthSlice(isLeapYear)
	result := 0
	for monthIndex := 0; monthIndex < month - 1 && monthIndex < len(monthSlice); monthIndex++ {
		result += monthSlice[monthIndex]
	}
	return result
}

// Note this can do weird stuff because day is not bounded
func getDayNumber(day, month int, isLeapYear bool) int {
	return day + getSumOfDaysBefore(month, isLeapYear)
}
