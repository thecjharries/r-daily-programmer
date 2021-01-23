package main

import (
	"fmt"
	. "github.com/thecjharries/dprgstd/mixedslice"
)

func main() {
	first := MixedSlice{"a", "b", "c", 1, 4}
	second := MixedSlice{"a", "x", 34, "4"}
	first.AddMissingElements(second)
	fmt.Println(first)
}
