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

func (s *MainSuite) TestConvertToScaleRepresentation(c *C) {
	c.Assert(convertToScaleRepresentation(1234567891111), DeepEquals, []int{0, 0, 1, 234, 567, 891, 111})
}

func (s *MainSuite) TestbuildScaleRepresentationStrings(c *C) {
	var short, long string
	short, long = buildScaleRepresentationStrings([]int{0, 0, 1, 234, 567, 891, 111})
	c.Assert(short, Equals, "1 trillion, 234 billion, 567 million, 891 thousand, and 111")
	c.Assert(long, Equals, "1 billion, 234 milliard, 567 million, 891 thousand, and 111")
	short, long = buildScaleRepresentationStrings([]int{0, 0, 0, 0, 0, 0, 0})
	c.Assert(short, Equals, "")
	c.Assert(long, Equals, "")
	short, long = buildScaleRepresentationStrings([]int{0, 0, 1, 234, 567, 891, 0})
	c.Assert(short, Equals, "1 trillion, 234 billion, 567 million, 891 thousand")
	c.Assert(long, Equals, "1 billion, 234 milliard, 567 million, 891 thousand")
	short, long = buildScaleRepresentationStrings([]int{0, 0, 0, 0, 0, 891, 111})
	c.Assert(short, Equals, "891 thousand and 111")
	c.Assert(long, Equals, "891 thousand and 111")
}
