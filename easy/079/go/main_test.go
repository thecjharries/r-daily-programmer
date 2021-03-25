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

func printSpy(format string, a ...interface{}) (n int, err error) {
	printSpyContents = fmt.Sprintf(format, a...)
	printCallCount++
	return
}

func (s *MainSuite) SetUpTest(c *C) {
	printCallCount = 0
	printSpyContents = ""
	zPrint = printSpy
}

func (s *MainSuite) TearDownTest(c *C) {
	zPrint = fmt.Printf
}

func (s *MainSuite) TestMain(c *C) {
	c.Assert(printCallCount, Equals, 0)
	c.Assert(printSpyContents, Equals, "")
	main()
	c.Assert(printCallCount, Equals, 1)
	c.Assert(printSpyContents, Equals, "hello world")
}

func (s *MainSuite) TestStepCountStepsGuard(c *C) {
	c.Assert(func() { stepCount(1, 2, 0) }, PanicMatches, ErrorStepsBelowTwo)
	c.Assert(func() { stepCount(1, 2, 4) }, Not(Panics), nil)
}

func (s *MainSuite) TestStepCountNoIntermediate(c *C) {
	c.Assert(stepCount(1, 2, 2), DeepEquals, []float64{1, 2})
}

func (s *MainSuite) TestStepCountWithSteps(c *C) {
	c.Assert(stepCount(1, 3, 3), DeepEquals, []float64{1, 2, 3})
	c.Assert(stepCount(1, 4, 4), DeepEquals, []float64{1, 2, 3, 4})
	c.Assert(stepCount(1, 2, 3), DeepEquals, []float64{1, 1.5, 2})
	c.Assert(stepCount(18.75, -22.00, 5), DeepEquals, []float64{18.75, 8.5625, -1.625, -11.8125, -22.0})
	c.Assert(stepCount(-5.75, 12.00, 5), DeepEquals, []float64{-5.75, -1.3125, 3.125, 7.5625, 12.0})
	c.Assert(stepCount(13.50, -20.75, 3), DeepEquals, []float64{13.5, -3.625, -20.75})
	c.Assert(stepCount(9.75, 3.00, 9), DeepEquals, []float64{9.75, 8.90625, 8.0625, 7.21875, 6.375, 5.53125, 4.6875, 3.84375, 3.0})
}
