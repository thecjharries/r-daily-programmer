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

func (s *MainSuite) TestSanitizeShift(c *C) {
	c.Assert(sanitizeShift(-51), Equals, 1)
	c.Assert(sanitizeShift(-25), Equals, 1)
	c.Assert(sanitizeShift(1), Equals, 1)
	c.Assert(sanitizeShift(27), Equals, 1)
	c.Assert(sanitizeShift(53), Equals, 1)
}

func (s *MainSuite) TestEncryptLetter(c *C) {
	c.Assert(encryptLetter("a", 0), Equals, "a")
	c.Assert(encryptLetter("A", 1), Equals, "b")
	c.Assert(encryptLetter("-", 0), Equals, "-")
}

func (s *MainSuite) TestDecryptLetter(c *C) {
	c.Assert(decryptLetter("a", 0), Equals, "a")
	c.Assert(decryptLetter("A", 1), Equals, "z")
	c.Assert(decryptLetter("-", 0), Equals, "-")
}

func (s *MainSuite) TestEncryptString(c *C) {
	c.Assert(encryptString("abc", 0), Equals, "abc")
	c.Assert(encryptString("ABC", 1), Equals, "bcd")
	c.Assert(encryptString("!$#", 24), Equals, "!$#")
}

func (s *MainSuite) TestDecryptString(c *C) {
	c.Assert(decryptString("abc", 0), Equals, "abc")
	c.Assert(decryptString("ABC", 1), Equals, "zab")
	c.Assert(decryptString("!$#", 24), Equals, "!$#")
}
