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

func (s *MainSuite) TestNewGameOfLife(c *C) {
	game := NewGameOfLife(10, 10, "..........\n..........\n..#.......\n...#......\n.###......\n..........\n..........\n..........\n..........\n..........")
	c.Assert(game.Width, Equals, 10)
	c.Assert(game.Height, Equals, 10)
	c.Assert(game.Map, Equals, "......................#..........#.......###........................................................")
}

func (s *MainSuite) TestGameOfLifeString(c *C) {
	game := NewGameOfLife(10, 10, "..........\n..........\n..#.......\n...#......\n.###......\n..........\n..........\n..........\n..........\n..........")
	c.Assert(game.String(), Equals, "..........\n..........\n..#.......\n...#......\n.###......\n..........\n..........\n..........\n..........\n..........\n")
}

func (s *MainSuite) TestGameOfLifeGetCellNeighborOnCount(c *C) {
	game := NewGameOfLife(10, 10, "..........\n..........\n..#.......\n...#......\n.###......\n..........\n..........\n..........\n..........\n..........")
	c.Assert(game.GetCellNeighborOnCount(0, 0), Equals, 0)
	c.Assert(game.GetCellNeighborOnCount(2, 2), Equals, 1)
	c.Assert(game.GetCellNeighborOnCount(3, 3), Equals, 3)
	c.Assert(game.GetCellNeighborOnCount(9, 9), Equals, 0)
}
