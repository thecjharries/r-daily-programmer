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
	reader *strings.Reader
}

const readerInput string = "test\n"
const getInputOutput string = "test"

var _ = Suite(&MainSuite{
	reader: strings.NewReader(readerInput),
})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestGetInput(c *C) {
	input := getInput("", s.reader)
	c.Assert(input, Equals, getInputOutput)
}
