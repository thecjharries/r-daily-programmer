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

func (s *MainSuite) TestProcessKeyPressNoModifiers(c *C) {
	c.Assert(processKeyPress('a', false, false), Equals, 'a')
	c.Assert(processKeyPress('0', false, false), Equals, '0')
	c.Assert(processKeyPress('z', false, false), Equals, 'z')
}

func (s *MainSuite) TestProcessKeyPressWithShiftNoCaps(c *C) {
	c.Assert(processKeyPress('a', true, false), Equals, 'A')
	c.Assert(processKeyPress('0', true, false), Equals, ')')
	c.Assert(processKeyPress('z', true, false), Equals, 'z')
}

func (s *MainSuite) TestProcessKeyPressNoShiftWithCaps(c *C) {
	c.Assert(processKeyPress('a', false, true), Equals, 'A')
	c.Assert(processKeyPress('0', false, true), Equals, '0')
	c.Assert(processKeyPress('z', false, true), Equals, 'z')
}


func (s *MainSuite) TestProcessKeyPressWithShiftWithCaps(c *C) {
	c.Assert(processKeyPress('a', true, true), Equals, 'A')
	c.Assert(processKeyPress('0', true, true), Equals, ')')
	c.Assert(processKeyPress('z', true, true), Equals, 'z')
}
