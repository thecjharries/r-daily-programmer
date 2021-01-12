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

func (s *MainSuite) TestGetMonthTerm(c *C) {
	c.Assert(getMonthTerm(1), Equals, 13)
	c.Assert(getMonthTerm(2), Equals, 14)
	c.Assert(getMonthTerm(3), Equals, 3)
	c.Assert(getMonthTerm(4), Equals, 4)
	c.Assert(getMonthTerm(5), Equals, 5)
	c.Assert(getMonthTerm(6), Equals, 6)
	c.Assert(getMonthTerm(7), Equals, 7)
	c.Assert(getMonthTerm(8), Equals, 8)
	c.Assert(getMonthTerm(9), Equals, 9)
	c.Assert(getMonthTerm(10), Equals, 10)
	c.Assert(getMonthTerm(11), Equals, 11)
	c.Assert(getMonthTerm(12), Equals, 12)
}

func (s *MainSuite) TestGetYearTerm(c *C) {
	c.Assert(getYearTerm(1, 2000), Equals, 1999)
	c.Assert(getYearTerm(2, 2000), Equals, 1999)
	c.Assert(getYearTerm(3, 2000), Equals, 2000)
	c.Assert(getYearTerm(4, 2000), Equals, 2000)
	c.Assert(getYearTerm(5, 2000), Equals, 2000)
	c.Assert(getYearTerm(6, 2000), Equals, 2000)
	c.Assert(getYearTerm(7, 2000), Equals, 2000)
	c.Assert(getYearTerm(8, 2000), Equals, 2000)
	c.Assert(getYearTerm(9, 2000), Equals, 2000)
	c.Assert(getYearTerm(10, 2000), Equals, 2000)
	c.Assert(getYearTerm(11, 2000), Equals, 2000)
	c.Assert(getYearTerm(12, 2000), Equals, 2000)
}

func (s *MainSuite) TestGetFirstTwoDigitsOfYear(c *C) {
	c.Assert(getFirstTwoDigitsOfYear(1999), Equals, 19)
	c.Assert(getFirstTwoDigitsOfYear(2000), Equals, 20)
}

func (s *MainSuite) TestGetLastTwoDigitsOfYear(c *C) {
	c.Assert(getLastTwoDigitsOfYear(1999), Equals, 99)
	c.Assert(getLastTwoDigitsOfYear(2000), Equals, 0)
}

func (s *MainSuite) TestGetDayOfWeek(c *C) {
	c.Assert(
		func() {
			getDayOfWeek(-1)
		},
		Panics,
		errorUnknownDayOfWeek,
	)
	c.Assert(getDayOfWeek(1), Equals, "Sunday")
	c.Assert(getDayOfWeek(2), Equals, "Monday")
	c.Assert(getDayOfWeek(3), Equals, "Tuesday")
	c.Assert(getDayOfWeek(4), Equals, "Wednesday")
	c.Assert(getDayOfWeek(5), Equals, "Thursday")
	c.Assert(getDayOfWeek(6), Equals, "Friday")
	c.Assert(getDayOfWeek(0), Equals, "Saturday")
}

func (s *MainSuite) TestZellersAlgorithm(c *C) {
	// https://en.wikipedia.org/wiki/Zeller%27s_congruence#Examples
	c.Assert(zellersAlgorithm(1, 1, 2000), Equals, 0)
	c.Assert(zellersAlgorithm(1, 3, 2000), Equals, 4)
	// Today is Tuesday, 2021/01/11
	c.Assert(zellersAlgorithm(11, 1, 2021), Equals, 2)
}
