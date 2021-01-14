package main

import "fmt"

var normalDayCount = [12]int{31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31}
var leapDayCount = [12]int{31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31}

func main() {
	fmt.Println("hello world")
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
