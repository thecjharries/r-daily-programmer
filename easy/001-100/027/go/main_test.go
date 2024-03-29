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

type isLeapYearFixture struct {
	year       int
	isLeapYear bool
}

var isLeapYearFixtures = []isLeapYearFixture{
	{1, false},
	{4, true},
	{100, false},
	{400, true},
}

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestIsLeapYear(c *C) {
	for _, fixture := range isLeapYearFixtures {
		c.Assert(fixture.isLeapYear, Equals, isLeapYear(fixture.year))
	}
}

func (s *MainSuite) TestGetCentury(c *C) {
	c.Assert(getCentury(1899), Equals, 19)
	c.Assert(getCentury(1900), Equals, 19)
	c.Assert(getCentury(1901), Equals, 20)
}
