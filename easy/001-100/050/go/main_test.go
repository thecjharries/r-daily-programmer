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

func (s *MainSuite) TestFindFirstTwoItemsThatSumToTotal(c *C) {
	c.Assert(findFirstTwoItemsThatSumToTotal(100, []int{5, 75, 25}), DeepEquals, [2]int{1, 2})
	c.Assert(findFirstTwoItemsThatSumToTotal(200, []int{150, 24, 79, 50, 88, 345, 3}), DeepEquals, [2]int{0, 3})
	c.Assert(findFirstTwoItemsThatSumToTotal(8, []int{2, 1, 9, 4, 4, 56, 90, 3}), DeepEquals, [2]int{3, 4})
	c.Assert(findFirstTwoItemsThatSumToTotal(100, []int{1, 1, 1, 1}), DeepEquals, [2]int{})
}
