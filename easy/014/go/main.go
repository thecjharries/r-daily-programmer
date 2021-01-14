package main

import "fmt"

func main() {
	fmt.Println("hello world")
}

// https://stackoverflow.com/a/19239850
func reverseSort(slice []int) []int {
	result := slice
	for lead, tail := 0, len(result) - 1; lead < tail; lead, tail = lead + 1, tail -1 {
		result[lead], result[tail] = result[tail], result[lead]
	}
	return result
}
