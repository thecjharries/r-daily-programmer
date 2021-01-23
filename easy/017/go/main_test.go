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

func (s *MainSuite) TestGetStarsForLine(c *C) {
	c.Assert(getStarsForLine(-1), Equals, "@")
	c.Assert(getStarsForLine(1), Equals, "@")
	c.Assert(getStarsForLine(2), Equals, "@@")
}

func (s *MainSuite) TestFindLengthOfLongestLine(c *C) {
	lines := []string{
		"one",
		"two",
		"three",
	}
	c.Assert(findLengthOfLongestLine(lines), Equals, 5)
}

func (s *MainSuite) TestRightJustifyStringSlice(c *C) {
	justifiedLines := []string{
		"  one",
		"  two",
		"three",
	}
	unjustifiedLines := []string{
		"one",
		"two",
		"three",
	}
	c.Assert(
		rightJustifyStringSlice(unjustifiedLines),
		DeepEquals,
		justifiedLines,
	)
}

func (s *MainSuite) TestReverseStringSlice(c *C) {
	input := []string{
		"  one",
		"  two",
		"three",
		"one",
		"two",
		"three",
	}
	output := []string{
		"three",
		"two",
		"one",
		"three",
		"  two",
		"  one",
	}
	c.Assert(reverseStringSlice(input), DeepEquals, output)
}

func (s *MainSuite) TestGenerateLines(c *C) {
	output := []string{
		"@",
		"@@",
		"@@@@",
	}
	c.Assert(generateLines(1), DeepEquals, output[:1])
	c.Assert(generateLines(2), DeepEquals, output[:2])
	c.Assert(generateLines(3), DeepEquals, output[:3])
}
