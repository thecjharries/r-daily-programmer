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

func (s *MainSuite) TestReverse(c *C) {
	var input, result []int
	input = []int{1, 2, 3, 4, 5}
	result = []int{1, 2, 3, 4, 5}
	c.Assert(reverse(1, input), DeepEquals, result)
	result = []int{2, 1, 3, 4, 5}
	c.Assert(reverse(2, input), DeepEquals, result)
	result = []int{5, 4, 3, 2, 1}
	c.Assert(reverse(5, input), DeepEquals, result)
	c.Assert(reverse(50, input), DeepEquals, result)
	input = []int{51, 41, 12, 62, 74}
	result = []int{12, 41, 51, 62, 74}
	c.Assert(reverse(3, input), DeepEquals, result)
}
