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
