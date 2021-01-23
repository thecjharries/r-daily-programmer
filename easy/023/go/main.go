package main

import (
	"fmt"
	"math"
)

type MixedSlice []interface{}

func (s *MixedSlice) SplitInHalf() (MixedSlice, MixedSlice) {
	halfLen := int(math.Ceil(float64(len(*s)) / 2))
	return (*s)[:halfLen], (*s)[halfLen:]
}

func main() {
	fmt.Println("hello world")
}
