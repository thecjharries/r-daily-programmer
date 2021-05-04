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

func (s *MainSuite) TestEdgeString(c *C) {
	edge := Edge{
		Name:   "A",
		Start:  1,
		Finish: 2,
	}
	c.Assert(edge.String(), Equals, "A")
}

func (s *MainSuite) TestGraphFromEdgesLen(c *C) {
	graph := GraphFromEdges{
		{"A", 1, 2},
		{"B", 2, 3},
		{"C", 2, 4},
		{"D", 4, 5},
	}
	c.Assert(graph.Len(), Equals, 4)
}

func (s *MainSuite) TestGraphFromEdgesLess(c *C) {
	graph := GraphFromEdges{
		{"A", 1, 2},
		{"B", 2, 3},
		{"C", 2, 4},
		{"D", 4, 5},
	}
	c.Assert(graph.Less(0, 1), Equals, true)
	c.Assert(graph.Less(1, 2), Equals, true)
	c.Assert(graph.Less(2, 3), Equals, true)
	c.Assert(graph.Less(3, 0), Equals, false)
}

func (s *MainSuite) TestGraphFromEdgesSwap(c *C) {
	graph := GraphFromEdges{
		{"A", 1, 2},
		{"B", 2, 3},
		{"C", 2, 4},
		{"D", 4, 5},
	}
	graphSwapped := GraphFromEdges{
		{"A", 1, 2},
		{"B", 2, 3},
		{"D", 4, 5},
		{"C", 2, 4},
	}
	c.Assert(graph, Not(DeepEquals), graphSwapped)
	graph.Swap(2, 3)
	c.Assert(graph, DeepEquals, graphSwapped)
}
