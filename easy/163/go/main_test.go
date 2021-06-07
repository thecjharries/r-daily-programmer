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

func (s *MainSuite) TestDiceRollDistributionSingleRoll(c *C) {
	distribution := NewDiceRollDistribution(6, []int{10, 100})
	c.Assert(distribution.Rolls[0][0], Equals, 0)
	c.Assert(distribution.Rolls[0][1], Equals, 0)
	c.Assert(distribution.Rolls[0][2], Equals, 0)
	c.Assert(distribution.Rolls[0][3], Equals, 0)
	c.Assert(distribution.Rolls[0][4], Equals, 0)
	c.Assert(distribution.Rolls[0][5], Equals, 0)
	c.Assert(distribution.Rolls[1][0], Equals, 0)
	c.Assert(distribution.Rolls[1][1], Equals, 0)
	c.Assert(distribution.Rolls[1][2], Equals, 0)
	c.Assert(distribution.Rolls[1][3], Equals, 0)
	c.Assert(distribution.Rolls[1][4], Equals, 0)
	c.Assert(distribution.Rolls[1][5], Equals, 0)
	distribution.SingleRoll(0)
	c.Assert(distribution.Rolls[0][0], Equals, 1)
	c.Assert(distribution.Rolls[0][1], Equals, 0)
	c.Assert(distribution.Rolls[0][2], Equals, 0)
	c.Assert(distribution.Rolls[0][3], Equals, 0)
	c.Assert(distribution.Rolls[0][4], Equals, 0)
	c.Assert(distribution.Rolls[0][5], Equals, 0)
	c.Assert(distribution.Rolls[1][0], Equals, 1)
	c.Assert(distribution.Rolls[1][1], Equals, 0)
	c.Assert(distribution.Rolls[1][2], Equals, 0)
	c.Assert(distribution.Rolls[1][3], Equals, 0)
	c.Assert(distribution.Rolls[1][4], Equals, 0)
	c.Assert(distribution.Rolls[1][5], Equals, 0)
	distribution.SingleRoll(10)
	c.Assert(distribution.Rolls[0][0], Equals, 1)
	c.Assert(distribution.Rolls[0][1], Equals, 0)
	c.Assert(distribution.Rolls[0][2], Equals, 0)
	c.Assert(distribution.Rolls[0][3], Equals, 0)
	c.Assert(distribution.Rolls[0][4], Equals, 0)
	c.Assert(distribution.Rolls[0][5], Equals, 0)
	c.Assert(distribution.Rolls[1][0], Equals, 2)
	c.Assert(distribution.Rolls[1][1], Equals, 0)
	c.Assert(distribution.Rolls[1][2], Equals, 0)
	c.Assert(distribution.Rolls[1][3], Equals, 0)
	c.Assert(distribution.Rolls[1][4], Equals, 0)
	c.Assert(distribution.Rolls[1][5], Equals, 0)
	distribution.SingleRoll(11)
	distribution.SingleRoll(12)
	distribution.SingleRoll(13)
	c.Assert(distribution.Rolls[0][0], Equals, 1)
	c.Assert(distribution.Rolls[0][1], Equals, 0)
	c.Assert(distribution.Rolls[0][2], Equals, 0)
	c.Assert(distribution.Rolls[0][3], Equals, 0)
	c.Assert(distribution.Rolls[0][4], Equals, 0)
	c.Assert(distribution.Rolls[0][5], Equals, 0)
	c.Assert(distribution.Rolls[1][0], Equals, 2)
	c.Assert(distribution.Rolls[1][1], Equals, 1)
	c.Assert(distribution.Rolls[1][2], Equals, 0)
	c.Assert(distribution.Rolls[1][3], Equals, 0)
	c.Assert(distribution.Rolls[1][4], Equals, 1)
	c.Assert(distribution.Rolls[1][5], Equals, 1)
}

func (s *MainSuite) TestNewDiceRollDistribution(c *C) {
	distribution := NewDiceRollDistribution(6, []int{100, 10})
	c.Assert(distribution.DiceSides, Equals, 6)
	c.Assert(distribution.Counts, DeepEquals, []int{10, 100})
	c.Assert(len(distribution.Rolls), Equals, 2)
}
