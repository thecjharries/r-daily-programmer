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

func (s *MainSuite) TestFindAllWords(c *C) {
	words := findAllWords("The lazy cat slept in the sunlight.")
	c.Assert(len(words), Equals, 7)
}

func (s *MainSuite) TestGetWordAtIndex(c *C) {
	words := findAllWords("...You...!!!@!3124131212 Hello have this is a --- string Solved !!...? to test @\n\n\n#!#@#@%$**#$@ Congratz this!!!!!!!!!!!!!!!!one ---Problem\n\n")
	c.Assert(getWordAtIndex(12, words), Equals, "Congratz")
	c.Assert(getWordAtIndex(-1, words), Equals, "")
	c.Assert(getWordAtIndex(1, words), Equals, "You")
	c.Assert(getWordAtIndex(-100, words), Equals, "")
	c.Assert(getWordAtIndex(4, words), Equals, "have")
	c.Assert(getWordAtIndex(1000, words), Equals, "")
	c.Assert(getWordAtIndex(9, words), Equals, "Solved")
	c.Assert(getWordAtIndex(-1000, words), Equals, "")
	c.Assert(getWordAtIndex(16, words), Equals, "")
	c.Assert(getWordAtIndex(13, words), Equals, "this")
	c.Assert(getWordAtIndex(17, words), Equals, "")
	c.Assert(getWordAtIndex(15, words), Equals, "Problem")
}
