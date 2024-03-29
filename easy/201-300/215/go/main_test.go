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

func (s *MainSuite) TestFloat64SliceIndexOf(c *C) {
	slice := Float64Slice{1, 2, 3, 4, 5}
	c.Assert(slice.IndexOf(10), Equals, -1)
	c.Assert(slice.IndexOf(5), Equals, 4)
}

func (s *MainSuite) TestExplodeAndSumNumber(c *C) {
	c.Assert(explodeAndSumNumber(2, 12), Equals, 5.0)
	c.Assert(explodeAndSumNumber(2, 5), Equals, 25.0)
	c.Assert(explodeAndSumNumber(2, 25), Equals, 29.0)
}

func (s *MainSuite) TestFindSadCycles(c *C) {
	c.Assert(findSadCycle(2, 13), DeepEquals, Float64Slice{1})
	c.Assert(findSadCycle(2, 12), DeepEquals, Float64Slice{89, 145, 42, 20, 4, 16, 37, 58})
	c.Assert(findSadCycle(5, 117649), DeepEquals, Float64Slice{10933, 59536, 73318, 50062})
}
