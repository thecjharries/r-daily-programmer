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

func (s *MainSuite) TestHexdumpFile(c *C) {
	output := `00000000  74 65 73 74 3a 0a 09 67  6f 20 74 65 73 74 20 2d  |test:..go test -|
00000010  76 20 2e 2f 2e 2e 2e 20  2d 63 6f 76 65 72 20 2d  |v ./... -cover -|
00000020  72 61 63 65 20 2d 63 6f  76 65 72 70 72 6f 66 69  |race -coverprofi|
00000030  6c 65 3d 2e 63 6f 76 65  72 61 67 65 2e 6f 75 74  |le=.coverage.out|
00000040  0a 0a 63 6f 76 65 72 61  67 65 3a 20 74 65 73 74  |..coverage: test|
00000050  0a 09 67 6f 20 74 6f 6f  6c 20 63 6f 76 65 72 20  |..go tool cover |
00000060  2d 66 75 6e 63 3d 2e 63  6f 76 65 72 61 67 65 2e  |-func=.coverage.|
00000070  6f 75 74 0a 0a 72 75 6e  3a 0a 09 67 6f 20 72 75  |out..run:..go ru|
00000080  6e 20 6d 61 69 6e 2e 67  6f 0a 0a 62 6f 6f 74 73  |n main.go..boots|
00000090  74 72 61 70 2d 66 65 61  74 75 72 65 2d 62 72 61  |trap-feature-bra|
000000a0  6e 63 68 3a 0a 09 67 69  74 20 66 6c 6f 77 20 66  |nch:..git flow f|
000000b0  65 61 74 75 72 65 20 73  74 61 72 74 20 24 28 6c  |eature start $(l|
000000c0  61 73 74 77 6f 72 64 20  24 28 73 75 62 73 74 20  |astword $(subst |
000000d0  2f 2c 20 2c 24 28 72 65  61 6c 70 61 74 68 20 2e  |/, ,$(realpath .|
000000e0  2e 2f 2e 2e 2f 29 29 29  2d 24 28 6c 61 73 74 77  |./../)))-$(lastw|
000000f0  6f 72 64 20 24 28 73 75  62 73 74 20 2f 2c 20 2c  |ord $(subst /, ,|
00000100  24 28 72 65 61 6c 70 61  74 68 20 2e 2e 2f 29 29  |$(realpath ../))|
00000110  29 0a 09 67 69 74 20 61  64 64 20 2e 2e 2f 52 45  |)..git add ../RE|
00000120  41 44 4d 45 2e 6d 64 0a  09 67 69 74 20 63 6f 6d  |ADME.md..git com|
00000130  6d 69 74 20 2d 6d 20 27  43 6f 70 79 20 70 72 6f  |mit -m 'Copy pro|
00000140  6d 70 74 27 0a 09 67 69  74 20 61 64 64 20 2e 0a  |mpt'..git add ..|
00000150  09 67 69 74 20 63 6f 6d  6d 69 74 20 2d 6d 20 27  |.git commit -m '|
00000160  43 6f 70 79 20 47 6f 20  62 6f 69 6c 65 72 70 6c  |Copy Go boilerpl|
00000170  61 74 65 27 0a 0a 67 69  74 2d 70 72 6f 6d 70 74  |ate'..git-prompt|
00000180  2d 72 75 6e 6e 65 72 3a  0a 09 67 69 74 20 61 64  |-runner:..git ad|
00000190  64 20 6d 61 69 6e 2e 67  6f 20 6d 61 69 6e 5f 74  |d main.go main_t|
000001a0  65 73 74 2e 67 6f 0a 09  67 69 74 20 63 6f 6d 6d  |est.go..git comm|
000001b0  69 74 20 2d 6d 20 27 41  64 64 20 70 72 6f 6d 70  |it -m 'Add promp|
000001c0  74 20 72 75 6e 6e 65 72  20 74 6f 20 6d 61 69 6e  |t runner to main|
000001d0  27 0a 0a 66 69 6e 69 73  68 3a 0a 09 67 69 74 20  |'..finish:..git |
000001e0  66 6c 6f 77 20 66 65 61  74 75 72 65 20 66 69 6e  |flow feature fin|
000001f0  69 73 68 0a                                       |ish.|
`
	c.Assert(hexdumpFile("Makefile"), Equals, output)
}
