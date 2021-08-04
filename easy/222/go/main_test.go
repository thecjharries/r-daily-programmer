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

func (s *MainSuite) TestGetLetterValue(c *C) {
	c.Assert(getLetterValue('A'), Equals, 1)
	c.Assert(getLetterValue('B'), Equals, 2)
	c.Assert(getLetterValue('C'), Equals, 3)
	c.Assert(getLetterValue('D'), Equals, 4)
	c.Assert(getLetterValue('E'), Equals, 5)
	c.Assert(getLetterValue('F'), Equals, 6)
	c.Assert(getLetterValue('G'), Equals, 7)
	c.Assert(getLetterValue('H'), Equals, 8)
	c.Assert(getLetterValue('I'), Equals, 9)
	c.Assert(getLetterValue('J'), Equals, 10)
	c.Assert(getLetterValue('K'), Equals, 11)
	c.Assert(getLetterValue('L'), Equals, 12)
	c.Assert(getLetterValue('M'), Equals, 13)
	c.Assert(getLetterValue('n'), Equals, 14)
	c.Assert(getLetterValue('o'), Equals, 15)
	c.Assert(getLetterValue('p'), Equals, 16)
	c.Assert(getLetterValue('q'), Equals, 17)
	c.Assert(getLetterValue('r'), Equals, 18)
	c.Assert(getLetterValue('s'), Equals, 19)
	c.Assert(getLetterValue('t'), Equals, 20)
	c.Assert(getLetterValue('u'), Equals, 21)
	c.Assert(getLetterValue('v'), Equals, 22)
	c.Assert(getLetterValue('w'), Equals, 23)
	c.Assert(getLetterValue('x'), Equals, 24)
	c.Assert(getLetterValue('y'), Equals, 25)
	c.Assert(getLetterValue('z'), Equals, 26)
	c.Assert(getLetterValue('1'), Equals, 0)
}
