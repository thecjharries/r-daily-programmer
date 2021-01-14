// Copyright 2020 CJ Harries
// Licensed under http://www.apache.org/licenses/LICENSE-2.0

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
