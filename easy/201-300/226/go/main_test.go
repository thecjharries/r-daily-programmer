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

func (s *MainSuite) TestFractionString(c *C) {
	fraction := Fraction{
		Numerator:   1,
		Denominator: 2,
	}
	c.Assert(fraction.String(), Equals, "1/2")
}

func (s *MainSuite) TestFractionGcd(c *C) {
	fraction := Fraction{}
	c.Assert(fraction.gcd(2, 4), Equals, 2)
	c.Assert(fraction.gcd(4, 2), Equals, 2)
	c.Assert(fraction.gcd(66, 55), Equals, 11)
}

func (s *MainSuite) TestFractionReduce(c *C) {
	fraction := new(Fraction)
	fraction.Numerator = 10
	fraction.Denominator = 50
	c.Assert(fraction.String(), Equals, "10/50")
	fraction.Reduce()
	c.Assert(fraction.String(), Equals, "1/5")
}

func (s *MainSuite) TestFractionAdd(c *C) {
	first, second := new(Fraction), new(Fraction)
	first.Numerator = 1
	first.Denominator = 6
	second.Numerator = 3
	second.Denominator = 10
	c.Assert(first.String(), Equals, "1/6")
	first.Add(second)
	c.Assert(first.String(), Equals, "7/15")
}
