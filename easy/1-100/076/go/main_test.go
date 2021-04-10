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

func (s *MainSuite) TestCreateExceptionsMap(c *C) {
	input := []string{"one", "two"}
	output := map[string]bool{
		"one": true,
		"two": true,
	}
	c.Assert(createExceptionsMap(input), DeepEquals, output)
}

func (s *MainSuite) TestTitleCaseWithExceptions(c *C) {
	var phrase, output string
	var exceptions []string
	phrase = "the quick brown fox jumps over the lazy dog"
	exceptions = []string{"jumps", "the", "over"}
	output = "The Quick Brown Fox jumps over the Lazy Dog"
	c.Assert(titleCaseWithExceptions(phrase, exceptions), Equals, output)
	phrase = "THE vitamins ARE IN my fresh CALIFORNIA raisins"
	exceptions = []string{"are", "is", "in", "your", "my"}
	output = "The Vitamins are in my Fresh California Raisins"
	c.Assert(titleCaseWithExceptions(phrase, exceptions), Equals, output)
}
