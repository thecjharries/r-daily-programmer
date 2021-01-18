// Copyright 2020 CJ Harries
// Licensed under http://www.apache.org/licenses/LICENSE-2.0

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

func (s *MainSuite) TestFormatPhoneNumber(c *C) {
	c.Assert(formatPhoneNumber("18002662278"), Equals, "1-800-266-2278")
	c.Assert(formatPhoneNumber("128002662278"), Equals, "12-800-266-2278")
}

func (s *MainSuite) TestConvertPhoneNumberToPureNumber(c *C) {
	c.Assert(convertPhoneNumberToPureNumber("1-800-COMCAST"), Equals, "18002662278")
}
