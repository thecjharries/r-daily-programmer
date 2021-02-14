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

func (s *MainSuite) TestCreateRowWhiteFirst(c *C) {
	output := []string{
		"   ###",
		"   ###",
		"   ###",
	}
	c.Assert(createRow(2, true), DeepEquals, output)
}

func (s *MainSuite) TestCreateRowBlackFirst(c *C) {
	output := []string{
		"###   ",
		"###   ",
		"###   ",
	}
	c.Assert(createRow(2, false), DeepEquals, output)
}

func (s *MainSuite) TestCreateBoard(c *C) {
	output := []string{
		"   ###   ",
		"   ###   ",
		"   ###   ",
		"###   ###",
		"###   ###",
		"###   ###",
	}
	c.Assert(createBoard(2, 3), DeepEquals, output)
}
