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

func (s *MainSuite) TestNewPoint2d(c *C) {
	output := &Point2d{0.1, 0.2}
	c.Assert(NewPoint2d(0.1, 0.2), DeepEquals, output)
}

func (s *MainSuite) TestPoint2dTranslate(c *C) {
	point := NewPoint2d(0, 0)
	c.Assert(point.X, DeepEquals, 0.0)
	c.Assert(point.Y, DeepEquals, 0.0)
	point.Translate(0, 5)
	c.Assert(point.X, DeepEquals, 0.0)
	c.Assert(point.Y, DeepEquals, 5.0)
}

func (s *MainSuite) TestPoint2dRotate(c *C) {
	point := NewPoint2d(0, 0)
	c.Assert(point.X, DeepEquals, 0.0)
	c.Assert(point.Y, DeepEquals, 0.0)
	point.Rotate(1, 1, 1)
	c.Assert(point.X, DeepEquals, 1.3011686789397567)
	c.Assert(point.Y, DeepEquals, 1.79372701072887)
}

func (s *MainSuite) TestPoint2dScale(c *C) {
	point := NewPoint2d(1, 1)
	c.Assert(point.X, DeepEquals, 1.0)
	c.Assert(point.Y, DeepEquals, 1.0)
	point.Scale(3, 3, 2)
	c.Assert(point.X, DeepEquals, -1.0)
	c.Assert(point.Y, DeepEquals, -1.0)
}
