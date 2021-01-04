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

const (
	minFromIntRange int = 0
	maxFromIntRage int = 10
)

var intRange = []int{1, 0, 10}

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestMin(c *C) {
	minInt := min(intRange...)
	c.Assert(minInt, Equals, minFromIntRange)
}

func (s *MainSuite) TestMax(c *C) {
	maxInt := max(intRange...)
	c.Assert(maxInt, Equals, maxFromIntRage)
}

func (s *MainSuite) TestRandomIntInRange(c *C) {
	c.Assert(randomIntInRange(1, 1), Equals, 1)
}
