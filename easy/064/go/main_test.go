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

type MainSuite struct {}

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestGreatestCommonDivisor(c *C) {
	c.Assert(greatestCommonDivisor(3,5), Equals, 1)
	c.Assert(greatestCommonDivisor(128, 64), Equals, 64)
	c.Assert(greatestCommonDivisor(90, 105), Equals, 15)
}

func (s *MainSuite) TestGetTotatives(c *C) {
	c.Assert(getTotatives(30), DeepEquals, []int{1, 7, 11, 13, 17, 19, 23, 29})
}
