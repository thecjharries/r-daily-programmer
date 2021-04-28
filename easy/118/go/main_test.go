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
	"time"

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

func (s *MainSuite) TestFormatTime(c *C) {
	testTime, _ := time.Parse(time.RFC3339Nano, "2021-04-28T17:41:44.68072657-05:00")
	c.Assert(formatTime("%l", testTime), Equals, "680")
	c.Assert(formatTime("%s", testTime), Equals, "44")
	c.Assert(formatTime("%m", testTime), Equals, "41")
	c.Assert(formatTime("%h", testTime), Equals, "5")
	c.Assert(formatTime("%H", testTime), Equals, "17")
	c.Assert(formatTime("%c", testTime), Equals, "PM")
	c.Assert(formatTime("%d", testTime), Equals, "28")
	c.Assert(formatTime("%M", testTime), Equals, "4")
	c.Assert(formatTime("%y", testTime), Equals, "2021")
	c.Assert(formatTime("%s.%l", testTime), Equals, "44.680")
	c.Assert(formatTime("%s:%m:%h %M/%d/%y", testTime), Equals, "44:41:5 4/28/2021")
	c.Assert(formatTime("The minute is %m! The hour is %h.", testTime), Equals, "The minute is 41! The hour is 5.")
}
