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

func (s *MainSuite) TestConvertFromBaseFibonacciToBase10(c *C) {
	c.Assert(convertFromBaseFibonacciToBase10("1"), Equals, 1)
	c.Assert(convertFromBaseFibonacciToBase10("10"), Equals, 1)
	c.Assert(convertFromBaseFibonacciToBase10("111111"), Equals, 20)
	c.Assert(convertFromBaseFibonacciToBase10("100000"), Equals, 8)
	c.Assert(convertFromBaseFibonacciToBase10("10110110100111001"), Equals, 2868)
}

func (s *MainSuite) TestConvertFromBase10ToBaseFibonacci(c *C) {
	c.Assert(convertFromBase10ToBaseFibonacci(16), Equals, "1001000")
	c.Assert(convertFromBase10ToBaseFibonacci(32), Equals, "10101000")
	c.Assert(convertFromBase10ToBaseFibonacci(9024720), Equals, "1010100101010100000010001000010010")
	c.Assert(convertFromBase10ToBaseFibonacci(1), Equals, "10")
}
