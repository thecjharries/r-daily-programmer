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

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func discover4DigitVampireNumbers() (results []string) {
	discovered := make(map[float64][]float64)
	minNumber := math.Pow(10, 1)
	maxNumber := math.Pow(10, 2)
	for firstNumber := minNumber; firstNumber < maxNumber; firstNumber++ {
		for secondNumber := minNumber; secondNumber < maxNumber; secondNumber++ {
			product := firstNumber * secondNumber
			_, exists := discovered[product]
			if 4 == len(fmt.Sprintf("%g", product)) && !exists {
				discovered[product] = []float64{firstNumber, secondNumber}
			}
		}
	}
	for key, value := range discovered {
		results = append(results, fmt.Sprintf("%g = %v", key, value))
	}
	sort.Strings(results)
	return
}
