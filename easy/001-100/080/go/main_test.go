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

func printSpy(format string, a ...interface{}) (n int, err error) {
	printSpyContents = fmt.Sprintf(format, a...)
	printCallCount++
	return
}

func (s *MainSuite) SetUpTest(c *C) {
	printCallCount = 0
	printSpyContents = ""
	zPrint = printSpy
}

func (s *MainSuite) TearDownTest(c *C) {
	zPrint = fmt.Printf
}

func (s *MainSuite) TestMain(c *C) {
	c.Assert(printCallCount, Equals, 0)
	c.Assert(printSpyContents, Equals, "")
	main()
	c.Assert(printCallCount, Equals, 1)
	c.Assert(printSpyContents, Equals, "[alerting altering integral relating tanglier triangle]")
}

func (s *MainSuite) TestAnagramDictionaryToAnagramKey(c *C) {
	dictionary := AnagramDictionary{}
	c.Assert(dictionary.ToAnagramKey("WOrd"), Equals, "dorw")
}

func (s *MainSuite) TestNewAnagramDictionary(c *C) {
	dictionary := NewAnagramDictionary("test_dictionary.txt")
	c.Assert(len(*dictionary), Equals, 1)
	c.Assert(len((*dictionary)["eloprsu"]), Equals, 3)
}

func (s *MainSuite) TestAnagramDictionaryAnagrams(c *C) {
	dictionary := NewAnagramDictionary("test_dictionary.txt")
	c.Assert(len(dictionary.Anagrams("leprous")), Equals, 3)
	c.Assert(len(dictionary.Anagrams("word")), Equals, 1)
}
