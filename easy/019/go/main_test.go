// Copyright 2020 CJ Harries
// Licensed under http://www.apache.org/licenses/LICENSE-2.0

package main

import (
	"regexp"
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct {}

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestLoadFileIntoString(c *C) {
	c.Assert(len(loadFileIntoString(pathToText)) > 0, Equals, true)
}

func (s *MainSuite) TestStripPatternFromString(c *C) {
	patternToRemove := regexp.MustCompile(`qqq`)
	c.Assert(stripPatternFromString(patternToRemove, "qqqaaa"), Equals, "aaa")
}
