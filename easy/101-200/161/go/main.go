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
	"math/rand"
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

func (c CardSuit) String() string {
	return []string{"clubs", "diamonds", "hearts", "spades"}[c]
}

var CardSuits = []CardSuit{CardSuitClubs, CardSuitDiamonds, CardSuitHearts, CardSuitSpades}

type Card struct {
	Value CardValue
	Suit  CardSuit
}

type Deck []Card

func (d *Deck) Shuffle() {
	rand.Shuffle(len(*d), func(i, j int) {
		(*d)[i], (*d)[j] = (*d)[j], (*d)[i]
	})
}

func (d *Deck) DealBlackjackHand() *BlackjackHand {
	hand := new(BlackjackHand)
	*hand = append(*hand, (*d)[0], (*d)[1])
	*d = (*d)[2:]
	return hand
}

func (d *Deck) BlackjackCount() (count int, total int) {
	for 0 < len(*d) {
		hand := d.DealBlackjackHand()
		total++
		if hand.IsBlackjack() {
			count++
		}
	}
	return
}

func NewDeck(totalDecks int) *Deck {
	var deck Deck
	for index := 0; index < totalDecks; index++ {
		for _, suit := range CardSuits {
			for _, value := range CardValues {
				card := Card{
					Value: value,
					Suit:  suit,
				}
				deck = append(deck, card)
			}
		}
	}
	return &deck
}

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

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
