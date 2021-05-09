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

type RealVector []float64

func (r *RealVector) RoundToFivePlaces(input float64) float64 {
	return math.Round(input*100000) / 100000
}

func (r *RealVector) Length() float64 {
	sum := 0.0
	for _, element := range *r {
		sum += math.Pow(element, 2)
	}
	return r.RoundToFivePlaces(math.Sqrt(sum))
}

func (r *RealVector) Normalized() *RealVector {
	unit := RealVector{}
	length := r.Length()
	for _, element := range *r {
		unit = append(unit, r.RoundToFivePlaces(element/length))
	}
	return &unit
}

// I'm assuming the vectors are the same length because I'm lazy
// If not this will panic
func (r *RealVector) Dot(vector *RealVector) float64 {
	sum := 0.0
	for index := 0; index < len(*r); index++ {
		sum += (*r)[index] * (*vector)[index]
	}
	return r.RoundToFivePlaces(sum)
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
