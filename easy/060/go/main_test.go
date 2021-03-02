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

func (s *MainSuite) TestRemoveEvenDivisors(c *C) {
	c.Assert(removeEvenDivisors(2), Equals, 1)
	c.Assert(removeEvenDivisors(3), Equals, 3)
	c.Assert(removeEvenDivisors(24), Equals, 3)
}

func (s *MainSuite) TestDeterminePoliteness(c *C) {
	c.Assert(determinePolitenessOfInt(90), Equals, 5)
	c.Assert(determinePolitenessOfInt(15), Equals, 3)
	c.Assert(determinePolitenessOfInt(32), Equals, 0)
}

func (s *MainSuite) TestCreateIntRangeInclusive(c *C) {
	c.Assert(createIntRangeInclusive(2, 5), DeepEquals, []int{2,3,4,5})
}
