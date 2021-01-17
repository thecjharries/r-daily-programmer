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

func (s *MainSuite) TestGetStarsForLine(c *C) {
	c.Assert(getStarsForLine(-1), Equals, "@")
	c.Assert(getStarsForLine(1), Equals, "@")
	c.Assert(getStarsForLine(2), Equals, "@@")
}

func (s *MainSuite) TestFindLengthOfLongestLine(c *C) {
	lines := []string{
		"one",
		"two",
		"three",
	}
	c.Assert(findLengthOfLongestLine(lines), Equals, 5)
}

func (s *MainSuite) TestRightJustifyStringSlice(c *C) {
	justifiedLines := []string{
		"  one",
		"  two",
		"three",
	}
	unjustifiedLines := []string{
		"one",
		"two",
		"three",
	}
	c.Assert(
		rightJustifyStringSlice(unjustifiedLines),
		DeepEquals,
		justifiedLines,
	)
}
