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
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct {}

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestGetLetterValue(c *C) {
	c.Assert(getLetterValue("a"), Equals, 1)
	c.Assert(getLetterValue("z"), Equals, 26)
}

func (s *MainSuite) TestGetWordValue(c *C) {
	c.Assert(getWordValue("Hat"), Equals, 29)
	c.Assert(getWordValue("Shoe"), Equals, 47)
}

func (s *MainSuite) TestCreateWordValueSlice(c *C) {
	input := []string{"Hat", "Shoe"}
	output := []WordValue{{"Hat", 29}, {"Shoe", 47}}
	c.Assert(createWordValueSlice(input), DeepEquals, output)
}

func (s *MainSuite) TestSortWordValueSlice(c *C) {
	input := []WordValue{{"Shoe", 47}, {"Hat", 29}}
	output := []WordValue{{"Hat", 29}, {"Shoe", 47}}
	c.Assert(sortWordValueSlice(input), DeepEquals, output)
}
