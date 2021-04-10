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

func (s *MainSuite) TestRunLengthTupleString(c *C) {
	var tuple RunLengthTuple
	tuple = RunLengthTuple{10, "l"}
	c.Assert(tuple.String(), Equals, "llllllllll")
	tuple = RunLengthTuple{1, "e"}
	c.Assert(tuple.String(), Equals, "e")
}

func (s *MainSuite) TestRunLengthEncodingString(c *C) {
	encoding := RunLengthEncoding{{1, "H"}, {5, "e"}, {5, "l"}, {5, "o"}, {1, "n"}, {1, "u"}, {1, "r"}, {1, "s"}, {1, "e"}}
	c.Assert(encoding.String(), Equals, "Heeeeelllllooooonurse")
}

func (s *MainSuite) TestNewRunLengthEncoding(c *C) {
	c.Assert(NewRunLengthEncoding("Heeeeelllllooooonurse"), DeepEquals, RunLengthEncoding{{1, "H"}, {5, "e"}, {5, "l"}, {5, "o"}, {1, "n"}, {1, "u"}, {1, "r"}, {1, "s"}, {1, "e"}})
}
