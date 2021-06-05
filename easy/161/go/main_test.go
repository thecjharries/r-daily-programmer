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
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct{}

var _ = Suite(&MainSuite{})

var printCallCount int
var printSpyContents string

func printSpy(a ...interface{}) (n int, err error) {
	printSpyContents = fmt.Sprint(a...)
	printCallCount++
	return
}

func (s *MainSuite) SetUpTest(c *C) {
	printCallCount = 0
	printSpyContents = ""
	zPrint = printSpy
	rand.Seed(0)
}

func (s *MainSuite) TearDownTest(c *C) {
	zPrint = fmt.Println
}

func (s *MainSuite) TestMain(c *C) {
	c.Assert(printCallCount, Equals, 0)
	c.Assert(printSpyContents, Equals, "")
	main()
	c.Assert(printCallCount, Equals, 1)
	c.Assert(printSpyContents, Equals, "hello world")
}

func (s *MainSuite) TestCardValueValue(c *C) {
	c.Assert(CardValue(CardValue2).Value(), Equals, 2)
}

func (s *MainSuite) TestCardSuitString(c *C) {
	c.Assert(CardSuit(CardSuitClubs).String(), Equals, "clubs")
}

func (s *MainSuite) TestDeckShuffle(c *C) {
	deck := NewDeck(1)
	c.Assert((*deck)[0].Suit, Equals, CardSuitClubs)
	c.Assert((*deck)[0].Value, Equals, CardValueAce)
	deck.Shuffle()
	c.Assert((*deck)[0].Suit, Equals, CardSuitDiamonds)
	c.Assert((*deck)[0].Value, Equals, CardValue7)
}

func (s *MainSuite) TestDeckDealBlackjackHand(c *C) {
	deck := NewDeck(1)
	c.Assert(len(*deck), Equals, 52)
	hand := deck.DealBlackjackHand()
	c.Assert(len(*deck), Equals, 50)
	c.Assert(len(*hand), Equals, 2)
	c.Assert((*hand)[0].Suit, Equals, CardSuitClubs)
	c.Assert((*hand)[0].Value, Equals, CardValueAce)
	c.Assert((*hand)[1].Suit, Equals, CardSuitClubs)
	c.Assert((*hand)[1].Value, Equals, CardValue2)
}

func (s *MainSuite) TestNewDeck(c *C) {
	deck := NewDeck(1)
	c.Assert(len(*deck), Equals, 52)
}

func (s *MainSuite) TestBlackjackHandValue(c *C) {
	hand := BlackjackHand{
		{CardValueAce, CardSuitClubs},
		{CardValueQueen, CardSuitClubs},
	}
	c.Assert(hand.Value(), Equals, 11)
}

func (s *MainSuite) TestBlackjackHandIsBlackjack(c *C) {
	var hand BlackjackHand
	hand = BlackjackHand{
		{CardValueAce, CardSuitClubs},
		{CardValueQueen, CardSuitClubs},
	}
	c.Assert(hand.IsBlackjack(), Equals, true)
	hand = BlackjackHand{
		{CardValue2, CardSuitClubs},
		{CardValue9, CardSuitClubs},
	}
	c.Assert(hand.IsBlackjack(), Equals, false)
	hand = BlackjackHand{
		{CardValue3, CardSuitClubs},
		{CardValue9, CardSuitClubs},
	}
	c.Assert(hand.IsBlackjack(), Equals, false)
}
