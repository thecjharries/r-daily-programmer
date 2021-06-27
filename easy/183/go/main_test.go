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

func (s *MainSuite) TestNewSemVer(c *C) {
	input := "2.0.11-alpha"
	output := SemVer{
		Major: 2,
		Minor: 0,
		Patch: 11,
		Label: "alpha",
	}
	c.Assert(NewSemVer(input), DeepEquals, output)
}

func (s *MainSuite) TestSemVersLen(c *C) {
	semvers := SemVers{
		NewSemVer("2.0.11-alpha"),
	}
	c.Assert(semvers.Len(), Equals, 1)
}

func (s *MainSuite) TestSemVersSwap(c *C) {
	semvers := SemVers{
		NewSemVer("2.0.11-alpha"),
		NewSemVer("0.1.7+amd64"),
	}
	c.Assert(semvers[0].Major, Equals, 2)
	semvers.Swap(0, 1)
	c.Assert(semvers[0].Major, Equals, 0)
}

func (s *MainSuite) TestSemVersLess(c *C) {
	semvers := SemVers{
		NewSemVer("2.0.11-alpha"),
		NewSemVer("0.1.7+amd64"),
		NewSemVer("2.0.11+i386"),
		NewSemVer("2.1.11+i386"),
		NewSemVer("2.0.12+i386"),
	}
	c.Assert(semvers.Less(0, 1), Equals, false)
	c.Assert(semvers.Less(1, 0), Equals, true)
	c.Assert(semvers.Less(0, 2), Equals, true)
	c.Assert(semvers.Less(2, 0), Equals, false)
	c.Assert(semvers.Less(0, 3), Equals, true)
	c.Assert(semvers.Less(3, 0), Equals, false)
	c.Assert(semvers.Less(0, 4), Equals, true)
	c.Assert(semvers.Less(4, 0), Equals, false)
}

func (s *MainSuite) TestSemVersSort(c *C) {
	input := SemVers{
		NewSemVer("2.0.11-alpha"),
		NewSemVer("0.1.7+amd64"),
		NewSemVer("0.10.7+20141005"),
		NewSemVer("2.0.12+i386"),
		NewSemVer("1.2.34"),
		NewSemVer("2.0.11+i386"),
		NewSemVer("20.1.1+i386"),
	}
	output := SemVers{
		NewSemVer("0.1.7+amd64"),
		NewSemVer("0.10.7+20141005"),
		NewSemVer("1.2.34"),
		NewSemVer("2.0.11-alpha"),
		NewSemVer("2.0.11+i386"),
		NewSemVer("2.0.12+i386"),
		NewSemVer("20.1.1+i386"),
	}
	c.Assert(input, Not(DeepEquals), output)
	sort.Sort(input)
	c.Assert(input, DeepEquals, output)
}
