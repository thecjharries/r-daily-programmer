package main

import (
	"fmt"
	"math"
)

type MixedSlice []interface{}

func (s *MixedSlice) SplitInHalf() ([]interface{}, []interface{}) {
	halfLen := int(math.Ceil(float64(len(*s)) / 2))
	return (*s)[:halfLen], (*s)[halfLen:]
}

func main() {
	fmt.Println("hello world")
}
