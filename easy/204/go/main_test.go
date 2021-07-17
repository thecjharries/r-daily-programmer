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

func (s *MainSuite) TestFindLine(c *C) {
	var line, haystack string
	line = "eye of newt"
	haystack = "\n  SECOND WITCH.\n    Fillet of a fenny snake,\n    In the caldron boil and bake;\n    Eye of newt, and toe of frog,\n    Wool of bat, and tongue of dog,\n    Adder's fork, and blind-worm's sting,\n    Lizard's leg, and howlet's wing,\n    For a charm of powerful trouble,\n    Like a hell-broth boil and bubble.\n\n  ALL.\n    Double, double, toil and trouble;\n    Fire, burn; and caldron, bubble."
	c.Assert(findLine(line, haystack), Equals, "    Fillet of a fenny snake,\n    In the caldron boil and bake;\n    Eye of newt, and toe of frog,\n    Wool of bat, and tongue of dog,\n    Adder's fork, and blind-worm's sting,\n    Lizard's leg, and howlet's wing,\n    For a charm of powerful trouble,\n    Like a hell-broth boil and bubble.")
}
