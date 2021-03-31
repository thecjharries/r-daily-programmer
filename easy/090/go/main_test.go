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

func (s *MainSuite) TestRasterStamp(c *C) {
	raster := Raster{
		Image:    [][]int{{0, 0}},
		CurrentX: 0,
		CurrentY: 0,
	}
	raster.Stamp()
	c.Assert(raster.Image, DeepEquals, [][]int{{1, 0}})
}

func (s *MainSuite) TestRasterMove(c *C) {
	raster := Raster{
		Image:    [][]int{{0, 0}},
		CurrentX: 0,
		CurrentY: 0,
	}
	raster.Move(1, 1)
	c.Assert(raster.CurrentX, Equals, 1)
	c.Assert(raster.CurrentY, Equals, 1)
}

func (s *MainSuite) TestRasterParseAction(c *C) {
	raster := Raster{
		Image:    [][]int{{0, 0}, {0, 0}},
		CurrentX: 0,
		CurrentY: 0,
	}
	raster.ParseAction('P')
	c.Assert(raster.Image, DeepEquals, [][]int{{1, 0}, {0, 0}})
	raster.ParseAction('S')
	c.Assert(raster.CurrentX, Equals, 0)
	c.Assert(raster.CurrentY, Equals, 1)
	raster.ParseAction('N')
	c.Assert(raster.CurrentX, Equals, 0)
	c.Assert(raster.CurrentY, Equals, 0)
	raster.ParseAction('E')
	c.Assert(raster.CurrentX, Equals, 1)
	c.Assert(raster.CurrentY, Equals, 0)
	raster.ParseAction('W')
	c.Assert(raster.CurrentX, Equals, 0)
	c.Assert(raster.CurrentY, Equals, 0)
}
