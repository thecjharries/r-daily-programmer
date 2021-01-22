package main

import "fmt"

type MixedSlice []interface{}

func (s *MixedSlice) Contains(possibleElement interface{}) bool {
	for _, element := range *s {
		if possibleElement == element {
			return true
		}
	}
	return false
}

func main() {
	test := MixedSlice{"1"}
	fmt.Println(test)
	fmt.Println(test.Contains(1))
	fmt.Println(test.Contains("1"))
}
