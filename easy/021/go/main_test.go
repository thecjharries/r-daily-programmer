// Copyright 2020 CJ Harries
// Licensed under http://www.apache.org/licenses/LICENSE-2.0

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
	result := []int64{1, 2, 3}
	c.Assert(convertPermutationsToInts(inputPermutations), DeepEquals, result)
}
