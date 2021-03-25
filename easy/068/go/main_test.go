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

func (s *MainSuite) TestReverseString(c *C) {
	c.Assert(reverseString("hello world"), Equals, "dlrow olleh")
	c.Assert(reverseString("abcde"), Equals, "edcba")
}

func (s *MainSuite) TestReverseInteger(c *C) {
	c.Assert(reverseInteger(100), Equals, 1)
	c.Assert(reverseInteger(13), Equals, 31)
}

func (s *MainSuite) TestIsIntegerPalindrome(c *C) {
	c.Assert(isIntegerPalindrome(12), Equals, false)
	c.Assert(isIntegerPalindrome(121), Equals, true)
}

func (s *MainSuite) TestIsPrime(c *C) {
	c.Assert(isPrime(2), Equals, true)
	c.Assert(isPrime(3), Equals, true)
	c.Assert(isPrime(4), Equals, false)
	c.Assert(isPrime(12), Equals, false)
	c.Assert(isPrime(13), Equals, true)
	c.Assert(isPrime(14), Equals, false)
	c.Assert(isPrime(30), Equals, false)
	c.Assert(isPrime(31), Equals, true)
	c.Assert(isPrime(32), Equals, false)
}

func (s *MainSuite) TestIsEmirp(c *C) {
	c.Assert(isEmirp(13), Equals, true)
	c.Assert(isEmirp(31), Equals, true)
	c.Assert(isEmirp(2), Equals, false)
	c.Assert(isEmirp(11), Equals, false)
}

func (s *MainSuite) TestGetEmirpBelowMaxInclusive(c *C) {
	c.Assert(getEmirpBelowMaxInclusive(10), DeepEquals, []int(nil))
	c.Assert(getEmirpBelowMaxInclusive(107), DeepEquals, []int{13, 17, 31, 37, 71, 73, 79, 97, 107})
}
