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

type CardValue int

const (
	CardValueAce CardValue = iota + 1
	CardValue2
	CardValue3
	CardValue4
	CardValue5
	CardValue6
	CardValue7
	CardValue8
	CardValue9
	CardValue10
	CardValueJack  = CardValue10
	CardValueQueen = CardValueJack
	CardValueKing  = CardValueQueen
)

func (c CardValue) Value() int {
	return int(c)
}

type CardSuit int

const (
	CardSuitClubs CardSuit = iota
	CardSuitDiamonds
	CardSuitHearts
	CardSuitSpades
)

func (c CardSuit) String() string {
	return []string{"clubs", "diamonds", "hearts", "spades"}[c]
}

type Card struct {
	Value CardValue
	Suit  CardSuit
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
