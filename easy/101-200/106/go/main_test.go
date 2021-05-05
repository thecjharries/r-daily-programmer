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
	"math/rand"
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
	rand.Seed(0)
}

func (s *MainSuite) TearDownTest(c *C) {
	zPrint = fmt.Println
}

func (s *MainSuite) TestMain(c *C) {
	c.Assert(printCallCount, Equals, 0)
	c.Assert(printSpyContents, Equals, "")
	main()
	c.Assert(printCallCount, Equals, 1)
}

func (s *MainSuite) TestLoadFileIntoString(c *C) {
	c.Assert(loadFileIntoString("sample_novel.txt"), Equals, "the quick brown fox jumped over the lazy dog.")
}

func (s *MainSuite) TestCountPatternsInString(c *C) {
	input := "test test cat."
	output := map[string]int{
		"test": 2,
		"cat":  1,
		".":    1,
	}
	c.Assert(countPatternsInString(input), DeepEquals, output)
}

func (s *MainSuite) TestDetermineTopTenWords(c *C) {
	input := map[string]int{
		"one":   1,
		"two":   2,
		"three": 3,
		"III":   3,
		"3":     3,
		"four":  4,
		"five":  5,
		"6":     6,
		"seven": 7,
		"eight": 8,
		"nine":  9,
		"1-0":   10,
		"ten":   10,
	}
	output := []*WordFrequency{
		{"1-0", 10},
		{"ten", 10},
		{"nine", 9},
		{"eight", 8},
		{"seven", 7},
		{"6", 6},
		{"five", 5},
		{"four", 4},
		{"three", 3},
		{"III", 3},
	}
	c.Assert(determineTopTenWords(input), DeepEquals, output)
}
