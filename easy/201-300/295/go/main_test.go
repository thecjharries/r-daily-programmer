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

func (s *MainSuite) TestConvertLetterByLetter(c *C) {
	c.Assert(convertLetterByLetter("floor", "brake"), DeepEquals, []string{
		"floor",
		"bloor",
		"broor",
		"braor",
		"brakr",
		"brake",
	})
	c.Assert(convertLetterByLetter("wood", "book"), DeepEquals, []string{
		"wood",
		"bood",
		"book",
	})
	c.Assert(convertLetterByLetter("a fall to the floor", "braking the door in"), DeepEquals, []string{
		"a fall to the floor",
		"b fall to the floor",
		"brfall to the floor",
		"braall to the floor",
		"brakll to the floor",
		"brakil to the floor",
		"brakin to the floor",
		"brakingto the floor",
		"braking o the floor",
		"braking t the floor",
		"braking ththe floor",
		"braking thehe floor",
		"braking the e floor",
		"braking the d floor",
		"braking the dofloor",
		"braking the dooloor",
		"braking the dooroor",
		"braking the door or",
		"braking the door ir",
		"braking the door in",
	})
}
