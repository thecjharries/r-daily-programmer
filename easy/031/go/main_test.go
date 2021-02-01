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

type baseChangeFixture  struct{
	base10 int
	base26 string
}

var base10ToBase26Fixtures = []baseChangeFixture{
	{0, "A"},
	{26, "BA"},
}

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestBase10ToBase26(c *C) {
	for _, fixture := range base10ToBase26Fixtures{
		c.Assert(base10ToBase26(fixture.base10), Equals, fixture.base26)
	}
}

func (s *MainSuite) TestBase26ToBase10(c *C) {
	for _, fixture := range base10ToBase26Fixtures{
		c.Assert(base26ToBase10(fixture.base26), Equals, fixture.base10)
	}
}
