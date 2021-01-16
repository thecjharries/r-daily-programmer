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

func (s *MainSuite) TestRemoveCharactersFromString(c *C) {
	c.Assert(removeCharactersFromString("a", "aaa"), Equals, "")
	c.Assert(removeCharactersFromString("]", "a]a"), Equals, "aa")
	c.Assert(removeCharactersFromString("-", "a-a"), Equals, "aa")
	c.Assert(removeCharactersFromString("^", "a^a"), Equals, "aa")
	c.Assert(removeCharactersFromString("aeiou ", "Daily Programmer"), Equals, "DlyPrgrmmr")

}
