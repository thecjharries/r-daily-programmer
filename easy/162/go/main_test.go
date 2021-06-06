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

func (s *MainSuite) TestDecompressWord(c *C) {
	var input string
	var dictionary []string
	input = "2! ! R 1^ 3 0 4^ . E"
	dictionary = []string{
		"is",
		"my",
		"hello",
		"name",
		"stan",
	}
	c.Assert(decompress(input, dictionary), Equals, "HELLO!\nMy name is Stan.")
	input = "0^ 1 6 7 8 5 10 2 . R\n0^ 1 6 7 8 3 10 4 . R\n0^ 1 6 7 8 15 16 17 . R\n0^ 1 6 7 8 11 . R\n0^ 1 6 7 12 13 14 9 . R\n0^ 1 6 7 8 , 18^ - 0^ - 19 . R E"
	dictionary = []string{
		"i",
		"do",
		"house",
		"with",
		"mouse",
		"in",
		"not",
		"like",
		"them",
		"ham",
		"a",
		"anywhere",
		"green",
		"eggs",
		"and",
		"here",
		"or",
		"there",
		"sam",
		"am",
	}
	c.Assert(decompress(input, dictionary), Equals, "I do not like them in a house.\nI do not like them with a mouse.\nI do not like them here or there.\nI do not like them anywhere.\nI do not like green eggs and ham.\nI do not like them, Sam-I-am.")
}
