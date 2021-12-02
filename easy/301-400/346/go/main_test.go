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
	"fmt"
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct{}

var _ = Suite(&MainSuite{})

var printCallCount int
var printSpyContents string

func printSpy(a ...interface{}) (n int, err error) {
	printSpyContents = fmt.Sprint(a...)
	printCallCount++
	return
}

func (s *MainSuite) SetUpTest(c *C) {
	printCallCount = 0
	printSpyContents = ""
	zPrint = printSpy
}

func (s *MainSuite) TearDownTest(c *C) {
	zPrint = fmt.Println
}

func (s *MainSuite) TestMain(c *C) {
	c.Assert(printCallCount, Equals, 0)
	c.Assert(printSpyContents, Equals, "")
	main()
	c.Assert(printCallCount, Equals, 1)
	c.Assert(printSpyContents, Equals, "hello world")
}

func (s *MainSuite) TestIsValid(c *C) {
	c.Assert(isValid("1000"), Equals, true)
	c.Assert(isValid("0123456789"), Equals, false)
}

func (s *MainSuite) TestBuildLetterSet(c *C) {
	c.Assert(buildLetterSet([]string{"SEND", "MORE", "MONEY"}), DeepEquals, []string{"S", "E", "N", "D", "M", "O", "R", "Y"})
}

func (s *MainSuite) TestBuildLetterMapping(c *C) {
	c.Assert(buildLetterMapping([]string{"S", "E", "N", "D", "M", "O", "R", "Y"}, []string{"0", "1", "2", "3", "4", "5", "6", "7", "8", "9"}), DeepEquals, map[string]string{"D": "3", "E": "1", "M": "4", "N": "2", "O": "5", "R": "6", "S": "0", "Y": "7"})
}

func (s *MainSuite) TestConvertWordToNumber(c *C) {
	c.Assert(convertWordToNumber("SEND", map[string]string{"D": "3", "E": "1", "M": "4", "N": "2", "O": "5", "R": "6", "S": "0", "Y": "7"}), Equals, -1)
	c.Assert(convertWordToNumber("MORE", map[string]string{"D": "3", "E": "1", "M": "4", "N": "2", "O": "5", "R": "6", "S": "0", "Y": "7"}), Equals, 4561)
}

func (s *MainSuite) TestIsMappingValid(c *C) {
	c.Assert(isMappingValid([]string{"SEND", "MORE", "MONEY"}, map[string]string{"D": "3", "E": "1", "M": "4", "N": "2", "O": "5", "R": "6", "S": "0", "Y": "7"}), Equals, false)
	c.Assert(isMappingValid([]string{"SEND", "MORE", "MONEY"}, map[string]string{"D": "7", "E": "5", "M": "1", "N": "6", "O": "0", "R": "8", "S": "9", "Y": "2"}), Equals, true)
	c.Assert(isMappingValid([]string{"SEND", "MORE", "MONEY"}, map[string]string{"D": "3", "E": "1", "M": "0", "N": "2", "O": "5", "R": "6", "S": "4", "Y": "7"}), Equals, false)
	c.Assert(isMappingValid([]string{"SEND", "MORE", "MONEY"}, map[string]string{"D": "3", "E": "1", "M": "4", "N": "2", "O": "5", "R": "6", "S": "0", "Y": "7"}), Equals, false)
}

func (s *MainSuite) TestFindWordMapping(c *C) {
	c.Assert(findWordMapping([]string{"SEND", "MORE", "MONEY"}), DeepEquals, map[string]string{"D": "7", "E": "5", "M": "1", "N": "6", "O": "0", "R": "8", "S": "9", "Y": "2"})
}
