// Copyright 2020 CJ Harries
// Licensed under http://www.apache.org/licenses/LICENSE-2.0

package main

import (
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct {}

type rotateAlphabetFixture struct {
	places int
	result string
}

var rotateAlphabetFixtures = []rotateAlphabetFixture{
	{0, "abcdefghijklmnopqrstuvwxyz"},
	{26, "abcdefghijklmnopqrstuvwxyz"},
	{1, "bcdefghijklmnopqrstuvwxyza"},
	{27, "bcdefghijklmnopqrstuvwxyza"},
}

type stringInputResultFixture struct {
	input string
	result string
}

var translateCharacterFixtures = []stringInputResultFixture{
	{"a", "a"},
	{"-", ""},
}

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestRotateAlphabet(c *C) {
	for _, fixture := range rotateAlphabetFixtures {
		result := rotateAlphabet(fixture.places)
		c.Assert(result, Equals, fixture.result)
	}
}

func (s *MainSuite) TestTranslateCharacter(c *C) {
	for _, fixture := range translateCharacterFixtures {
		result := translateCharacter(fixture.input, alphabet)
		c.Assert(result, Equals, fixture.result)
	}
}
