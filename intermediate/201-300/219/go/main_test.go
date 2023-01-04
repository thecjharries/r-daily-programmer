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

func (s *MainSuite) TestTodoListAddItem(c *C) {
	todo := new(TodoList)
	c.Assert(len(*todo), Equals, 0)
	todo.AddItem("Take a shower")
	c.Assert(len(*todo), Equals, 1)
}

func (s *MainSuite) TestTodoListDeleteItem(c *C) {
	todo := new(TodoList)
	c.Assert(len(*todo), Equals, 0)
	todo.
		AddItem("Take a shower").
		AddItem("Go to work").
		AddItem("Buy a new phone").
		DeleteItem("Go to work")
	c.Assert(len(*todo), Equals, 2)
}

func (s *MainSuite) TestTodoListViewList(c *C) {
	todo := new(TodoList)
	c.Assert(len(*todo), Equals, 0)
	todo.
		AddItem("Take a shower").
		AddItem("Go to work").
		AddItem("Buy a new phone").
		DeleteItem("Go to work")
	c.Assert(len(*todo), Equals, 2)
	c.Assert(todo.ViewList(), Equals, "Take a shower\nBuy a new phone")
}

func (s *MainSuite) TestTodoListUpdateItem(c *C) {
	todo := new(TodoList)
	c.Assert(len(*todo), Equals, 0)
	todo.AddItem("Take a shower")
	c.Assert((*todo)[0], Equals, "Take a shower")
	todo.UpdateItem("Take a shower", "Take a bath")
	c.Assert((*todo)[0], Equals, "Take a bath")
}
