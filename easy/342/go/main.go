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

type Polynomial []float64

func (p Polynomial) Degree() int {
	for index := len(p) - 1; index >= 0; index-- {
		if (p)[index] != 0 {
			return index
		}
	}
	return 0
}

// https://rosettacode.org/wiki/Polynomial_long_division#Go
func (p Polynomial) DivideBy(divisor Polynomial) (quotient, remainder Polynomial) {
	remainder = make(Polynomial, len(p))
	copy(remainder, p)
	fmt.Println("remainder:", remainder)
	if divisor.Degree() <= p.Degree() {
		quotient = make(Polynomial, p.Degree()-divisor.Degree()+1)
		for divisor.Degree() <= remainder.Degree() {
			fmt.Println("remainder: ", remainder)
			denominator := make(Polynomial, remainder.Degree()+1)
			copy(denominator[remainder.Degree()-divisor.Degree():], divisor)
			quotient[remainder.Degree()-divisor.Degree()] = remainder[remainder.Degree()] / denominator[remainder.Degree()]
			for index := range denominator {
				denominator[index] *= quotient[remainder.Degree()-divisor.Degree()]
				remainder[index] -= denominator[index]
			}
		}
	}
	return
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
