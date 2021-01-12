// Copyright 2020 CJ Harries
// Licensed under http://www.apache.org/licenses/LICENSE-2.0

package main

import (
	"fmt"
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

func (s *MainSuite) TestPermuteAndPrintStringBaseCase(c *C) {
	callCount := 0
	zPrintFunction = func(a ...interface{}) (int, error){
		callCount++
		fmt.Println(a...)
		return 0, nil
	}
	permuteAndPrint([]rune("a"), 0)
	c.Assert(callCount, Equals, 1)
}

func (s *MainSuite) TestPermuteAndPrintStringRecursiveCase(c *C) {
	callCount := 0
	zPrintFunction = func(a ...interface{}) (int, error){
		callCount++
		fmt.Println(a...)
		return 0, nil
	}
	permuteAndPrint([]rune("abc"), 0)
	c.Assert(callCount, Equals, 6)
}
