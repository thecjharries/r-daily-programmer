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

func (s *MainSuite) TestGenerateOperatorPossibilities(c *C) {
	c.Assert(generateOperatorPossibilities(2), DeepEquals, [][]string{[]string{"+", "+"}, []string{"+", "-"}, []string{"+", "*"}, []string{"+", "-"}, []string{"-", "+"}, []string{"-", "-"}, []string{"-", "*"}, []string{"-", "-"}, []string{"*", "+"}, []string{"*", "-"}, []string{"*", "*"}, []string{"*", "-"}, []string{"-", "+"}, []string{"-", "-"}, []string{"-", "*"}, []string{"-", "-"}})
}

func (s *MainSuite) TestCountdown(c *C) {
	c.Assert(countdown([]int{1, 3, 7, 6, 8, 3, 250}), Equals, "3 + 3 * 7 + 1 * 6 - 8 = 250")
	c.Assert(countdown([]int{25, 100, 9, 7, 3, 7, 881}), Equals, "25 - 9 * 7 * 7 + 100 - 3 = 881")
	// Took too long to generate this one
	//c.Assert(countdown([]int{6, 75, 3, 25, 50, 100, 952}), Equals, "100 + 6 * 3 * 75 - 50 / 25 = 952")
}
