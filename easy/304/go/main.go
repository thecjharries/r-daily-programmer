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

type Account struct {
	Number  int
	Name    string
	Balance float64
}

func NewAccount(number int, args ...string) Account {
	balance, err := strconv.ParseFloat(args[0], 64)
	if nil != err {
		return Account{
			Number: number,
			Name:   args[0],
		}
	}
	return Account{
		Number:  number,
		Balance: balance,
	}
}

type Journal map[int]Account

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
