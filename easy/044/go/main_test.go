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

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestSanitizeNewLines(c *C) {
	input := "qqq\nqqq"
	output := "qqq qqq"
	c.Assert(sanitizeNewLines(input), Equals, output)
}

func (s *MainSuite) TestExplodeStringOnSentenceStopPattern(c *C) {
	input := "qqq.qqq"
	output := []string{"qqq", "qqq"}
	c.Assert(explodeStringOnSentenceStopPattern(input), DeepEquals, output)
}

func (s *MainSuite) TestFindLargestSentence(c *C) {
	input := []string{"qqq", "qqqq"}
	output := "qqqq"
	c.Assert(findLargestSentence(input), Equals, output)
}

func (s *MainSuite) TestCompletePrompt(c *C) {
	input := "qqq\nqqq. qqq"
	outputSentence, outputLength := "qqq qqq", 7
	foundSentence, foundLength := completePrompt(input)
	c.Assert(foundSentence, Equals, outputSentence)
	c.Assert(foundLength, Equals, outputLength)
}
