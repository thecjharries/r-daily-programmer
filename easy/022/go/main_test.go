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

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestMixedSliceContains(c *C) {
	mixedSlice := MixedSlice{"1", 2}
	c.Assert(mixedSlice.Contains(1), Equals, false)
	c.Assert(mixedSlice.Contains("1"), Equals, true)
	c.Assert(mixedSlice.Contains(2), Equals, true)
	c.Assert(mixedSlice.Contains("2"), Equals, false)
}

func (s *MainSuite) TestMixedSliceAddMissingElements(c *C) {
	first := MixedSlice{1, 2}
	second := MixedSlice{2, 3, 3}
	result := MixedSlice{1, 2, 3}
	c.Assert(first, Not(DeepEquals), result)
	first.AddMissingElements(second)
	c.Assert(first, DeepEquals, result)
	third := MixedSlice{"a", "b", "c", 1, 4}
	fourth := MixedSlice{"a", "x", 34, "4"}
	result = MixedSlice{"a", "b", "c", 1, 4, "x", 34, "4"}
	c.Assert(third, Not(DeepEquals), result)
	third.AddMissingElements(fourth)
	c.Assert(third, DeepEquals, result)
}
