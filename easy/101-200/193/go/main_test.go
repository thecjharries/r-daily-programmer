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

func (s *MainSuite) TestCalculateCubeFromVolume(c *C) {
	c.Assert(calculateCubeFromVolume(27.0), Equals, "Cube: 3.00m width, 3.00m, high, 3.00m tall")
}

func (s *MainSuite) TestCalculateCylinderFromVolume(c *C) {
	c.Assert(calculateCylinderFromVolume(27.0), Equals, "Cylinder: 3.00m tall, Diameter of 3.39m")
}

func (s *MainSuite) TestCalculateSphereFromVolume(c *C) {
	c.Assert(calculateSphereFromVolume(27.0), Equals, "Sphere: 1.86m Radius")
}

func (s *MainSuite) TestCalculateConeFromVolume(c *C) {
	c.Assert(calculateConeFromVolume(27.0), Equals, "Cone: 9.00m tall, 1.69m Radius")
}
