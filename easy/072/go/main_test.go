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

func (s *MainSuite) TestApplyRules(c *C) {
	c.Assert(applyRules([]rune{'2','2','2'}), DeepEquals, []rune{'2','2','2'})
	c.Assert(applyRules([]rune{'1','1','1'}), DeepEquals, []rune{'1','0','1'})
	c.Assert(applyRules([]rune{'1','1','0'}), DeepEquals, []rune{'1','1','0'})
	c.Assert(applyRules([]rune{'1','0','1'}), DeepEquals, []rune{'1','1','1'})
	c.Assert(applyRules([]rune{'0','1','1'}), DeepEquals, []rune{'0','1','1'})
	c.Assert(applyRules([]rune{'0','0','1'}), DeepEquals, []rune{'0','1','1'})
	c.Assert(applyRules([]rune{'0','1','0'}), DeepEquals, []rune{'0','1','0'})
	c.Assert(applyRules([]rune{'1','0','0'}), DeepEquals, []rune{'1','0','0'})
	c.Assert(applyRules([]rune{'0','0','0'}), DeepEquals, []rune{'0','0','0'})
}
