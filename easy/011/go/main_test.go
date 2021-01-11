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
	c.Assert(getMonthTerm(1), Equals, 11)
	c.Assert(getMonthTerm(2), Equals, 12)
	c.Assert(getMonthTerm(3), Equals, 1)
	c.Assert(getMonthTerm(4), Equals, 2)
	c.Assert(getMonthTerm(5), Equals, 3)
	c.Assert(getMonthTerm(6), Equals, 4)
	c.Assert(getMonthTerm(7), Equals, 5)
	c.Assert(getMonthTerm(8), Equals, 6)
	c.Assert(getMonthTerm(9), Equals, 7)
	c.Assert(getMonthTerm(10), Equals, 8)
	c.Assert(getMonthTerm(11), Equals, 9)
	c.Assert(getMonthTerm(12), Equals, 10)
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
