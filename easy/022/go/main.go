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

func (s *MixedSlice) AddMissingElements(mixedSlice MixedSlice) {
	for _, element := range mixedSlice {
		if !s.Contains(element) {
			*s = append(*s, element)
		}
	}
}

func main() {
	first := MixedSlice{"a", "b", "c", 1, 4}
	second := MixedSlice{"a", "x", 34, "4"}
	first.AddMissingElements(second)
	fmt.Println(first)
}
