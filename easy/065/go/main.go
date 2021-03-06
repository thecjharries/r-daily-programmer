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

var usdDenominations = []float64{100.00, 20.00, 10.00, 5.00, 1.00, 0.25, 0.10, 0.05, 0.01}

func main() {
	fmt.Println("hello world")
}

func convertToUsd(number float64) float64 {
	return math.Round(number * 100) / 100
}

func reduceCostByDenomination(cost, denomination float64) (reducedCost float64, denominationCount int) {
	reducedCost = cost
	denominationCount = 0
	for denomination <= reducedCost {
		reducedCost = convertToUsd(reducedCost - denomination)
		denominationCount++
	}
	return
}

func convertCostToDenominations(cost float64, denominations []float64) []int {
	reducingCost := cost
	denominationCount := make([]int, len(denominations))
	for index, denomination := range denominations {
		reducingCost, denominationCount[index] = reduceCostByDenomination(reducingCost, denomination)
	}
	return denominationCount
}
