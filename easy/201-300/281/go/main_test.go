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

func (s *MainSuite) TestFindBaseUpTo16(c *C) {
	c.Assert(findBaseUpTo16("1"), Equals, 2)
	c.Assert(findBaseUpTo16("21"), Equals, 3)
	c.Assert(findBaseUpTo16("ab3"), Equals, 12)
	c.Assert(findBaseUpTo16("ff"), Equals, 16)
	c.Assert(findBaseUpTo16("z"), Equals, 16)
}

func (s *MainSuite) TestFindLowestBaseAndBase10(c *C) {
	c.Assert(findLowestBaseAndBase10("1"), Equals, "base 2 => 1")
	c.Assert(findLowestBaseAndBase10("21"), Equals, "base 3 => 7")
	c.Assert(findLowestBaseAndBase10("ab3"), Equals, "base 12 => 1575")
	c.Assert(findLowestBaseAndBase10("ff"), Equals, "base 16 => 255")
}
