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

func (s *MainSuite) TestIntConcatenationLen(c *C) {
	input := IntConcatenation{1, 2, 3}
	c.Assert(input.Len(), Equals, 3)
}

func (s *MainSuite) TestIntConcatenationSwap(c *C) {
	input := IntConcatenation{1, 2, 3}
	c.Assert(input, DeepEquals, IntConcatenation{1, 2, 3})
	input.Swap(0, 2)
	c.Assert(input, DeepEquals, IntConcatenation{3, 2, 1})
}

//func (s *MainSuite) TestFindLargestConcatenation(c *C) {
//	c.Assert(findLargestConcatenation([]int{5, 56, 50}), Equals, 56550)
//	c.Assert(findLargestConcatenation([]int{79, 82, 34, 83, 69}), Equals, 8382796934)
//	c.Assert(findLargestConcatenation([]int{420, 34, 19, 71, 341}), Equals, 714203434119)
//	c.Assert(findLargestConcatenation([]int{17, 32, 91, 7, 46}), Equals, 917463217)
//}
