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

func (s *MainSuite) TestGetHorizontalRule(c *C) {
	c.Assert(getHorizontalRule(1), Equals, "------------")
}

func (s *MainSuite) TestGetHeaderRow(c *C) {
	c.Assert(getHeaderRow("+", 4), Equals, " +  |  0  1  2  3  4 ")
}

func (s *MainSuite) TestGetTableRow(c *C) {
	operation := func(a, b int) int { return a + b }
	c.Assert(getTableRow(0, 4, operation), Equals, " 0  |  0  1  2  3  4 ")
}

func (s *MainSuite) TestBuildTable(c *C) {
	c.Assert(buildTable("+", 4), Equals, " +  |  0  1  2  3  4 \n---------------------\n 0  |  0  1  2  3  4 \n 1  |  1  2  3  4  5 \n 2  |  2  3  4  5  6 \n 3  |  3  4  5  6  7 \n 4  |  4  5  6  7  8 ")
}
