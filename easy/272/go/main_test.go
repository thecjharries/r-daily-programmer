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

func (s *MainSuite) TestNewEnglishScrabbleTiles(c *C) {
	tiles := NewEnglishScrabbleTiles()
	c.Assert(len(tiles), Equals, 27)
}

func (s *MainSuite) TestScrabbleTilesRemoveTile(c *C) {
	tiles := NewEnglishScrabbleTiles()
	c.Assert(func() { tiles.RemoveTile("1") }, Panics, "No such tile")
	c.Assert(func() { tiles.RemoveTile("Z") }, Not(Panics), "")
	c.Assert(func() { tiles.RemoveTile("Z") }, Panics, "There are no remaining tiles of that letter")
}

func (s *MainSuite) TestScrabbleTilesRemoveManyTiles(c *C) {
	tiles := NewEnglishScrabbleTiles()
	c.Assert(tiles["A"], Equals, 9)
	tiles.RemoveManyTiles("AEERTYOXMCNB_S")
	c.Assert(tiles["A"], Equals, 8)
}

func (s *MainSuite) TestScrableTilesPrintRemaining(c *C) {
	tiles := NewEnglishScrabbleTiles()
	tiles.RemoveManyTiles("AEERTYOXMCNB_S")
	c.Assert(tiles.PrintRemaining(), Equals, "10: E\n9: I\n8: A\n7: O\n5: N, R, T\n4: D, L, U\n3: G, S\n2: F, H, P, V, W\n1: B, C, J, K, M, Q, Y, Z, _\n0: X")
}
