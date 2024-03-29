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
	"strconv"
)

type Coin struct {
	Name  string
	Value int
	Count int
}

type Currency []Coin

func (c *Currency) String() (output string) {
	for _, coin := range *c {
		if 0 < coin.Count {
			output += fmt.Sprintf("\n%s: %s", coin.Name, strconv.Itoa(coin.Count))
		}
	}
	return
}

func NewCurrency(value float64, coins ...Coin) (currency Currency) {
	currentValue := int(value * 100)
	currency = make([]Coin, len(coins))
	copy(currency, coins)
	for index, coin := range currency {
		currency[index].Count = currentValue / coin.Value
		currentValue -= currency[index].Count * coin.Value
	}
	return
}

var UsCoins = []Coin{
	{"Quarter", 25, 0},
	{"Dime", 10, 0},
	{"Nickel", 5, 0},
	{"Penny", 1, 0},
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
