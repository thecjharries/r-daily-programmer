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

func (s *MainSuite) TestTransposeText(c *C) {
	c.Assert(transposeText("Some\ntext."), Equals, "St\noe\nmx\net\n .")
	c.Assert(transposeText("package main\n\nimport \"fmt\"\n\nfunc main() {\n    queue := make(chan string, 2)\n    queue <- \"one\"\n    queue <- \"twoO\"\n    close(queue)\n    for elem := range queue {\n        fmt.Println(elem)\n    }\n}"), Equals, "p i f       }\na m u\nc p n\nk o c\na r  qqqcf }\ng t muuulo\ne   aeeeor\n  \" iuuus\nm f neeeeef\na m (   (lm\ni t ):<<qet\nn \"  =--um.\n    {   e P\n     m\"\"u:r\n     aote=i\n     knw) n\n     eeo rt\n     (\"O al\n     c \" nn\n     h   g(\n     a   ee\n     n    l\n         qe\n     s   um\n     t   e)\n     r   u\n     i   e\n     n\n     g   {\n     ,\n\n     2\n     )")
}
