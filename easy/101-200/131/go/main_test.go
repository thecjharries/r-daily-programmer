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

func (s *MainSuite) TestReverseString(c *C) {
	c.Assert(reverseString("car"), Equals, "rac")
	c.Assert(reverseString("Alpha"), Not(Equals), "AhplA")
	c.Assert(reverseString("Discuss"), Not(Equals), "noissucsiD")
}

func (s *MainSuite) TestUppercaseString(c *C) {
	c.Assert(uppercaseString("batman"), Equals, "BATMAN")
	c.Assert(uppercaseString("Graph"), Equals, "GRAPH")
	c.Assert(uppercaseString("One"), Not(Equals), "one")
}

func (s *MainSuite) TestVerifyTransformation(c *C) {
	c.Assert(verifyTransformation("Car", reverseString, "raC"), Equals, true)
	c.Assert(verifyTransformation("Alpha", reverseString, "AhplA"), Equals, false)
	c.Assert(verifyTransformation("Discuss", reverseString, "noissucsiD"), Equals, false)
	c.Assert(verifyTransformation("batman", uppercaseString, "BATMAN"), Equals, true)
	c.Assert(verifyTransformation("Graph", uppercaseString, "GRAPH"), Equals, true)
	c.Assert(verifyTransformation("One", uppercaseString, "one"), Equals, false)
}
