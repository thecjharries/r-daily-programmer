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

func (s *MainSuite) TestNewCoordinate(c *C) {
	c.Assert(NewCoordinate(1, 1), DeepEquals, &Coordinate{1, 1})
}

func (s *MainSuite) TestNewRectangle(c *C) {
	output := &Rectangle{
		TopLeft:     &Coordinate{1, 1},
		BottomRight: &Coordinate{2, 2},
	}
	c.Assert(NewRectangleFromFloat64s(1, 1, 2, 2), DeepEquals, output)
}

func (s *MainSuite) TestRectangleContainsRectangle(c *C) {
	var first, second *Rectangle
	first = &Rectangle{&Coordinate{1, 1}, &Coordinate{10, 10}}
	second = &Rectangle{&Coordinate{1, 2}, &Coordinate{5, 5}}
	c.Assert(first.ContainsRectangle(second), Equals, true)
	second = &Rectangle{&Coordinate{1, 2}, &Coordinate{15, 5}}
	c.Assert(first.ContainsRectangle(second), Equals, false)
}

func (s *MainSuite) TestRectangleContainsCoordinate(c *C) {
	var coordinate *Coordinate
	rectangle := &Rectangle{&Coordinate{1, 1}, &Coordinate{10, 10}}
	coordinate = &Coordinate{1, 1}
	c.Assert(rectangle.ContainsCoordinate(coordinate), Equals, true)
	coordinate = &Coordinate{0, 0}
	c.Assert(rectangle.ContainsCoordinate(coordinate), Equals, false)
}

func (s *MainSuite) TestRectangleIntersectsRectangle(c *C) {
	var first, second *Rectangle
	first = &Rectangle{&Coordinate{1, 1}, &Coordinate{10, 10}}
	second = &Rectangle{&Coordinate{1, 2}, &Coordinate{5, 5}}
	c.Assert(first.IntersectsRectangle(second), Equals, true)
	second = &Rectangle{&Coordinate{11, 12}, &Coordinate{15, 15}}
	c.Assert(first.IntersectsRectangle(second), Equals, false)

}

func (s *MainSuite) TestRectangleIntersectionNoIntersection(c *C) {
	first := &Rectangle{&Coordinate{1, 1}, &Coordinate{2, 2}}
	second := &Rectangle{&Coordinate{2, 2}, &Coordinate{3, 3}}
	c.Assert(rectangleIntersection(first, second), DeepEquals, (*Rectangle)(nil))
}

func (s *MainSuite) TestRectangleIntersectionContainment(c *C) {
	first := &Rectangle{&Coordinate{1, 1}, &Coordinate{10, 10}}
	second := &Rectangle{&Coordinate{1, 2}, &Coordinate{5, 5}}
	c.Assert(rectangleIntersection(first, second), DeepEquals, second)
	c.Assert(rectangleIntersection(second, first), DeepEquals, second)
}
