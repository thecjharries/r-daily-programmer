// Copyright 2020 CJ Harries
// Licensed under http://www.apache.org/licenses/LICENSE-2.0

package main

import (
	"strings"
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct {
	getInputReader *strings.Reader
}

const getStringInputInput string = "test\n"
const getStringInputOutput string = "test"

var _ = Suite(&MainSuite{
	getInputReader: strings.NewReader(getStringInputInput),
})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestGetStringInput(c *C) {
	input := getStringInput("", s.getInputReader)
	c.Assert(input, Equals, getStringInputOutput)
}

func (s *MainSuite) TestSortInput(c *C) {
	c.Assert(sortInput("9876543210"), DeepEquals, strings.Split("0123456789", ""))
	c.Assert(
		sortInput("zyxwvutsrqponmlkjihgfedcba"),
		DeepEquals,
		strings.Split("abcdefghijklmnopqrstuvwxyz", ""),
	)
}
