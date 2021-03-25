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

func (s *MainSuite) TestGetSlidingWindowMinimums(c *C) {
	var input, output []int
	var windowSize int
	input = []int{5, 2, 8, 6, 4, 7}
	windowSize = 20
	c.Assert(getSlidingWindowMinimums(input, windowSize), DeepEquals, output)
	input = []int{4, 3, 2, 1, 5, 7, 6, 8, 9}
	output = []int{2, 1, 1, 1, 5, 6, 6}
	windowSize = 3
	c.Assert(getSlidingWindowMinimums(input, windowSize), DeepEquals, output)
	input = []int{5, 2, 8, 6, 4, 7}
	output = []int{2, 2, 6, 4, 4}
	windowSize = 2
	c.Assert(getSlidingWindowMinimums(input, windowSize), DeepEquals, output)
}
