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
	"regexp"
	"strings"
)

type CardValue int

const (
	CardValue2 CardValue = iota + 2
	CardValue3
	CardValue4
	CardValue5
	CardValue6
	CardValue7
	CardValue8
	CardValue9
	CardValue10
	CardValueAce
	CardValueJack  = CardValue10
	CardValueQueen = CardValueJack
	CardValueKing  = CardValueQueen
)

var cardValueNameToCardValue = map[string]CardValue{
	"two":   CardValue2,
	"three": CardValue3,
	"four":  CardValue4,
	"five":  CardValue5,
	"six":   CardValue6,
	"seven": CardValue7,
	"eight": CardValue8,
	"nine":  CardValue9,
	"ten":   CardValue10,
	"ace":   CardValueAce,
	"jack":  CardValueJack,
	"queen": CardValueQueen,
	"king":  CardValueKing,
}

func (c CardValue) Value() int {
	return int(c)
}

var CardValues = []CardValue{CardValueAce, CardValue2, CardValue3, CardValue4, CardValue5, CardValue6, CardValue7, CardValue8, CardValue9, CardValue10, CardValueJack, CardValueQueen, CardValueKing}

type CardSuit int

const (
	CardSuitClubs CardSuit = iota
	CardSuitDiamonds
	CardSuitHearts
	CardSuitSpades
)

var cardSuitNameToCardSuit = map[string]CardSuit{
	"clubs":    CardSuitClubs,
	"diamonds": CardSuitDiamonds,
	"hearts":   CardSuitHearts,
	"spades":   CardSuitSpades,
}

func (c CardSuit) String() string {
	return []string{"clubs", "diamonds", "hearts", "spades"}[c]
}

var CardSuits = []CardSuit{CardSuitClubs, CardSuitDiamonds, CardSuitHearts, CardSuitSpades}

type Card struct {
	Value CardValue
	Suit  CardSuit
}

var handSyntaxPattern = regexp.MustCompile(`(?i)(\w+) of (\w+)`)

type BlackjackHand []Card

func (h *BlackjackHand) Value() (value int) {
	aceCount := -1
	for _, card := range *h {
		value += card.Value.Value()
		if CardValueAce == card.Value {
			aceCount++
		}
	}
	if 0 < aceCount {
		value -= aceCount * 10
	}
	return
}

func (h *BlackjackHand) IsBlackjack() bool {
	return 2 == len(*h) && 21 == h.Value()
}

func NewBlackjackHandFromString(input string) (hand BlackjackHand) {
	matches := handSyntaxPattern.FindAllStringSubmatch(strings.ToLower(input), -1)
	for _, match := range matches {
		hand = append(
			hand,
			Card{
				Value: cardValueNameToCardValue[match[1]],
				Suit:  cardSuitNameToCardSuit[match[2]],
			},
		)
	}
	return
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
