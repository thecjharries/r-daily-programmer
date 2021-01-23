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

func (s *MainSuite) TestIsPrime(c *C) {
	c.Assert(isPrime(3, knownPrimesSeed), Equals, true)
	c.Assert(isPrime(4, knownPrimesSeed), Equals, false)
	c.Assert(isPrime(5, append(knownPrimesSeed, 3)), Equals, true)
	c.Assert(isPrime(6, knownPrimesSeed), Equals, false)
	c.Assert(isPrime(7, append(knownPrimesSeed, 3, 5)), Equals, true)
	c.Assert(isPrime(8, knownPrimesSeed), Equals, false)
	c.Assert(isPrime(9, append(knownPrimesSeed, 3, 5, 7)), Equals, false)
}

func (s *MainSuite) TestFindPrimesBelow(c *C) {
	primesUnderTen := []int{2, 3, 5, 7}
	c.Assert(findPrimesBelow(10), DeepEquals, primesUnderTen)
}
