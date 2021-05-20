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

func (s *MainSuite) TestParticleSimplifiedColombsLaw(c *C) {
	var particle1, particle2 Particle
	particle1 = Particle{
		Mass: 1,
		X:    -5.2,
		Y:    3.8,
	}
	particle2 = Particle{
		Mass: 1,
		X:    8.7,
		Y:    -4.1,
	}
	c.Assert(particle1.SimplifiedColombsLaw(particle2), Equals, particle2.SimplifiedColombsLaw(particle1))
	c.Assert(particle1.SimplifiedColombsLaw(particle2), Equals, 0.0039)
	particle1 = Particle{
		Mass: 4,
		X:    0.04,
		Y:    -0.02,
	}
	particle2 = Particle{
		Mass: 4,
		X:    -0.02,
		Y:    -0.03,
	}
	c.Assert(particle1.SimplifiedColombsLaw(particle2), Equals, particle2.SimplifiedColombsLaw(particle1))
	c.Assert(particle1.SimplifiedColombsLaw(particle2), Equals, 4324.3243)
}
