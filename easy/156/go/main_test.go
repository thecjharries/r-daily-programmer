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
	c.Assert(printSpyContents, Equals, "April Fools!!!! Way to go! You have solved the hidden message. Now it is your turn to join in on this prank. Do not post anything that will give away this message. Let people decode the message on their own and read this message. You can play along by NOT posting how you decoded this message. Instead post your solution to be your favorite Hello world program in one language of your choice.  Make sure your program says \"Hello World!!!\" with 3 ! at the end. This way people browsing the challenge will think we have all lost our minds. Those who post hello world solutions without the three !!! will have not decoded the message and so you can  politely point out their solution is in error (they are just following along without knowing) Enjoy this fun. The truth will be held by those who can decode the message. :)")
}

func (s *MainSuite) TestDecodePrompt(c *C) {
	c.Assert(decodePrompt("Etvmp$Jsspw%%%%"), Equals, "April Fools!!!!")
}
