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

func (s *MainSuite) TestLoadDictionary(c *C) {
	c.Assert(loadDictionary("test_dictionary.txt"), DeepEquals, []string{"bee", "cab", "ghost"})
}

func (s *MainSuite) TestConvertDictionaryToSortedLetterMap(c *C) {
	dictionary := []string{"dog", "god", "bee"}
	output := map[string][]string{
		"dgo": {"dog", "god"},
		"bee": {"bee"},
	}
	c.Assert(convertDictionaryToSortedLetterMap(dictionary), DeepEquals, output)
}

func (s *MainSuite) TestFindPossibleUnscrambling(c *C) {
	wordList := map[string][]string{
		"dgo": {"dog", "god"},
		"bee": {"bee"},
	}
	c.Assert(findPossibleUnscrambling("ogd", wordList), DeepEquals, []string{"dog", "god"})
	c.Assert(findPossibleUnscrambling("nope", wordList), DeepEquals, []string(nil))
}

func (s *MainSuite) TestSortWord(c *C) {
	c.Assert(sortWord("god"), Equals, "dgo")
	c.Assert(sortWord("bee"), Equals, "bee")
}
