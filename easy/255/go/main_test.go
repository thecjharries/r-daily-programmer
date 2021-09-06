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

func (s *MainSuite) TestNewSwitches(c *C) {
	c.Assert(len(NewSwitches(10)), Equals, 10)
}

func (s *MainSuite) TestSwitchesToggle(c *C) {
	switches := NewSwitches(10)
	c.Assert(switches[3], Equals, false)
	switches.Toggle(3, 4)
	c.Assert(switches[3], Equals, true)
	c.Assert(switches[7], Equals, false)
	switches.Toggle(7, 5)
	c.Assert(switches[7], Equals, true)
}

func (s *MainSuite) TestSwitchesGetOnCount(c *C) {
	switches := NewSwitches(10)
	c.Assert(switches.GetOnCount(), Equals, 0)
	switches.Toggle(3, 6)
	switches.Toggle(0, 4)
	switches.Toggle(7, 3)
	switches.Toggle(9, 9)
	c.Assert(switches.GetOnCount(), Equals, 7)
	challenge := NewSwitches(1000)
	challenge.Toggle(616, 293)
	challenge.Toggle(344, 942)
	challenge.Toggle(27, 524)
	challenge.Toggle(716, 291)
	challenge.Toggle(860, 284)
	challenge.Toggle(74, 928)
	challenge.Toggle(970, 594)
	challenge.Toggle(832, 772)
	challenge.Toggle(343, 301)
	challenge.Toggle(194, 882)
	challenge.Toggle(948, 912)
	challenge.Toggle(533, 654)
	challenge.Toggle(242, 792)
	challenge.Toggle(408, 34)
	challenge.Toggle(162, 249)
	challenge.Toggle(852, 693)
	challenge.Toggle(526, 365)
	challenge.Toggle(869, 303)
	challenge.Toggle(7, 992)
	challenge.Toggle(200, 487)
	challenge.Toggle(961, 885)
	challenge.Toggle(678, 828)
	challenge.Toggle(441, 152)
	challenge.Toggle(394, 453)
	c.Assert(challenge.GetOnCount(), Equals, 423)
}
