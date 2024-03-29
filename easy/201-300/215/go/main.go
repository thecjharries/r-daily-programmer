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
	"strconv"
)

type Float64Slice []float64

func (f *Float64Slice) IndexOf(number float64) int {
	for index, element := range *f {
		if number == element {
			return index
		}
	}
	return -1
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func explodeAndSumNumber(base, number float64) (total float64) {
	numberAsString := strconv.Itoa(int(number))
	for _, element := range numberAsString {
		currentFloat, _ := strconv.ParseFloat(string(element), 64)
		total += math.Pow(currentFloat, base)
	}
	return
}

func findSadCycle(base, start float64) (result Float64Slice) {
	result = make(Float64Slice, 0)
	currentNumber := start
	for 1 != currentNumber && -1 == result.IndexOf(currentNumber) {
		result = append(result, currentNumber)
		currentNumber = explodeAndSumNumber(base, currentNumber)
	}
	if 1 == currentNumber {
		return Float64Slice{1}
	}
	return result[result.IndexOf(currentNumber):]
}
