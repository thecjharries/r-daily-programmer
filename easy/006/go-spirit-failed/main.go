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

	"github.com/shopspring/decimal"
)

var pi30Digits decimal.Decimal
var oneDecimal = decimal.New(1, 0)
var twoDecimal = decimal.New(2, 0)
var threeDecimal = decimal.New(3, 0)
var fourDecimal = decimal.New(4, 0)

const digitPrecision int32 = 30

func main() {
	pi30Digits, _ = decimal.NewFromString("3.141592653589793238462643383279")
	fmt.Println("CJ doesn't understand the numerical analysis well enough to solve the spirit of the prompt")
	// See https://play.golang.org/p/hF9jklt5lp for a solution
}

func roundDecimalToPrecision(number decimal.Decimal, precision int32) decimal.Decimal {
	return number.Round(precision)
}

// THIS IS SUPER SLOW
// Doesn't hit 30 digits in 100000000 iterations
func leibnizConvergenceFormulaNthTerm(n int64) decimal.Decimal {
	if 0 >= n {
		return twoDecimal.Div(threeDecimal)
	}
	nDecimal := decimal.New(n, 0)
	denominatorFirst := (fourDecimal.Mul(nDecimal)).Add(oneDecimal)
	denominatorSecond := (fourDecimal.Mul(nDecimal)).Add(threeDecimal)
	denominator := denominatorFirst.Mul(denominatorSecond)
	return twoDecimal.Div(denominator)
}

func machinPi() decimal.Decimal {
	firstTerm := decimal.New(4, 0).Mul(oneDecimal.Div(decimal.New(5, 0)).Atan())
	secondTerm := decimal.New(-1, 0).Mul(oneDecimal.Div(decimal.New(239, 0)).Atan())
	return firstTerm.Add(secondTerm)
}

// https://en.wikipedia.org/wiki/Approximations_of_%CF%80#Machin-like_formula
// This doesn't work, probably because of how the lib calculates arctan
func shanksMachinPi() decimal.Decimal {
	firstTerm := decimal.New(6, 0).Mul(oneDecimal.Div(decimal.New(8, 0)).Atan())
	secondTerm := decimal.New(2, 0).Mul(oneDecimal.Div(decimal.New(57, 0)).Atan())
	thirdTerm := decimal.New(1, 0).Mul(oneDecimal.Div(decimal.New(239, 0)).Atan())
	return firstTerm.Add(secondTerm).Add(thirdTerm)
}

// https://en.wikipedia.org/wiki/Approximations_of_%CF%80#Machin-like_formula
// This doesn't work, probably because of how the lib calculates arctan
func shanksMachinPiCheck() decimal.Decimal {
	firstTerm := decimal.New(12, 0).Mul(oneDecimal.Div(decimal.New(18, 0)).Atan())
	secondTerm := decimal.New(8, 0).Mul(oneDecimal.Div(decimal.New(57, 0)).Atan())
	thirdTerm := decimal.New(-5, 0).Mul(oneDecimal.Div(decimal.New(239, 0)).Atan())
	return firstTerm.Add(secondTerm).Add(thirdTerm)
}
