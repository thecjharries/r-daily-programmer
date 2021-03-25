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
	"strings"
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct {
	getInputReader    *strings.Reader
	buildOutputReader *strings.Reader
}

const getInputInput string = "test\n"
const getInputOutput string = "test"
const buildResultInput string = "qqq\n"
const buildResultOutput string = "your name is qqq, you are  years old, and your username is "

var _ = Suite(&MainSuite{
	getInputReader:    strings.NewReader(getInputInput),
	buildOutputReader: strings.NewReader(buildResultInput),
})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestGetInput(c *C) {
	input := getInput("", s.getInputReader)
	c.Assert(input, Equals, getInputOutput)
}

func (s *MainSuite) TestBuildPrompt(c *C) {
	result := buildResult(s.buildOutputReader)
	c.Assert(result, Equals, buildResultOutput)
}
