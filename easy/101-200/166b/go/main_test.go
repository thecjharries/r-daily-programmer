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

func (s *MainSuite) TestPlanetVolume(c *C) {
	var planet Planet
	planet = Planet{Radius: 3104500}
	c.Assert(planet.Volume(), Equals, 1.2533247092362817e+20)
	planet = Planet{Radius: 7636500}
	c.Assert(planet.Volume(), Equals, 1.8653987335682594e+21)
}

func (s *MainSuite) TestPlanetMass(c *C) {
	var planet Planet
	planet = Planet{Radius: 3104500, AverageDensity: 5009}
	c.Assert(planet.Mass(), Equals, 6.277903468564534e+23)
	planet = Planet{Radius: 7636500, AverageDensity: 4966}
	c.Assert(planet.Mass(), Equals, 9.263570110899977e+24)
}

func (s *MainSuite) TestPlanetCalculateRoundedWeight(c *C) {
	var planet Planet
	planet = Planet{Radius: 3104500, AverageDensity: 5009}
	c.Assert(planet.CalculateRoundedWeight(100), Equals, 434.467)
	planet = Planet{Radius: 7636500, AverageDensity: 4966}
	c.Assert(planet.CalculateRoundedWeight(100), Equals, 1059.536)

}
