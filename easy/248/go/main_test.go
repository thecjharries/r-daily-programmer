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

func (s *MainSuite) TestPointString(c *C) {
	point := Point{0, 0, 255}
	c.Assert(point.String(), Equals, "  0   0 255")
}

func (s *MainSuite) TestNewGrid(c *C) {
	grid := NewGrid(1, 1)
	c.Assert(len(*grid), Equals, 1)
	c.Assert((*grid)[0][0].Red, Equals, 0)
}

func (s *MainSuite) TestGridString(c *C) {
	grid := NewGrid(5, 3)
	c.Assert(grid.String(), Equals, "  0   0   0     0   0   0     0   0   0     0   0   0     0   0   0\n  0   0   0     0   0   0     0   0   0     0   0   0     0   0   0\n  0   0   0     0   0   0     0   0   0     0   0   0     0   0   0")
}

func (s *MainSuite) TestGridLine(c *C) {
	grid := NewGrid(5, 3)
	c.Assert((*grid)[0][2].Red, Equals, 0)
	grid.Line(Point{100, 100, 100}, 0, 2, 2, 4)
	c.Assert((*grid)[0][2].Red, Equals, 100)
	c.Assert((*grid)[1][3].Green, Equals, 100)
	c.Assert((*grid)[2][4].Blue, Equals, 100)
}
