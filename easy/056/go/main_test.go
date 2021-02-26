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
	"io/ioutil"
	"os"
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct {}

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestDeleteFile(c *C) {
	filename := "test"
	fileHandle, _ := os.Create(filename)
	_ = fileHandle.Close()
	_, existsError := os.Stat(filename)
	c.Assert(existsError, IsNil)
	deleteFile(filename)
	_, notExistsError := os.Stat(filename)
	c.Assert(os.IsNotExist(notExistsError), Equals, true)
}

func (s *MainSuite) TestWriteAbacabaSequenceToFile(c *C) {
	output := `a
aba
abacaba
`
	alphabet := "abc"
	filename := "scratch.txt"
	writeAbacabaSequenceToFile(alphabet, filename)
	fileContents, _ := ioutil.ReadFile(filename)
	c.Assert(fileContents, DeepEquals, []byte(output))
}
