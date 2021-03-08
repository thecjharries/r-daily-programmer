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

func (s *MainSuite) TestReverseString(c *C) {
	c.Assert(reverseString("hello world"), Equals, "dlrow olleh")
	c.Assert(reverseString("abcde"), Equals, "edcba")
}

func (s *MainSuite) TestZeroPadBinaryString(c *C) {
	c.Assert(zeroPadBinaryString("101", 6), Equals, "000101")
	c.Assert(zeroPadBinaryString("110101", 3), Equals, "101")
}

func (s *MainSuite) TestReverseDecimalWithBinaryBits(c *C) {
	c.Assert(reverseDecimalWithBinaryBits(13, 32), Equals, 2952790016)
}
