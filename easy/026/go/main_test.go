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

type removeConsecutiveDuplicateRunesFixture struct{
	input []rune
	stripped []rune
	duplicates []rune
}

var removeConsecutiveDuplicateRunesFixtures = []removeConsecutiveDuplicateRunesFixture{
	{[]rune("ddaaiillyypprrooggrraammeerr"), []rune("dailyprogramer"), []rune("dailyprogramer")},
	{[]rune("aabbccddeded"), []rune("abcdeded"), []rune("abcd")},
	{[]rune("flabby aapples"), []rune("flaby aples"), []rune("bap")},
}

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestRemoveConsecutiveDuplicateRunes(c *C) {
	for _, fixture := range removeConsecutiveDuplicateRunesFixtures {
		stripped, duplicates := removeConsecutiveDuplicateRunes(fixture.input)
		c.Assert(stripped, DeepEquals, fixture.stripped)
		c.Assert(duplicates, DeepEquals, fixture.duplicates)
	}
}
