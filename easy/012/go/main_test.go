// Copyright 2021 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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

func (s *MainSuite) TestPermuteAndPrintStringBaseCase(c *C) {
	callCount := 0
	zPrintFunction = func(a ...interface{}) (int, error){
		callCount++
		return 0, nil
	}
	permuteAndPrint([]rune("a"), 0)
	c.Assert(callCount, Equals, 1)
}

func (s *MainSuite) TestPermuteAndPrintStringRecursiveCase(c *C) {
	callCount := 0
	zPrintFunction = func(a ...interface{}) (int, error){
		callCount++
		return 0, nil
	}
	permuteAndPrint([]rune("abc"), 0)
	c.Assert(callCount, Equals, 6)
}
