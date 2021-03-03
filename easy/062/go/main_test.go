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

func (s *MainSuite) TestDoesSubsetSumLessThanMax(c *C) {
	c.Assert(doesSubsetSumLessThanMax(10, []float64{1, 2, 3}), Equals, true)
	c.Assert(doesSubsetSumLessThanMax(3, []float64{1, 2, 3}), Equals, false)
}

func (s *MainSuite) TestDoesUllmansPuzzle(c *C) {
	var elements, result, foundResult []float64
	var maxSum float64
	var size int
	var possible bool
	elements = []float64{18.1, 55.1, 91.2, 74.6, 73.0, 85.9, 73.9, 81.4, 87.1, 49.3, 88.8, 5.7, 26.3, 7.1, 58.2, 31.7, 5.8, 76.9, 16.5, 8.1, 48.3, 6.8, 92.4, 83.0, 19.6}
	maxSum = 98.2
	size = 3
	result = []float64{18.1, 55.1, 5.7}
	possible, foundResult = ullmansPuzzle(elements, maxSum, size)
	c.Assert(possible, Equals, true)
	c.Assert(foundResult, DeepEquals, result)
	maxSum = 1
	possible, foundResult = ullmansPuzzle(elements, maxSum, size)
	c.Assert(possible, Equals, false)
	c.Assert(foundResult, DeepEquals, []float64(nil))
}
