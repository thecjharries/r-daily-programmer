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

func (s *MainSuite) TestNewPromptTriangle(c *C) {
	result := NewPromptTriangle(PromptTriangle{
		a: 1,
		b: 1,
		c: 1,
		A: 1,
		B: 1,
		C: 1,
	})
	c.Assert(result.a, Equals, 1.0)
	c.Assert(result.b, Equals, 1.0)
	c.Assert(result.c, Equals, 1.0)
	c.Assert(result.A, Equals, 1.0)
	c.Assert(result.B, Equals, 1.0)
	c.Assert(result.C, Equals, math.Pi/2)
}

func (s *MainSuite) TestPromptTriangleComputeEdgeC(c *C) {
	triangle := PromptTriangle{
		a: 4,
		b: 3,
	}
	c.Assert(triangle.c, Equals, 0.0)
	triangle.ComputeEdgeC()
	c.Assert(triangle.c, Equals, 5.0)
}
