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
	c.Assert(printSpyContents, Equals, "[abbccc abcbcc abccbc abcccb acbbcc acbcbc acbccb accbbc accbcb acccbb babccc bacbcc baccbc bacccb bbaccc bbcacc bbccac bbccca bcabcc bcacbc bcaccb bcbacc bcbcac bcbcca bccabc bccacb bccbac bccbca bcccab bcccba cabbcc cabcbc cabccb cacbbc cacbcb caccbb cbabcc cbacbc cbaccb cbbacc cbbcac cbbcca cbcabc cbcacb cbcbac cbcbca cbccab cbccba ccabbc ccabcb ccacbb ccbabc ccbacb ccbbac ccbbca ccbcab ccbcba cccabb cccbab cccbba]")
}

func (s *MainSuite) TestGetAllStringPermutations(c *C) {
	var input string
	var output []string
	input = "baz"
	output = []string{"baz", "bza", "abz", "azb", "zab", "zba"}
	c.Assert(getAllStringPermutations(input, 0), DeepEquals, output)
}

func (s *MainSuite) TestGetAllUniqueStringPermutations(c *C) {
	var input string
	var output []string
	input = "foo"
	output = []string{"foo", "ofo", "oof"}
	c.Assert(getAllUniqueStringPermutations(input), DeepEquals, output)
}
