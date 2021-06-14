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

func (s *MainSuite) TestBlackjackHandValue(c *C) {
	var hand BlackjackHand
	hand = BlackjackHand{
		{CardValueAce, CardSuitClubs},
		{CardValueQueen, CardSuitClubs},
	}
	c.Assert(hand.Value(), Equals, 21)
	hand = BlackjackHand{
		{CardValueAce, CardSuitClubs},
		{CardValueAce, CardSuitClubs},
	}
	c.Assert(hand.Value(), Equals, 12)
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

func (s *MainSuite) TestNewBlackjackHandFromString(c *C) {
	input := "Ace of Diamonds, Ten of Clubs"
	output := BlackjackHand{
		{CardValueAce, CardSuitDiamonds},
		{CardValue10, CardSuitClubs},
	}
	c.Assert(NewBlackjackHandFromString(input), DeepEquals, output)
}

func (s *MainSuite) TestNewBlackjackGameFromStrings(c *C) {
	input := []string{
		"Ace of Diamonds, Ten of Clubs",
		"Three of Hearts, Six of Spades, Seven of Spades",
		"Ten of Hearts, Three of Diamonds, Jack of Clubs",
	}
	output := BlackjackGame{
		{
			{CardValueAce, CardSuitDiamonds},
			{CardValue10, CardSuitClubs},
		},
		{
			{CardValue3, CardSuitHearts},
			{CardValue6, CardSuitSpades},
			{CardValue7, CardSuitSpades},
		},
		{
			{CardValue10, CardSuitHearts},
			{CardValue3, CardSuitDiamonds},
			{CardValueJack, CardSuitClubs},
		},
	}
	c.Assert(NewBlackjackGameFromStrings(input), DeepEquals, output)
}
