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
	"io"
	"math"
	"os"

	. "github.com/thecjharries/dprgstd/clinput"
)

func main() {
	total := GetFloat64Input("Meal total?", io.Reader(os.Stdin))
	desiredPercent := GetFloat64Input("Desired tip percent?", io.Reader(os.Stdin))
	tipPercent15 := getUsdPercentage(total, 15)
	tipPercent20 := getUsdPercentage(total, 20)
	tipPercent25 := getUsdPercentage(total, 25)
	tipPercentDesired := getUsdPercentage(total, desiredPercent)
	fmt.Printf("15.00%% tip: %.2f\n", tipPercent15)
	fmt.Printf("20.00%% tip: %.2f\n", tipPercent20)
	fmt.Printf("25.00%% tip: %.2f\n", tipPercent25)
	fmt.Printf("%.2f%% tip: %.2f\n", desiredPercent, tipPercentDesired)
}

func roundToUsd(input float64) float64 {
	return math.Round(input*100) / 100
}

func getUsdPercentage(base float64, percent float64) float64 {
	usdBase := roundToUsd(base)
	return roundToUsd(usdBase * percent / 100)
}
