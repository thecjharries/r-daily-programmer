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
	"math/rand"
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
	rand.Seed(0)
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

func (s *MainSuite) TestShuffleIntSlice(c *C) {
	var input, firstOutput, secondOutput []int
	input = []int{1, 2, 3, 4, 5, 6, 7, 8}
	firstOutput = shuffleIntSlice(input)
	secondOutput = shuffleIntSlice(input)
	c.Assert(firstOutput, Not(DeepEquals), secondOutput)
	c.Assert(firstOutput, DeepEquals, []int{3, 4, 2, 5, 8, 6, 7, 1})
	c.Assert(secondOutput, DeepEquals, []int{1, 3, 2, 6, 8, 4, 5, 7})
}

func (s *MainSuite) TestShuffleIntSliceFisherYates(c *C) {
	var input, firstOutput, secondOutput []int
	input = []int{1, 2, 3, 4, 5, 6, 7, 8}
	firstOutput = shuffleIntSliceFisherYaters(input)
	secondOutput = shuffleIntSliceFisherYaters(input)
	c.Assert(firstOutput, Not(DeepEquals), secondOutput)
	c.Assert(firstOutput, DeepEquals, []int{2, 8, 7, 5, 3, 4, 1, 6})
	c.Assert(secondOutput, DeepEquals, []int{6, 5, 2, 3, 7, 4, 8, 1})
}
