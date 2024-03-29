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
	"fmt"
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct{}

var _ = Suite(&MainSuite{})

var printCallCount int
var printSpyContents string

func printSpy(a ...interface{}) (n int, err error) {
	printSpyContents = fmt.Sprint(a...)
	printCallCount++
	return
}

func (s *MainSuite) SetUpTest(c *C) {
	printCallCount = 0
	printSpyContents = ""
	zPrint = printSpy
}

func (s *MainSuite) TearDownTest(c *C) {
	zPrint = fmt.Println
}

func (s *MainSuite) TestMain(c *C) {
	c.Assert(printCallCount, Equals, 0)
	c.Assert(printSpyContents, Equals, "")
	main()
	c.Assert(printCallCount, Equals, 1)
	c.Assert(printSpyContents, Equals, "hello world")
}

func (s *MainSuite) TestIsPrime(c *C) {
	c.Assert(isPrime(1), Equals, false)
	c.Assert(isPrime(2), Equals, true)
	c.Assert(isPrime(3), Equals, true)
	c.Assert(isPrime(4), Equals, false)
	c.Assert(isPrime(5), Equals, true)
	c.Assert(isPrime(6), Equals, false)
	c.Assert(isPrime(7), Equals, true)
	c.Assert(isPrime(8), Equals, false)
	c.Assert(isPrime(9), Equals, false)
	c.Assert(isPrime(10), Equals, false)
	c.Assert(isPrime(15), Equals, false)
}

func (s *MainSuite) TestFindPrimeRange(c *C) {
	c.Assert(findPrimeRange(270), DeepEquals, []int{269, 271})
	c.Assert(findPrimeRange(541), DeepEquals, []int(nil))
	c.Assert(findPrimeRange(993), DeepEquals, []int{991, 997})
	c.Assert(findPrimeRange(649), DeepEquals, []int{647, 653})
}
