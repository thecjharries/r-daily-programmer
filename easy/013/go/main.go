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
