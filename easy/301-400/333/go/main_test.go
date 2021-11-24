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

func (s *MainSuite) TestAssemblePackets(c *C) {
	c.Assert(assemblePackets([]string{
		"6220    1   10  Because he's the hero Gotham deserves, ",
		"6220    9   10",
		"5181    5   7   in time, like tears in rain. Time to die.",
		"6220    3   10  So we'll hunt him. ",
		"6220    5   10  Because he's not a hero. ",
		"5181    6   7",
		"5181    2   7   shoulder of Orion. I watched C-beams",
		"5181    4   7   Gate. All those moments will be lost",
		"6220    6   10  He's a silent guardian. ",
		"5181    3   7   glitter in the dark near the Tannhäuser",
		"6220    7   10  A watchful protector.",
		"5181    1   7   believe. Attack ships on fire off the",
		"6220    0   10  We have to chase him.",
		"5181    0   7   I've seen things you people wouldn't",
		"6220    4   10  Because he can take it.",
		"6220    2   10  but not the one it needs right now.",
		"6220    8   10  A Dark Knight.",
	}), DeepEquals, map[int]string{5181: "I've seen things you people wouldn'tbelieve. Attack ships on fire off theshoulder of Orion. I watched C-beamsglitter in the dark near the TannhäuserGate. All those moments will be lostin time, like tears in rain. Time to die.", 6220: "We have to chase him.Because he's the hero Gotham deserves, but not the one it needs right now.So we'll hunt him. Because he can take it.Because he's not a hero. He's a silent guardian. A watchful protector.A Dark Knight."})
}
