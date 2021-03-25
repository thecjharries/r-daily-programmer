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

func (s *MainSuite) TestGenerateFibonacciSequenceLessThan(c *C) {
	output := []int64{0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89}
	c.Assert(generateFibonacciSequenceLessThan(100, []int64{}), DeepEquals, output)
}

func (s *MainSuite) TestReduceFibonacciSequenceToBeLessThan(c *C) {
	fibonacciSequence := []int64{0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89}
	var input int64
	var output []int64
	input = 70
	output = fibonacciSequence[:11]
	c.Assert(reduceFibonacciSequenceToBeLessThan(input, fibonacciSequence), DeepEquals, output)
	input = 20
	output = fibonacciSequence[:8]
	c.Assert(reduceFibonacciSequenceToBeLessThan(input, fibonacciSequence), DeepEquals, output)
}

func (s *MainSuite) TestGenerateZeckendorfRepresentation(c *C) {
	var input int64
	var output []int64
	input = 100
	output = []int64{89, 8, 3}
	c.Assert(generateZeckendorfRepresentation(input), DeepEquals, output)
	input = 14348907
	output = []int64{9227465, 3524578, 1346269, 196418, 46368, 6765, 987, 55, 2}
	c.Assert(generateZeckendorfRepresentation(input), DeepEquals, output)
}
