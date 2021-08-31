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

func findBestPrices(prices []float64) (low, high float64) {
	bestPriceDifference := 0.0
	for lowIndex := 0; lowIndex < len(prices)-2; lowIndex++ {
		for highIndex := lowIndex + 2; highIndex < len(prices); highIndex++ {
			currentPriceDifference := prices[highIndex] - prices[lowIndex]
			if 0 < currentPriceDifference && bestPriceDifference < currentPriceDifference {
				bestPriceDifference = currentPriceDifference
				low = prices[lowIndex]
				high = prices[highIndex]
			}
		}
	}
	return
}
