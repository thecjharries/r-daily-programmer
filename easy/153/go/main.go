// Copyright 2021 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package main

import "fmt"

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func factorial(n int) int {
	if 2 > n {
		return 1
	}
	return n * factorial(n-1)
}

func pascalsPyramid(layer int) (pyramid [][]int) {
	for row := 0; row <= layer; row++ {
		var pyramidRow []int
		for a := layer - row; a > 0; a-- {
			b := layer - row - a
			pyramidRow = append(pyramidRow, factorial(layer)/(factorial(row)*factorial(a)*factorial(b)))
		}
		pyramid = append(pyramid, pyramidRow)
	}
	return
}
