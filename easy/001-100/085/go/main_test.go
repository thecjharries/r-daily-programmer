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

func (s *MainSuite) TestSum(c *C) {
	c.Assert(sum(1, 2, 3), Equals, 6)
}

func (s *MainSuite) TestComputeRowSums(c *C) {
	matrix := []int{10, 5, 4, 20, 9, 33, 27, 16, 11, 6, 55, 3, 11, 6, 55, 3}
	numRows := 4
	numCols := 4
	output := map[int][][]int{
		39: {{10, 5, 4, 20}},
		85: {{9, 33, 27, 16}},
		75: {{11, 6, 55, 3}, {11, 6, 55, 3}},
	}
	c.Assert(computeRowSums(matrix, numRows, numCols), DeepEquals, output)
}

func (s *MainSuite) TestComputeColumnSums(c *C) {
	matrix := []int{10, 5, 4, 20, 20, 9, 33, 27, 16, 16, 11, 6, 55, 3, 3}
	numRows := 3
	numCols := 5
	output := map[int][]int{
		30: {0},
		44: {1},
		86: {2},
		39: {3, 4},
	}
	c.Assert(computeColumnSums(matrix, numRows, numCols), DeepEquals, output)
}
