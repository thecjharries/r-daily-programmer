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
	"math"
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

func (s *MainSuite) TestNewRectangle(c *C) {
	rectangle := NewRectangle(1, 2, 3, 4)
	c.Assert(rectangle.MinX, Equals, 1.0)
	c.Assert(rectangle.MinY, Equals, 2.0)
	c.Assert(rectangle.MaxX, Equals, 3.0)
	c.Assert(rectangle.MaxY, Equals, 4.0)
}

func (s *MainSuite) TestNewExtremeRectangle(c *C) {
	rectangle := NewExtremeRectangle()
	c.Assert(rectangle.MinX, Equals, math.Inf(1))
	c.Assert(rectangle.MinY, Equals, math.Inf(1))
}

func (s *MainSuite) TestRectangleCover(c *C) {
	circle := Circle{1, 1, 2}
	rectangle := NewExtremeRectangle()
	rectangle.Cover(&circle)
	c.Assert(rectangle.MinX, Equals, -1.0)
	c.Assert(rectangle.MinY, Equals, -1.0)
	c.Assert(rectangle.MaxX, Equals, 3.0)
	c.Assert(rectangle.MaxY, Equals, 3.0)
}

func (s *MainSuite) TestRectangleString(c *C) {
	rectangle := NewRectangle(1, 2, 3, 4)
	c.Assert(rectangle.String(), Equals, "[1.000000, 2.000000, 3.000000, 4.000000]")
}

func (s *MainSuite) TestRectangleCoverAll(c *C) {
	rectangle := NewExtremeRectangle()
	rectangle.CoverAll([]*Circle{
		{1, 1, 2},
		{2, 2, 0.5},
		{-1, -3, 2},
		{5, 2, 1},
	})
	c.Assert(rectangle.String(), Equals, "[-3.000000, -5.000000, 6.000000, 3.000000]")
}
