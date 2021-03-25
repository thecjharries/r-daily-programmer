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

func (s *MainSuite) TestInsertCharacterIntoEveryPositionOfString(c *C) {
	inputString := []rune("aa")
	inputCharacter := 'b'
	results := []string{
		"baa",
		"aba",
		"aab",
	}
	c.Assert(insertCharacterIntoEveryPositionOfString(inputString, inputCharacter), DeepEquals, results)
}

func (s *MainSuite) TestPermuteStringRecursion(c *C) {
	inputString := []rune("aab")
	results := []string{
		"baa",
		"aba",
		"aab",
		"baa",
		"aba",
		"aab",
	}
	c.Assert(permuteStringRecursion(inputString[1:], []string{string(inputString[0])}), DeepEquals, results)
}

func (s *MainSuite) TestPermuteString(c *C) {
	inputString := "aab"
	results := []string{
		"baa",
		"aba",
		"aab",
		"baa",
		"aba",
		"aab",
	}
	c.Assert(permuteString(inputString), DeepEquals, results)
}

func (s *MainSuite) TestConvertPermutationsToInts(c *C) {
	inputPermutations := []string{"1", "2", "3"}
	result := []int{1, 2, 3}
	c.Assert(convertPermutationsToInts(inputPermutations), DeepEquals, result)
}

func (s *MainSuite) TestFindNextLargestPermutation(c *C) {
	input := 123
	permutations := []int{321, 312, 231, 213, 123, 132}
	c.Assert(findNextLargestPermutation(input, permutations), DeepEquals, 132)
}
