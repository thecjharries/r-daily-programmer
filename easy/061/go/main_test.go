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

func (s *MainSuite) TestRotateNumber(c *C) {
	c.Assert(rotateNumber(19), Equals, int64(25))
	c.Assert(rotateNumber(25), Equals, int64(28))
	c.Assert(rotateNumber(28), Equals, int64(14))
	c.Assert(rotateNumber(14), Equals, int64(7))
	c.Assert(rotateNumber(7), Equals, int64(7))
}

func (s *MainSuite) TestGenerateBinaryRotationSequence(c *C) {
	c.Assert(generateBinaryRotationSequence(19), DeepEquals, []int64{19,25,28,14,7})
	c.Assert(generateBinaryRotationSequence(205), DeepEquals, []int64{205,230,115,121,124,62,31})
	c.Assert(generateBinaryRotationSequence(357), DeepEquals, []int64{357,434,217,236,118,59,61,62,31})
}
