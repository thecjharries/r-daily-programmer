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

func (s *MainSuite) TestThreeSum(c *C) {
	c.Assert(threeSum([]int{9, -6, -5, 9, 8, 3, -4, 8, 1, 7, -4, 9, -9, 1, 9, -9, 9, 4, -6, -8}), DeepEquals, [][]int{[]int{-5, -4, 9}, []int{-6, 3, 3}, []int{-5, 1, 4}, []int{-4, -4, 8}, []int{-9, 1, 8}, []int{-4, 1, 3}, []int{-8, 1, 7}, []int{-8, 4, 4}})
	c.Assert(threeSum([]int{4, 5, -1, -2, -7, 2, -5, -3, -7, -3, 1}), DeepEquals, [][]int{[]int{-3, -1, 4}, []int{-2, -2, 4}, []int{-5, 1, 4}, []int{-3, -2, 5}, []int{-7, 2, 5}, []int{-1, -1, 2}, []int{-3, 1, 2}})
	c.Assert(threeSum([]int{-1, -6, -3, -7, 5, -8, 2, -8, 1}), DeepEquals, [][]int{[]int{-1, -1, 2}, []int{-6, 1, 5}, []int{-3, 1, 2}, []int{-7, 2, 5}})
	c.Assert(threeSum([]int{-5, -1, -4, 2, 9, -9, -6, -1, -7}), DeepEquals, [][]int{[]int{-5, -4, 9}, []int{-1, -1, 2}, []int{-4, 2, 2}})
}
