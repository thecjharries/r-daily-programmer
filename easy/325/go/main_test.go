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

func (s *MainSuite) TestFindPathThroughMazeFollowingSequence(c *C) {
	c.Assert(findPathThroughMazeFollowingSequence([]string{"O", "G"}, [][]string{
		{"B", "O", "R", "O", "Y"},
		{"O", "R", "B", "G", "R"},
		{"B", "O", "G", "O", "Y"},
		{"Y", "G", "B", "Y", "G"},
		{"R", "O", "R", "B", "R"},
	}), DeepEquals, [][]int{[]int{4, 1}, []int{3, 1}, []int{2, 1}, []int{2, 2}, []int{2, 3}, []int{1, 3}, []int{0, 3}})
	c.Assert(findPathThroughMazeFollowingSequence([]string{"R", "O", "Y", "P", "O"}, [][]string{
		{"R", "R", "B", "R", "R", "R", "B", "P", "Y", "G", "P", "B", "B", "B", "G", "P", "B", "P", "P", "R"},
		{"B", "G", "Y", "P", "R", "P", "Y", "Y", "O", "R", "Y", "P", "P", "Y", "Y", "R", "R", "R", "P", "P"},
		{"B", "P", "G", "R", "O", "P", "Y", "G", "R", "Y", "Y", "G", "P", "O", "R", "Y", "P", "B", "O", "O"},
		{"R", "B", "B", "O", "R", "P", "Y", "O", "O", "Y", "R", "P", "B", "R", "G", "R", "B", "G", "P", "G"},
		{"R", "P", "Y", "G", "G", "G", "P", "Y", "P", "Y", "O", "G", "B", "O", "R", "Y", "P", "B", "Y", "O"},
		{"O", "R", "B", "G", "B", "Y", "B", "P", "G", "R", "P", "Y", "R", "O", "G", "Y", "G", "Y", "R", "P"},
		{"B", "G", "O", "O", "O", "G", "B", "B", "R", "O", "Y", "Y", "Y", "Y", "P", "B", "Y", "Y", "G", "G"},
		{"P", "P", "G", "B", "O", "P", "Y", "G", "B", "R", "O", "G", "B", "G", "R", "O", "Y", "R", "B", "R"},
		{"Y", "Y", "P", "P", "R", "B", "Y", "B", "P", "O", "O", "G", "P", "Y", "R", "P", "P", "Y", "R", "Y"},
		{"P", "O", "O", "B", "B", "B", "G", "O", "Y", "G", "O", "P", "B", "G", "Y", "R", "R", "Y", "R", "B"},
		{"P", "P", "Y", "R", "B", "O", "O", "R", "O", "R", "Y", "B", "G", "B", "G", "O", "O", "P", "B", "Y"},
		{"B", "B", "R", "G", "Y", "G", "P", "Y", "G", "P", "R", "R", "P", "Y", "G", "O", "O", "Y", "R", "R"},
		{"O", "G", "R", "Y", "B", "P", "Y", "O", "P", "B", "R", "Y", "B", "G", "P", "G", "O", "O", "B", "P"},
		{"R", "Y", "G", "P", "G", "G", "O", "R", "Y", "O", "O", "G", "R", "G", "P", "P", "Y", "P", "B", "G"},
		{"P", "Y", "P", "R", "O", "O", "R", "O", "Y", "R", "P", "O", "R", "Y", "P", "Y", "B", "B", "Y", "R"},
		{"O", "Y", "P", "G", "R", "P", "R", "G", "P", "O", "B", "B", "R", "B", "O", "B", "Y", "Y", "B", "P"},
		{"B", "Y", "Y", "P", "O", "Y", "O", "Y", "O", "R", "B", "R", "G", "G", "Y", "G", "R", "G", "Y", "G"},
		{"Y", "B", "Y", "Y", "G", "B", "R", "R", "O", "B", "O", "P", "P", "O", "B", "O", "R", "R", "R", "P"},
		{"P", "O", "O", "O", "P", "Y", "G", "G", "Y", "P", "O", "G", "P", "O", "B", "G", "P", "R", "P", "B"},
		{"R", "B", "B", "R", "R", "R", "R", "B", "B", "B", "Y", "O", "B", "G", "P", "G", "G", "O", "O", "Y"},
	}), DeepEquals, [][]int{[]int{19, 6}, []int{19, 5}, []int{19, 4}, []int{19, 3}, []int{18, 3}, []int{17, 3}, []int{16, 3}, []int{16, 4}, []int{15, 4}, []int{16, 4}, []int{16, 5}, []int{15, 5}, []int{14, 5}, []int{14, 6}, []int{13, 6}, []int{12, 6}, []int{11, 6}, []int{10, 6}, []int{10, 7}, []int{9, 7}, []int{9, 8}, []int{8, 8}, []int{8, 9}, []int{7, 9}, []int{6, 9}, []int{6, 10}, []int{5, 10}, []int{4, 10}, []int{3, 10}, []int{4, 10}, []int{4, 9}, []int{4, 8}, []int{3, 8}, []int{2, 8}, []int{1, 8}, []int{0, 8}})
}
