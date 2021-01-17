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

func (s *MainSuite) TestReverseStringSlice(c *C) {
	input := []string{
		"  one",
		"  two",
		"three",
		"one",
		"two",
		"three",
	}
	output := []string{
		"three",
		"two",
		"one",
		"three",
		"  two",
		"  one",
	}
	c.Assert(reverseStringSlice(input), DeepEquals, output)
}

func (s *MainSuite) TestGenerateLines(c *C) {
	output := []string{
		"@",
		"@@",
		"@@@@",
	}
	c.Assert(generateLines(1), DeepEquals, output[:1])
	c.Assert(generateLines(2), DeepEquals, output[:2])
	c.Assert(generateLines(3), DeepEquals, output[:3])
}
