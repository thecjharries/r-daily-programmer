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

func (s *MainSuite) TestEncodeLetter(c *C) {
	c.Assert(encodeLetter('s', 't'), Equals, 'l')
	c.Assert(encodeLetter('n', 'h'), Equals, 'u')
	c.Assert(encodeLetter('i', 'e'), Equals, 'm')
}

func (s *MainSuite) TestDecodeLetter(c *C) {
	c.Assert(decodeLetter('c', 'k'), Equals, 'i')
	c.Assert(decodeLetter('l', 'l'), Equals, 'a')
	c.Assert(decodeLetter('o', 'a'), Equals, 'm')
}

func (s *MainSuite) TestEncode(c *C) {
	c.Assert(encode("snitch", "thepackagehasbeendelivered"), Equals, "lumicjcnoxjhkomxpkwyqogywq")
	c.Assert(encode("bond", "theredfoxtrotsquietlyatmidnight"), Equals, "uvrufrsryherugdxjsgozogpjralhvg")
}

func (s *MainSuite) TestDecode(c *C) {
	c.Assert(decode("cloak", "klatrgafedvtssdwywcyty"), Equals, "iamtheprettiestunicorn")
	c.Assert(decode("python", "pjphmfamhrcaifxifvvfmzwqtmyswst"), Equals, "alwayslookonthebrightsideoflife")
}
