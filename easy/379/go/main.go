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
)

var brackets = []float64{10000, 30000, 100000}
var rates = []float64{0, 0.1, 0.25, 0.4}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func tax(input float64) (total float64) {
	current := input
	if current > brackets[len(brackets)-1] {
		total = (current - brackets[len(brackets)-1]) * rates[len(rates)-1]
		current = brackets[len(brackets)-1]
	}
	for i := len(brackets) - 1; i > 0; i-- {
		fmt.Println(current)
		if current >= brackets[i-1] {
			fmt.Println(current, brackets[i])
			total += (current - brackets[i-1]) * rates[i]
			current = brackets[i-1]
		}
	}
	if current > 0 {
		total += current * rates[0]
	}
	return math.Floor(total)
}
