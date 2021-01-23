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
	"path"
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct {}

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestReadFileIntoStringSlice(c *C) {
	stringSlice := readFileIntoStringSlice(path.Join(".", "sample.txt"))
	c.Assert(106 == len(stringSlice), Equals, true)
	c.Assert(stringSlice[0], Equals, "https://www.w3.org/TR/PNG/iso_8859-1.txt")
}

func (s *MainSuite) TestFindLengthOfLongestLine(c *C) {
	lines := []string{
		"one",
		"two",
		"three",
	}
	c.Assert(findLengthOfLongestLine(lines), Equals, 5)
}

func (s *MainSuite) TestLeftJustifyStringSlice(c *C) {
	unjustifiedLines := []string{
		"  one",
		"  two",
		"three",
	}
	justifiedLines := []string{
		"one",
		"two",
		"three",
	}
	c.Assert(
		leftJustifyStringSlice(unjustifiedLines),
		DeepEquals,
		justifiedLines,
	)
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
