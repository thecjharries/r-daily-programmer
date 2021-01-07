package main

import (
	"fmt"
	"github.com/shopspring/decimal"
)

var pi30Digits decimal.Decimal

func main() {
	pi30Digits, _ = decimal.NewFromString("3.141592653589793238462643383279")
	fmt.Println("hello world")
}
