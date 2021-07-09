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

func (s *MainSuite) TestNewIntegerSet(c *C) {
	input := []int{1, 1, 2, 2, 3, 3, 4, 4}
	output := IntegerSet{
		1: struct{}{},
		2: struct{}{},
		3: struct{}{},
		4: struct{}{},
	}
	set := NewIntegerSet(input...)
	c.Assert(set, DeepEquals, output)
}

func (s *MainSuite) TestIntegerSetContains(c *C) {
	set := IntegerSet{
		1: struct{}{},
		2: struct{}{},
		3: struct{}{},
		4: struct{}{},
	}
	c.Assert(set.Contains(1), Equals, true)
	c.Assert(set.Contains(5), Equals, false)
}

func (s *MainSuite) TestIntegerSetUnion(c *C) {
	firstSet := NewIntegerSet(1, 2, 3)
	secondSet := NewIntegerSet(4, 5, 6)
	finalSet := NewIntegerSet(1, 2, 3, 4, 5, 6)
	c.Assert(firstSet.Union(secondSet), DeepEquals, finalSet)
	c.Assert(secondSet.Union(finalSet), DeepEquals, finalSet)
}

func (s *MainSuite) TestIntegerSetIntersection(c *C) {
	firstSet := NewIntegerSet(1, 2, 3)
	secondSet := NewIntegerSet(4, 5, 6)
	emptySet := NewIntegerSet()
	c.Assert(firstSet.Intersection(secondSet), DeepEquals, emptySet)
	c.Assert(secondSet.Intersection(firstSet), DeepEquals, emptySet)
	thirdSet := NewIntegerSet(1, 4, 5, 6)
	subSet := NewIntegerSet(1)
	c.Assert(firstSet.Intersection(thirdSet), DeepEquals, subSet)
	c.Assert(thirdSet.Intersection(firstSet), DeepEquals, subSet)
}

func (s *MainSuite) TestIntegerSetEquals(c *C) {
	firstSet := NewIntegerSet(1, 2, 3)
	secondSet := NewIntegerSet(4, 5, 6)
	thirdSet := NewIntegerSet(1, 2, 3)
	fourthSet := NewIntegerSet(1, 2, 3, 4)
	c.Assert(firstSet.Equals(secondSet), Equals, false)
	c.Assert(secondSet.Equals(firstSet), Equals, false)
	c.Assert(firstSet.Equals(thirdSet), Equals, true)
	c.Assert(thirdSet.Equals(firstSet), Equals, true)
	c.Assert(firstSet.Equals(fourthSet), Equals, false)
	c.Assert(fourthSet.Equals(firstSet), Equals, false)
}

func (s *MainSuite) TestIntegerSetString(c *C) {
	set := NewIntegerSet(1, 2, 3)
	output := "{1, 2, 3}"
	c.Assert(set.String(), Equals, output)
}
