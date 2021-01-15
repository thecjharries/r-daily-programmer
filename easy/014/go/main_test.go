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

func (s *MainSuite) TestReverseSort(c *C) {
	c.Assert(reverseSort([]int{1, 2, 3}), DeepEquals, []int{3, 2, 1})
}

func (s *MainSuite) TestReverseSortByBlock(c *C) {
	c.Assert(
		reverseSortByBlock([]int{12, 24, 32, 44, 55, 66}, 2),
		DeepEquals,
		[]int{24, 12, 44, 32, 66, 55},
	)
}
