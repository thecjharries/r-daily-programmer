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

type ParseSingleEquationFixture struct {
	Input string
	A, B  float64
}

func (s *MainSuite) TestParseSingleEquation(c *C) {
	fixtures := []ParseSingleEquationFixture{
		{"y=5x-4", 5.0, -4.0},
		{"y=2x+2", 2.0, 2.0},
		{"y=-5x", -5.0, 0.0},
		{"y=-4x+1", -4.0, 1.0},
		{"y=0.5", 0.0, 0.5},
	}
	for _, fixture := range fixtures {
		a, b := parseSingleEquation(fixture.Input)
		c.Assert(a, Equals, fixture.A)
		c.Assert(b, Equals, fixture.B)
	}
}

func (s *MainSuite) TestSolveEquationPair(c *C) {
	var x, y float64
	x, y = solveEquationPair("y=2x+2", "y=5x-4")
	c.Assert(x, Equals, 2.0)
	c.Assert(y, Equals, 6.0)
	x, y = solveEquationPair("y=-5x", "y=-4x+1")
	c.Assert(x, Equals, -1.0)
	c.Assert(y, Equals, 5.0)
	x, y = solveEquationPair("y=0.5x+1.3", "y=-1.4x-0.2")
	c.Assert(x, Equals, -0.7894736842105263)
	c.Assert(y, Equals, 0.9052631578947369)
}
