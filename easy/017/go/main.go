package main

import (
	"fmt"
	"math"
	"strings"
)

func main() {
	fmt.Println("hello world")
}

func getStarsForLine(lineNumber int) string {
	count := int(math.Pow(2, math.Max(0, float64(lineNumber - 1))))
	return strings.Repeat("@", count)
}
