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
	buildOutputReader *strings.Reader
}

const getInputInput string = "test\n"
const getInputOutput string = "test"
const buildResultInput string = "qqq\n"
const buildResultOutput string = "your name is qqq, you are  years old, and your username is "

var _ = Suite(&MainSuite{
	getInputReader: strings.NewReader(getInputInput),
	buildOutputReader: strings.NewReader(buildResultInput),
})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestGetInput(c *C) {
	input := getInput("", s.getInputReader)
	c.Assert(input, Equals, getInputOutput)
}

func (s *MainSuite) TestBuildPrompt(c *C) {
	result := buildResult(s.buildOutputReader)
	c.Assert(result, Equals, buildResultOutput)
}
