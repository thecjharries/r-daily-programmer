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

func (s *MainSuite) TestMixedSliceSplitInHalf(c *C) {
	first := MixedSlice{1, 2, 3, 4}
	firstResult := []MixedSlice{{1, 2}, {3, 4}}
	returnedFirst, returnedSecond := first.SplitInHalf()
	c.Assert(returnedFirst, DeepEquals, firstResult[0])
	c.Assert(returnedSecond, DeepEquals, firstResult[1])
	second := MixedSlice{"a", "b", "c"}
	secondResult := []MixedSlice{{"a", "b"}, {"c"}}
	returnedFirst, returnedSecond = second.SplitInHalf()
	c.Assert(returnedFirst, DeepEquals, secondResult[0])
	c.Assert(returnedSecond, DeepEquals, secondResult[1])

}
