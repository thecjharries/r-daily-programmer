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

type continuedFractionFixture struct {
	n           int64
	m           int64
	numerator   int64
	denominator int64
}

var continuedFractionFixtures = []continuedFractionFixture{
	{1, 1, 1, 1},
	{10, 1, 1, 1},
	{1, 2, 2, 1},
	{1, 3, 8, 7},
	{1, 4, 60, 46},
}

type piFixture struct {
	precision   int64
	numerator   int64
	denominator int64
}

var piFixtures = []piFixture{
	{1, 4, 1},
	{2, 4, 2},
	{3, 28, 8},
	{4, 184, 60},
}

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestContinuedFraction(c *C) {
	for _, fixture := range continuedFractionFixtures {
		numerator, denominator := continuedFraction(fixture.n, fixture.m)
		c.Assert(numerator, Equals, fixture.numerator)
		c.Assert(denominator, Equals, fixture.denominator)
	}
}

func (s *MainSuite) TestPi(c *C) {
	for _, fixture := range piFixtures {
		numerator, denominator := pi(fixture.precision)
		c.Assert(numerator, Equals, fixture.numerator)
		c.Assert(denominator, Equals, fixture.denominator)
	}
}
