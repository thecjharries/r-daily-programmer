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
	getStringInputReader *strings.Reader
}

const getStringInputInput string = "test\n"
const getStringInputOutput string = "test"
const lenValidCreds int = 1

var _ = Suite(&MainSuite{
	getStringInputReader: strings.NewReader(getStringInputInput),
})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestGetStringInput(c *C) {
	input := getStringInput("", s.getStringInputReader)
	c.Assert(input, Equals, getStringInputOutput)
}

func (s *MainSuite) TestLoadValidCredentials(c *C) {
	creds := loadValidCredentials(validCredsPath)
	c.Assert(len(creds), Equals, lenValidCreds)
}
