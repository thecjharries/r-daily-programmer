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

func (s *MainSuite) TestDecolumnizeText(c *C) {
	var input, output string
	input = "This is an example piece of text. This is an exam-\nple piece of text. This is an example piece of\ntext. This is an example\npiece of text. This is a +-----------------------+\nsample for a challenge.  |                       |\nLorum ipsum dolor sit a- |       top class       |\nmet and other words. The |        feature        |\nproper word for a layout |                       |\nlike this would be type- +-----------------------+\nsetting, or so I would\nimagine, but for now let's carry on calling it an\nexample piece of text. Hold up - the end of the\n                 paragraph is approaching - notice\n+--------------+ the double line break for a para-\n|              | graph.\n|              |\n|   feature    | And so begins the start of the\n|   bonanza    | second paragraph but as you can\n|              | see it's only marginally better\n|              | than the other one so you've not\n+--------------+ really gained much - sorry. I am\n                 certainly not a budding author\nas you can see from this example input. Perhaps I\nneed to work on my writing skills."
	output = "This is an example piece of text. This is an example piece of text. This is an example piece of text. This is an example piece of text. This is a sample for a challenge. Lorum ipsum dolor sit amet and other words. The proper word for a layout like this would be typesetting, or so I would imagine, but for now let's carry on calling it an example piece of text. Hold up - the end of the paragraph is approaching - notice the double line break for a paragraph.\n\nAnd so begins the start of the second paragraph but as you can see it's only marginally better than the other one so you've not really gained much - sorry. I am certainly not a budding author as you can see from this example input. Perhaps I need to work on my writing skills."
	c.Assert(decolumnizeText(input), Equals, output)
}
