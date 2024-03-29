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
	"math/rand"
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct{}

var _ = Suite(&MainSuite{})

const newSeed int64 = 0

var oldSeed int64

func (s *MainSuite) SetUpTest(c *C) {
	oldSeed = zSeed
	zSeed = newSeed
	rand.Seed(zSeed)
}

func (s *MainSuite) TearDownTest(c *C) {
	zSeed = oldSeed
}

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestGetRandomLetters(c *C) {
	c.Assert(getRandomLetters(0), Equals, "")
	c.Assert(getRandomLetters(1), Equals, "c")
	c.Assert(getRandomLetters(10), Equals, "ubyhizzkak")
}

func (s *MainSuite) TestPadPlaintext(c *C) {
	c.Assert(padPlaintext(3, "qqq"), Equals, "qqq")
	c.Assert(padPlaintext(4, "qqq"), Equals, "qqqc")
	c.Assert(padPlaintext(10, "qqq"), Equals, "qqqubyhizz")
}

func (s *MainSuite) TestEncrypt(c *C) {
	c.Assert(encrypt(3, "The cake is a lie!"), Equals, "T kiaihces eea  l!")
	c.Assert(encrypt(7, "The cake is a lie!"), Equals, "Telh ieie s!c caauk b")
}

func (s *MainSuite) TestDecrypt(c *C) {
	c.Assert(decrypt(3, "T kiaihces eea  l!"), Equals, "The cake is a lie!")
	c.Assert(decrypt(7, "Telh ieie s!c caauk b"), Equals, "The cake is a lie!cub")
}
