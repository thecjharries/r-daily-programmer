package main

import (
	"fmt"
	"math"
)

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

func reverseSortByBlock(slice []int, blockSize int) []int {
	var result []int
	for index := 0; index < len(slice); index += blockSize {
		sliceCap := int(math.Min(float64(index + blockSize), float64(len(slice))))
		result = append(result, reverseSort(slice[index:sliceCap])...)
	}
	return result
}
