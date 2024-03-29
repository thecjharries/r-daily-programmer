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

type MainSuite struct{}

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestSplitEquation(c *C) {
	input := "L0(x,y)=abs(x)+abs(y)"
	outputLhs := "L0(x,y)"
	outputRhs := "abs(x)+abs(y)"
	foundLhs, foundRhs := splitEquation(input)
	c.Assert(foundLhs, Equals, outputLhs)
	c.Assert(foundRhs, Equals, outputRhs)
}

func (s *MainSuite) TestSanitizeRhs(c *C) {
	input := "abs(x)+abs(y)"
	output := "fabsf(x)+fabsf(y)"
	c.Assert(sanitizeRhs(input), Equals, output)
}

func (s *MainSuite) TestSanitizeLhs(c *C) {
	input := "L0(x,y)"
	output := "L0(float x, float y)"
	c.Assert(sanitizeLhs(input), Equals, output)
}

func (s *MainSuite) TestProcessEquation(c *C) {
	input := "L0(x,y)=abs(x)+abs(y)"
	output := `float L0(float x, float y)
{
	return fabsf(x)+fabsf(y);
}`
	c.Assert(processEquation(input), Equals, output)
}
