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

func (s *MainSuite) TestConvertNumberToBase(c *C) {
	c.Assert(convertNumberToBase(10, base36Digits), Equals, "A")
	c.Assert(convertNumberToBase(100, base36Digits), Equals, "2S")
	c.Assert(convertNumberToBase(1000, base36Digits), Equals, "RS")
	c.Assert(convertNumberToBase(10000, base36Digits), Equals, "7PS")
	c.Assert(convertNumberToBase(100000, base36Digits), Equals, "255S")
}

func (s *MainSuite) TestIsNumberPalindromicInBase(c *C) {
	c.Assert(isNumberPalindromicInBase(1, base36Digits), Equals, true)
	c.Assert(isNumberPalindromicInBase(100, base36Digits), Equals, false)
	c.Assert(isNumberPalindromicInBase(16708, base27Digits), Equals, true)
	c.Assert(isNumberPalindromicInBase(15167, base27Digits), Equals, true)
	c.Assert(isNumberPalindromicInBase(15167, base36Digits), Equals, true)
}
