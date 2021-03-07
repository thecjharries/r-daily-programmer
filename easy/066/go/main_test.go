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

func (s *MainSuite) TestGetNumeralPosition(c *C) {
	c.Assert(getNumeralPosition('q'), Equals, -1)
	c.Assert(getNumeralPosition('M'), Equals, 0)
	c.Assert(getNumeralPosition('D'), Equals, 1)
	c.Assert(getNumeralPosition('C'), Equals, 2)
	c.Assert(getNumeralPosition('L'), Equals, 3)
	c.Assert(getNumeralPosition('X'), Equals, 4)
	c.Assert(getNumeralPosition('V'), Equals, 5)
	c.Assert(getNumeralPosition('I'), Equals, 6)
}

func (s *MainSuite) TestIsFirstNumeralSmallerThanSecond(c *C) {
	c.Assert(isFirstNumeralSmallerThanSecond('q', 'M'), Equals, false)
	c.Assert(isFirstNumeralSmallerThanSecond('M', 'D'), Equals, false)
	c.Assert(isFirstNumeralSmallerThanSecond('D', 'M'), Equals, true)
}
