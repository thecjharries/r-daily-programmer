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

func (s *MainSuite) TestSortDigits(c *C) {
	c.Assert(sortDigits(6589, true), Equals, 5689)
	c.Assert(sortDigits(6589, false), Equals, 9865)
	c.Assert(sortDigits(21, true), Equals, 12)
	c.Assert(sortDigits(21, false), Equals, 2100)
}

func (s *MainSuite) TestKaprekarIterationCount(c *C) {
	c.Assert(kaprekarIterationCount(6589), Equals, 2)
	c.Assert(kaprekarIterationCount(5455), Equals, 5)
	c.Assert(kaprekarIterationCount(6174), Equals, 0)
	c.Assert(kaprekarIterationCount(10), Equals, -1)
	c.Assert(kaprekarIterationCount(3333), Equals, -1)
}
