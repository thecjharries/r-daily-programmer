// Copyright 2020 CJ Harries
// Licensed under http://www.apache.org/licenses/LICENSE-2.0

package main

import (
	"path"
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct {}

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestReadFileIntoStringSlice(c *C) {
	stringSlice := readFileIntoStringSlice(path.Join(".", "sample.txt"))
	c.Assert(106 == len(stringSlice), Equals, true)
	c.Assert(stringSlice[0], Equals, "https://www.w3.org/TR/PNG/iso_8859-1.txt")
}

func (s *MainSuite) TestFindLengthOfLongestLine(c *C) {
	lines := []string{
		"one",
		"two",
		"three",
	}
	c.Assert(findLengthOfLongestLine(lines), Equals, 5)
}

func (s *MainSuite) TestLeftJustifyStringSlice(c *C) {
	unjustifiedLines := []string{
		"  one",
		"  two",
		"three",
	}
	justifiedLines := []string{
		"one",
		"two",
		"three",
	}
	c.Assert(
		leftJustifyStringSlice(unjustifiedLines),
		DeepEquals,
		justifiedLines,
	)
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
