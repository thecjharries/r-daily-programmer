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
	"regexp"
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct {
	allowedFormRegexp []regexp.Regexp
}

var _ = Suite(&MainSuite{
	allowedFormRegexp: compilePatterns(allowedFormPatterns),
})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestCompilePatterns(c *C) {
	result := compilePatterns(allowedFormPatterns)
	c.Assert(len(result), Equals, len(allowedFormPatterns))
}

func (s *MainSuite) TestValidateNumber(c *C) {
	c.Assert(
		validateNumber("1234567890", s.allowedFormRegexp),
		Equals,
		true,
	)
	c.Assert(
		validateNumber("123-456-7890", s.allowedFormRegexp),
		Equals,
		true,
	)
	c.Assert(
		validateNumber("123.456.7890", s.allowedFormRegexp),
		Equals,
		true,
	)
	c.Assert(
		validateNumber("(123)456-7890", s.allowedFormRegexp),
		Equals,
		true,
	)
	c.Assert(
		validateNumber("(123) 456-7890", s.allowedFormRegexp),
		Equals,
		true,
	)
	c.Assert(
		validateNumber("456-7890", s.allowedFormRegexp),
		Equals,
		true,
	)
	c.Assert(
		validateNumber("123-45-6789", s.allowedFormRegexp),
		Equals,
		false,
	)
	c.Assert(
		validateNumber("123:4567890", s.allowedFormRegexp),
		Equals,
		false,
	)
	c.Assert(
		validateNumber("123/456-7890", s.allowedFormRegexp),
		Equals,
		false,
	)
}
