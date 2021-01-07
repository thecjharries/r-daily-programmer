package main

import (
	"fmt"
	"github.com/shopspring/decimal"
)

var pi30Digits decimal.Decimal

const digitPrecision int32 = 30

func main() {
	pi30Digits, _ = decimal.NewFromString("3.141592653589793238462643383279")
	fmt.Println("hello world")
}

func roundDecimalToPrecision(number decimal.Decimal, precision int32) decimal.Decimal {
	return number.Round(precision)
}
