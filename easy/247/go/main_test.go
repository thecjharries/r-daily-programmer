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
	"sort"
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
	c.Assert(printSpyContents, Equals, "hello world")
}

func (s *MainSuite) TestSecretSantaListSort(c *C) {
	unsorted := SecretSantaList{{"Joe"}, {"Jeff", "Jerry"}, {"Johnson"}}
	sorted := SecretSantaList{{"Jeff", "Jerry"}, {"Joe"}, {"Johnson"}}
	c.Assert(unsorted, Not(DeepEquals), sorted)
	sort.Sort(unsorted)
	c.Assert(unsorted, DeepEquals, sorted)
}

func (s *MainSuite) TestAssignSecretSantas(c *C) {
	var results []string
	var list SecretSantaList
	list = SecretSantaList{{"Jeff", "Jerry"}, {"Joe"}, {"Johnson"}}
	results = []string{"Jeff -> Johnson", "Jerry -> Joe"}
	c.Assert(assignSecretSantas(list), DeepEquals, results)
	list = SecretSantaList{{"Jeff", "Jerry"}, {"Joe"}, {"Johnson"}, {"Bruno", "Anna", "Matthew", "Lucas"}}
	results = []string{"Lucas -> Jerry", "Matthew -> Joe", "Bruno -> Jeff", "Anna -> Johnson"}
	c.Assert(assignSecretSantas(list), DeepEquals, results)
	list = SecretSantaList{{"Jeff", "Jerry"}, {"Joe"}, {"Johnson"}, {"Bruno", "Anna", "Matthew", "Lucas", "Bob"}}
	c.Assert(assignSecretSantas(list), DeepEquals, []string(nil))
}
