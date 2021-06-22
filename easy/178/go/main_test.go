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
	c.Assert(printSpyContents, Equals, "(-4.000000, -7.000000)")
}

func (s *MainSuite) TestNewPoint2d(c *C) {
	output := &Point2d{0.1, 0.2}
	c.Assert(NewPoint2d(0.1, 0.2), DeepEquals, output)
}

func (s *MainSuite) TestPoint2dTranslate(c *C) {
	point := NewPoint2d(0, 5)
	c.Assert(point.X, Equals, 0.0)
	c.Assert(point.Y, Equals, 5.0)
	point.Translate(3, 2)
	c.Assert(point.X, Equals, 3.0)
	c.Assert(point.Y, Equals, 7.0)
}

func (s *MainSuite) TestPoint2dScale(c *C) {
	point := NewPoint2d(3, 7)
	c.Assert(point.X, Equals, 3.0)
	c.Assert(point.Y, Equals, 7.0)
	point.Scale(1, 3, 0.5)
	c.Assert(point.X, Equals, 2.0)
	c.Assert(point.Y, Equals, 5.0)
}

func (s *MainSuite) TestPoint2dRotate(c *C) {
	point := NewPoint2d(2, 5)
	c.Assert(point.X, Equals, 2.0)
	c.Assert(point.Y, Equals, 5.0)
	point.Rotate(3, 2, math.Pi/2)
	c.Assert(point.X, Equals, 6.0)
	c.Assert(point.Y, Equals, 3.0)
}

func (s *MainSuite) TestPoint2dReflect(c *C) {
	point := NewPoint2d(3, 3)
	c.Assert(point.X, Equals, 3.0)
	c.Assert(point.Y, Equals, 3.0)
	point.Reflect(true, false)
	c.Assert(point.X, Equals, 3.0)
	c.Assert(point.Y, Equals, -3.0)
	point.Reflect(true, true)
	c.Assert(point.X, Equals, -3.0)
	c.Assert(point.Y, Equals, 3.0)
}

func (s *MainSuite) TestPoint2dString(c *C) {
	point := NewPoint2d(1, 1)
	c.Assert(point.String(), Equals, "(1.000000, 1.000000)")
}
