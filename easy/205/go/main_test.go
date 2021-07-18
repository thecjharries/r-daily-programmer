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

func (s *MainSuite) TestSimplifyDate(c *C) {
	var first, second, output string
	currentYear := time.Now().Year()
	first = fmt.Sprintf("%d-07-01", currentYear)
	second = fmt.Sprintf("%d-07-04", currentYear)
	output = "July 1 - 4"
	c.Assert(simplifyDateRange(first, second), Equals, output)
	first = fmt.Sprintf("%d-12-01", currentYear)
	second = fmt.Sprintf("%d-02-03", currentYear+1)
	output = "December 1 - February 3"
	c.Assert(simplifyDateRange(first, second), Equals, output)
	first = fmt.Sprintf("%d-12-01", currentYear)
	second = fmt.Sprintf("%d-02-03", currentYear+2)
	output = fmt.Sprintf("December 1, %d - February 3, %d", currentYear, currentYear+2)
	c.Assert(simplifyDateRange(first, second), Equals, output)
	first = fmt.Sprintf("%d-03-01", currentYear+1)
	second = fmt.Sprintf("%d-05-05", currentYear+1)
	output = fmt.Sprintf("March 1 - May 5, %d", currentYear+1)
	c.Assert(simplifyDateRange(first, second), Equals, output)
	first = fmt.Sprintf("%d-01-01", currentYear+2)
	second = fmt.Sprintf("%d-01-01", currentYear+2)
	output = fmt.Sprintf("January 1, %d", currentYear+2)
	c.Assert(simplifyDateRange(first, second), Equals, output)
	first = fmt.Sprintf("%d-09-05", currentYear+7)
	second = fmt.Sprintf("%d-09-04", currentYear+8)
	output = fmt.Sprintf("September 5, %d - September 4, %d", currentYear+7, currentYear+8)
	c.Assert(simplifyDateRange(first, second), Equals, output)
	first = fmt.Sprintf("%d-04-01", currentYear)
	second = fmt.Sprintf("%d-09-10", currentYear+5)
	output = fmt.Sprintf("April 1, %d - September 10, %d", currentYear, currentYear+5)
	c.Assert(simplifyDateRange(first, second), Equals, output)
	first = fmt.Sprintf("%d-12-11", currentYear)
	second = fmt.Sprintf("%d-12-11", currentYear+1)
	output = fmt.Sprintf("December 11, %d - December 11, %d", currentYear, currentYear+1)
	c.Assert(simplifyDateRange(first, second), Equals, output)
}
