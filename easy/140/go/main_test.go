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

func (s *MainSuite) TestSanitizeVariableNameString(c *C) {
	c.Assert(sanitizeVariableNameString("hello world"), Equals, "hello world")
	c.Assert(sanitizeVariableNameString("hello world111"), Equals, "hello world111")
	c.Assert(sanitizeVariableNameString("hello_world"), Equals, "hello_world")
	c.Assert(sanitizeVariableNameString("hello-world"), Equals, "helloworld")
}

func (s *MainSuite) TestWriteInCamelCase(c *C) {
	c.Assert(writeInCamelCase("hello world"), Equals, "helloWorld")
	c.Assert(writeInCamelCase("user id"), Equals, "userId")
	c.Assert(writeInCamelCase("map controller delegate manager"), Equals, "mapControllerDelegateManager")
}

func (s *MainSuite) TestWriteInSnakeCase(c *C) {
	c.Assert(writeInSnakeCase("hello world"), Equals, "hello_world")
	c.Assert(writeInSnakeCase("user id"), Equals, "user_id")
	c.Assert(writeInSnakeCase("map controller delegate manager"), Equals, "map_controller_delegate_manager")
}

func (s *MainSuite) TestWriteInConstantCase(c *C) {
	c.Assert(writeInConstantCase("hello world"), Equals, "HELLO_WORLD")
	c.Assert(writeInConstantCase("user id"), Equals, "USER_ID")
	c.Assert(writeInConstantCase("map controller delegate manager"), Equals, "MAP_CONTROLLER_DELEGATE_MANAGER")
}
