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
