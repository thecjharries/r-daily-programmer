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
	"sort"
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

func (s *MainSuite) TestIntConcatenationLess(c *C) {
	input := IntConcatenation{1, 2, 3}
	c.Assert(input.Less(0, 1), Equals, true)
	c.Assert(input.Less(0, 2), Equals, true)
	c.Assert(input.Less(1, 2), Equals, true)
}

func (s *MainSuite) TestIntConcatenationSort(c *C) {
	var input IntConcatenation
	input = IntConcatenation{5, 56, 50}
	sort.Sort(input)
	c.Assert(input, DeepEquals, IntConcatenation{50, 5, 56})
	input = IntConcatenation{79, 82, 34, 83, 69}
	sort.Sort(sort.Reverse(input))
	c.Assert(input, DeepEquals, IntConcatenation{83, 82, 79, 69, 34})
}
