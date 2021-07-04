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

func (s *MainSuite) TestNewFlag(c *C) {
	flag := &Flag{
		Short: "a",
		Long:  "all",
	}
	c.Assert(NewFlag("a", "all"), DeepEquals, flag)
}

func (s *MainSuite) TestFlagIsActive(c *C) {
	flag := &Flag{
		Short: "a",
		Long:  "all",
	}
	c.Assert(flag.IsActive("a"), Equals, true)
	c.Assert(flag.IsActive("all"), Equals, true)
	c.Assert(flag.IsActive("qqq"), Equals, false)
}

func (s *MainSuite) TestFlagString(c *C) {
	flag := &Flag{
		Short: "a",
		Long:  "all",
	}
	c.Assert(flag.String(), Equals, "flag: all")
}

func (s *MainSuite) TestParseFlags(c *C) {
	input := "-aN --force 12 --verbose 192.168.0.44"
	availableFlags := []*Flag{
		NewFlag("a", "all"),
		NewFlag("f", "force"),
		NewFlag("n", "networking"),
		NewFlag("N", "numerical-list"),
	}
	output := "flag: all\nflag: numerical-list\nflag: force\nparameter: 12\nflag: verbose\nparameter: 192.168.0.44\n"
	c.Assert(parseFlags(input, availableFlags), Equals, output)
}
