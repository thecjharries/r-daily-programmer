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

func (s *MainSuite) TestDindPossibleChairs(c *C) {
	var goldilocks Seat
	var seats []Seat
	goldilocks = Seat{100, 80}
	seats = []Seat{
		{30, 50},
		{130, 75},
		{90, 60},
		{150, 85},
		{120, 70},
		{200, 200},
		{110, 100},
	}
	c.Assert(findPossibleChairs(goldilocks, seats), DeepEquals, []int{1, 4})
	goldilocks = Seat{100, 120}
	seats = []Seat{
		{297, 90},
		{66, 110},
		{257, 113},
		{276, 191},
		{280, 129},
		{219, 163},
		{254, 193},
		{86, 153},
		{206, 147},
		{71, 137},
		{104, 40},
		{238, 127},
		{52, 146},
		{129, 197},
		{144, 59},
		{157, 124},
		{210, 59},
		{11, 54},
		{268, 119},
		{261, 121},
		{12, 189},
		{186, 108},
		{174, 21},
		{77, 18},
		{54, 90},
		{174, 52},
		{16, 129},
		{59, 181},
		{290, 123},
		{248, 132},
	}
	c.Assert(findPossibleChairs(goldilocks, seats), DeepEquals, []int{0, 2, 10, 14, 16, 18, 21, 22, 25})
}
