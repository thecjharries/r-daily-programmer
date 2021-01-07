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
	var n int64 = 0
	calculatedPi := leibnizConvergenceFormulaNthTerm(0).Mul(fourDecimal)
	for !calculatedPi.Equals(pi30Digits) {
		n++
		calculatedPi = leibnizConvergenceFormulaNthTerm(n).Mul(fourDecimal)
		if 10000 < n {
			fmt.Println("whoops")
			fmt.Println(calculatedPi.String())
			break
		}
	}
}

func roundDecimalToPrecision(number decimal.Decimal, precision int32) decimal.Decimal {
	return number.Round(precision)
}

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
