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

func (s *MainSuite) TestCalculateConsecutiveDistanceRating(c *C) {
	c.Assert(calculateConsecutiveDistanceRating([]int{31, 63, 53, 56, 96, 62, 73, 25, 54, 55, 64}), Equals, 26)
	c.Assert(calculateConsecutiveDistanceRating([]int{77, 39, 35, 38, 41, 42, 76, 73, 40, 31, 10}), Equals, 20)
	c.Assert(calculateConsecutiveDistanceRating([]int{30, 63, 57, 87, 37, 31, 58, 83, 34, 76, 38}), Equals, 15)
	c.Assert(calculateConsecutiveDistanceRating([]int{18, 62, 55, 92, 88, 57, 90, 10, 11, 96, 12}), Equals, 3)
	c.Assert(calculateConsecutiveDistanceRating([]int{26, 8, 7, 25, 52, 17, 45, 64, 11, 35, 12}), Equals, 6)
	c.Assert(calculateConsecutiveDistanceRating([]int{89, 57, 21, 55, 56, 81, 54, 100, 22, 62, 50}), Equals, 13)
}
