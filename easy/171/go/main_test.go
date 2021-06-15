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

func (s *MainSuite) TestConvertHexToAsciiPicture(c *C) {
	var input, output string
	input = "18 3C 7E 7E 18 18 18 18"
	output = "00011000\n00111100\n01111110\n01111110\n00011000\n00011000\n00011000\n00011000\n"
	c.Assert(convertHexToAsciiPicture(input), Equals, output)
	input = "FF 81 BD A5 A5 BD 81 FF"
	output = "11111111\n10000001\n10111101\n10100101\n10100101\n10111101\n10000001\n11111111\n"
	c.Assert(convertHexToAsciiPicture(input), Equals, output)
	input = "AA 55 AA 55 AA 55 AA 55"
	output = "10101010\n01010101\n10101010\n01010101\n10101010\n01010101\n10101010\n01010101\n"
	c.Assert(convertHexToAsciiPicture(input), Equals, output)
	input = "3E 7F FC F8 F8 FC 7F 3E"
	output = "00111110\n01111111\n11111100\n11111000\n11111000\n11111100\n01111111\n00111110\n"
	c.Assert(convertHexToAsciiPicture(input), Equals, output)
	input = "93 93 93 F3 F3 93 93 93"
	output = "10010011\n10010011\n10010011\n11110011\n11110011\n10010011\n10010011\n10010011\n"
	c.Assert(convertHexToAsciiPicture(input), Equals, output)
}
