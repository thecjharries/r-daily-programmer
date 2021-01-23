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

const (
	minFromIntRange int = 0
	maxFromIntRage int = 10
	randomStringLength int = 20
)

var intRange = []int{1, 0, 10}

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestMin(c *C) {
	minInt := min(intRange...)
	c.Assert(minInt, Equals, minFromIntRange)
}

func (s *MainSuite) TestMax(c *C) {
	maxInt := max(intRange...)
	c.Assert(maxInt, Equals, maxFromIntRage)
}

func (s *MainSuite) TestRandomIntInRange(c *C) {
	c.Assert(randomIntInRange(1, 1), Equals, 1)
}

func (s *MainSuite) TestRandomAsciiCharacterInRange(c *C) {
	randomCharacter := randomAsciiCharacterInRange(characterLowerBound, characterUpperBound)
	c.Assert(int(randomCharacter[0]) >= characterLowerBound, Equals, true)
	c.Assert(int(randomCharacter[0]) <= characterUpperBound, Equals, true)
}

func (s *MainSuite) TestGenerateRandomCharactersInBoundsOfLength(c *C) {
	randomString := generateRandomCharactersInBoundsOfLength(
		characterLowerBound,
		characterUpperBound,
		randomStringLength,
	)
	c.Assert(len(randomString), Equals, randomStringLength)
}
