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

type rotateAlphabetFixture struct {
	places int
	result string
}

var rotateAlphabetFixtures = []rotateAlphabetFixture{
	{0, "abcdefghijklmnopqrstuvwxyz"},
	{26, "abcdefghijklmnopqrstuvwxyz"},
	{1, "bcdefghijklmnopqrstuvwxyza"},
	{27, "bcdefghijklmnopqrstuvwxyza"},
	{-1, "zabcdefghijklmnopqrstuvwxy"},
	{-27, "zabcdefghijklmnopqrstuvwxy"},
}

type stringInputResultFixture struct {
	input  string
	result string
}

var translateCharacterFixtures = []stringInputResultFixture{
	{"a", "a"},
	{"-", ""},
}

const translateStringInput string = "test"
const translateStringResult string = "test"

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

func (s *MainSuite) TestTranslateString(c *C) {
	c.Assert(translateStringResult, Equals, translateString(translateStringInput, alphabet))
}

func (s *MainSuite) TestNewCaesarCipher(c *C) {
	cipher := NewCaesarCipher(0)
	c.Assert(cipher.places, Equals, 0)
	c.Assert(cipher.encryptAlphabet, Equals, alphabet)
	c.Assert(cipher.decryptAlphabet, Equals, alphabet)
}

func (s *MainSuite) TestEncrypt(c *C) {
	cipher := NewCaesarCipher(0)
	c.Assert(translateStringResult, Equals, cipher.Encrypt(translateStringInput))
}

func (s *MainSuite) TestDecrypt(c *C) {
	cipher := NewCaesarCipher(0)
	c.Assert(translateStringResult, Equals, cipher.Decrypt(translateStringInput))
}
