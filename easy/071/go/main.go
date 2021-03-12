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

type PythagoreanTriplet struct {
	A float64
	B float64
	C float64
}

func main() {
	fmt.Println(bruteForcePythagoreanTripletsFor(504))
}

func bruteForcePythagoreanTripletsFor(sum float64) (triplets []PythagoreanTriplet) {
	for a := float64(1); a <= sum - 2; a++ {
		for b := a; b <= sum - 2; b++ {
			if math.Pow(a, 2) + math.Pow(b, 2) == math.Pow(sum - a - b, 2) {
				triplets = append(triplets, PythagoreanTriplet{a, b, sum - a - b})
			}
		}
	}
	return
}
