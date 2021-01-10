// Copyright 2020 CJ Harries
// Licensed under http://www.apache.org/licenses/LICENSE-2.0

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
