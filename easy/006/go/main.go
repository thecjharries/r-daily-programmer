package main

import (
	"fmt"
	"github.com/shopspring/decimal"
)

var pi30Digits decimal.Decimal
var oneDecimal = decimal.New(1, 0)
var twoDecimal = decimal.New(2, 0)
var threeDecimal = decimal.New(3,0)
var fourDecimal = decimal.New(4,0)

const digitPrecision int32 = 30

func main() {
	pi30Digits, _ = decimal.NewFromString("3.141592653589793238462643383279")
	fmt.Println("hello world")
}

func roundDecimalToPrecision(number decimal.Decimal, precision int32) decimal.Decimal {
	return number.Round(precision)
}

func leibnizConvergenceFormulaToNPlaces(n int64) decimal.Decimal {
	if 0 == n {
		return twoDecimal.Div(threeDecimal)
	}
	nDecimal := decimal.New(n, 0)
	denominatorFirst := (fourDecimal.Mul(nDecimal)).Add(oneDecimal)
	denominatorSecond := (fourDecimal.Mul(nDecimal)).Add(threeDecimal)
	denominator := denominatorFirst.Mul(denominatorSecond)
	nthTerm := twoDecimal.Div(denominator)
	return nthTerm.Add(leibnizConvergenceFormulaToNPlaces(n - 1))
}
