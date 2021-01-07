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

func (s *MainSuite) TestRomanCharacterToMorseCharacterHasSingleCharacterInput(c *C) {
	c.Assert(
		func() {
			romanCharacterToMorseCharacter("aa")
		},
		Panics,
		errorNotSingleCharacter,
	)
}

func (s *MainSuite) TestRomanCharacterToMorseCharacterHasKnownCharacter(c *C) {
	c.Assert(
		func() {
			romanCharacterToMorseCharacter("|")
		},
		Panics,
		errorUnknownCharacter,
	)
}

func (s *MainSuite) TestRomanCharacterToMorseCharacterReturnsProperOutput(c *C) {
	c.Assert(romanCharacterToMorseCharacter("a"), Equals, ".-")
}
