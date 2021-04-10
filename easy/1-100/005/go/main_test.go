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
	getStringInputReader *strings.Reader
	validCredentials     []Credentials
}

const getStringInputInput string = "test\n"
const getStringInputOutput string = "test"
const lenValidCreds int = 1

var _ = Suite(&MainSuite{
	getStringInputReader: strings.NewReader(getStringInputInput),
	validCredentials:     []Credentials{{"sage", "sage"}},
})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestGetStringInput(c *C) {
	input := getStringInput("", s.getStringInputReader)
	c.Assert(input, Equals, getStringInputOutput)
}

func (s *MainSuite) TestLoadValidCredentials(c *C) {
	creds := loadValidCredentials(validCredsPath)
	c.Assert(len(creds), Equals, lenValidCreds)
}

func (s *MainSuite) TestAreTheseCredentialsValid(c *C) {
	c.Assert(areTheseCredentialsValid(Credentials{"bad", "creds"}, s.validCredentials), Equals, false)
	c.Assert(areTheseCredentialsValid(s.validCredentials[0], s.validCredentials), Equals, true)
}
