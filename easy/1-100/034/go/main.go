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

import (
	"fmt"
	"math"
	"sort"
)

func main() {
	input := []float64{1, 2, 3}
	fmt.Println(input)
	fmt.Println(sumSquaresOfTwoLargest(input...))
}

func findTwoLargest(inputs ...float64) (float64, float64) {
	sort.Float64s(inputs)
	return inputs[len(inputs)-1], inputs[len(inputs)-2]
}

func sumSquaresOfTwoLargest(inputs ...float64) float64 {
	first, second := findTwoLargest(inputs...)
	return math.Pow(first, 2) + math.Pow(second, 2)
}
