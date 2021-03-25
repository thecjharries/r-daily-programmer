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
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct{}

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestConvertToUsd(c *C) {
	c.Assert(convertToUsd(10.005), Equals, 10.01)
}

func (s *MainSuite) TestReduceCostByDenomination(c *C) {
	var reduced float64
	var count int
	reduced, count = reduceCostByDenomination(24, 20)
	c.Assert(reduced, Equals, 4.0)
	c.Assert(count, Equals, 1)
	reduced, count = reduceCostByDenomination(0.40, 0.01)
	c.Assert(reduced, Equals, 0.0)
	c.Assert(count, Equals, 40)
}

func (s *MainSuite) TestConvertCostToDenominations(c *C) {
	var cost float64
	var output []int
	cost = 12.33
	output = []int{0, 0, 1, 0, 2, 1, 0, 1, 3}
	c.Assert(convertCostToDenominations(cost, usdDenominations), DeepEquals, output)
}
