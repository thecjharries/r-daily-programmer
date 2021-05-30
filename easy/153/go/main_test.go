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
	c.Assert(printSpyContents, Equals, "[[1 14 91 364 1001 2002 3003 3432 3003 2002 1001 364 91 14 1] [14 182 1092 4004 10010 18018 24024 24024 18018 10010 4004 1092 182 14] [91 1092 6006 20020 45045 72072 84084 72072 45045 20020 6006 1092 91] [364 4004 20020 60060 120120 168168 168168 120120 60060 20020 4004 364] [1001 10010 45045 120120 210210 252252 210210 120120 45045 10010 1001] [2002 18018 72072 168168 252252 252252 168168 72072 18018 2002] [3003 24024 84084 168168 210210 168168 84084 24024 3003] [3432 24024 72072 120120 120120 72072 24024 3432] [3003 18018 45045 60060 45045 18018 3003] [2002 10010 20020 20020 10010 2002] [1001 4004 6006 4004 1001] [364 1092 1092 364] [91 182 91] [14 14] [1]]")
}

func (s *MainSuite) TestFactorial(c *C) {
	c.Assert(factorial(1), Equals, 1)
	c.Assert(factorial(2), Equals, 2)
	c.Assert(factorial(3), Equals, 6)
}

func (s *MainSuite) TestPascalsPyramid(c *C) {
	c.Assert(pascalsPyramid(0), DeepEquals, [][]int{{1}})
	c.Assert(pascalsPyramid(1), DeepEquals, [][]int{{1, 1}, {1}})
	c.Assert(pascalsPyramid(2), DeepEquals, [][]int{{1, 2, 1}, {2, 2}, {1}})
}
