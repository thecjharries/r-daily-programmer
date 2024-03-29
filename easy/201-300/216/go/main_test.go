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

func (s *MainSuite) TestCardValueString(c *C) {
	c.Assert(CardValue(CardValueAce).String(), Equals, "ace")
}

func (s *MainSuite) TestCardSuitString(c *C) {
	c.Assert(CardSuit(CardSuitClubs).String(), Equals, "clubs")
}

func (s *MainSuite) TestCardsString(c *C) {
	deck := NewDeck(1)
	cards := deck.DealCards(2)
	c.Assert(cards.String(), Equals, "ace of clubs, two of clubs")
}

func (s *MainSuite) TestDeckShuffle(c *C) {
	deck := NewDeck(1)
	c.Assert((*deck)[0].Suit, Equals, CardSuitClubs)
	c.Assert((*deck)[0].Value, Equals, CardValueAce)
	deck.Shuffle()
	c.Assert((*deck)[0].Suit, Equals, CardSuitDiamonds)
	c.Assert((*deck)[0].Value, Equals, CardValue7)
}

func (s *MainSuite) TestDeckDealCards(c *C) {
	deck := NewDeck(1)
	c.Assert(deck.DealCards(-1), DeepEquals, Cards(nil))
	cards := deck.DealCards(1)
	c.Assert(cards[0].Suit, Equals, CardSuitClubs)
	c.Assert(cards[0].Value, Equals, CardValueAce)
	c.Assert(len(cards), Equals, 1)

}

func (s *MainSuite) TestDeckDealSimpleTexasHoldEmHand(c *C) {
	deck := NewDeck(1)
	c.Assert(deck.DealSimpleTexasHoldEmHand(3), Equals, "Your hand: ace of clubs, two of clubs\nCPU 1 hand: three of clubs, four of clubs\nCPU 2 hand: five of clubs, six of clubs\n\nFlop: seven of clubs, eight of clubs, nine of clubs\nTurn: ten of clubs\nRiver: jack of clubs")
}

func (s *MainSuite) TestNewDeck(c *C) {
	deck := NewDeck(1)
	c.Assert(len(*deck), Equals, 52)
}
