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

func (s *MainSuite) TestMixedSliceContains(c *C) {
	mixedSlice := MixedSlice{"1", 2}
	c.Assert(mixedSlice.Contains(1), Equals, false)
	c.Assert(mixedSlice.Contains("1"), Equals, true)
	c.Assert(mixedSlice.Contains(2), Equals, true)
	c.Assert(mixedSlice.Contains("2"), Equals, false)
}
