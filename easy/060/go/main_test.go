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

func (s *MainSuite) TestIsIntPolite(c *C) {
	c.Assert(isIntPolite(1), Equals, false)
	c.Assert(isIntPolite(2), Equals, false)
	c.Assert(isIntPolite(3), Equals, true)
	c.Assert(isIntPolite(4), Equals, false)
	c.Assert(isIntPolite(5), Equals, true)
	c.Assert(isIntPolite(6), Equals, true)
	c.Assert(isIntPolite(7), Equals, true)
	c.Assert(isIntPolite(8), Equals, false)
	c.Assert(isIntPolite(9), Equals, true)
	c.Assert(isIntPolite(10), Equals, true)
	c.Assert(isIntPolite(11), Equals, true)
	c.Assert(isIntPolite(12), Equals, true)
	c.Assert(isIntPolite(13), Equals, true)
	c.Assert(isIntPolite(14), Equals, true)
	c.Assert(isIntPolite(15), Equals, true)
	c.Assert(isIntPolite(16), Equals, false)
	c.Assert(isIntPolite(17), Equals, true)
	c.Assert(isIntPolite(18), Equals, true)
	c.Assert(isIntPolite(19), Equals, true)
	c.Assert(isIntPolite(20), Equals, true)
	c.Assert(isIntPolite(21), Equals, true)
	c.Assert(isIntPolite(22), Equals, true)
	c.Assert(isIntPolite(23), Equals, true)
	c.Assert(isIntPolite(24), Equals, true)
	c.Assert(isIntPolite(25), Equals, true)
	c.Assert(isIntPolite(26), Equals, true)
	c.Assert(isIntPolite(27), Equals, true)
	c.Assert(isIntPolite(28), Equals, true)
	c.Assert(isIntPolite(29), Equals, true)
	c.Assert(isIntPolite(30), Equals, true)
	c.Assert(isIntPolite(31), Equals, true)
	c.Assert(isIntPolite(32), Equals, false)
}

func (s *MainSuite) TestRemoveEvenDivisors(c *C) {
	c.Assert(removeEvenDivisors(2), Equals, 1)
	c.Assert(removeEvenDivisors(3), Equals, 3)
	c.Assert(removeEvenDivisors(24), Equals, 3)
}

func (s *MainSuite) TestDeterminePoliteness(c *C) {
	c.Assert(determinePolitenessOfInt(90), Equals, 5)
	c.Assert(determinePolitenessOfInt(15), Equals, 3)
}
