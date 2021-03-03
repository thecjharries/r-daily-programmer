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

func (s *MainSuite) TestGenerateCombinations(c *C) {
	var input []float64
	var size int
	var output [][]float64
	input = []float64{1, 2, 3}
	size = 2
	output = [][]float64{{1, 2}, {1, 3}, {2, 3}}
	c.Assert(generateCombinations(size, input), DeepEquals, output)
	input = []float64{1, 2, 3, 4, 5}
	size = 3
	output = [][]float64{
		{1, 2, 3},
		{1, 2, 4},
		{1, 2, 5},
		{1, 3, 4},
		{1, 3, 5},
		{1, 4, 5},
		{2, 3, 4},
		{2, 3, 5},
		{2, 4, 5},
		{3, 4, 5},
	}
	c.Assert(generateCombinations(size, input), DeepEquals, output)
}
