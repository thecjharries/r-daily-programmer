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
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct {}

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestGetDaysInMonthNormally(c *C) {
	monthSlice := getDaysInMonthSlice(false)
	c.Assert(monthSlice[1], Equals, 28)
}

func (s *MainSuite) TestGetDaysInMonthLeap(c *C) {
	monthSlice := getDaysInMonthSlice(true)
	c.Assert(monthSlice[1], Equals, 29)
}

func (s *MainSuite) TestGetSumOfDaysBefore(c *C) {
	c.Assert(getSumOfDaysBefore(-1, true), Equals, 0)
	c.Assert(getSumOfDaysBefore(1, true), Equals, 0)
	c.Assert(getSumOfDaysBefore(2, true), Equals, 31)
	c.Assert(getSumOfDaysBefore(3, true), Equals, 60)
	c.Assert(getSumOfDaysBefore(4, true), Equals, 91)
	c.Assert(getSumOfDaysBefore(5, true), Equals, 121)
	c.Assert(getSumOfDaysBefore(6, true), Equals, 152)
	c.Assert(getSumOfDaysBefore(7, true), Equals, 182)
	c.Assert(getSumOfDaysBefore(8, true), Equals, 213)
	c.Assert(getSumOfDaysBefore(9, true), Equals, 244)
	c.Assert(getSumOfDaysBefore(10, true), Equals, 274)
	c.Assert(getSumOfDaysBefore(11, true), Equals, 305)
	c.Assert(getSumOfDaysBefore(12, true), Equals, 335)
	c.Assert(getSumOfDaysBefore(22, true), Equals, 366)
}

func (s *MainSuite) TestGetDayNumber(c *C) {
	c.Assert(getDayNumber(1, 3, true), Equals, 61)
	c.Assert(getDayNumber(1, 3, false), Equals, 60)
}

func (s *MainSuite) TestBrokennessOfGetDayNumber(c *C) {
	c.Assert(getDayNumber(-1, 1, true), Equals, -1)
	c.Assert(getDayNumber(40, 1, true), Equals, 40)
}
