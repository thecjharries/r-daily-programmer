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
	"io"
	"strings"
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct{}

var _ = Suite(&MainSuite{})
var oldInput io.Reader

func (s *MainSuite) SetUpTest(c *C) {
	oldInput = zInput
}

func (s *MainSuite) TearDownTest(c *C) {
	zInput = oldInput
}

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestQuestionAnswerEvaluate(c *C) {
	qa := QuestionAnswer{"one", "two"}
	zInput = strings.NewReader("exit")
	c.Assert(qa.Evaluate(), Equals, "exiting")
	zInput = strings.NewReader(qa.Answer)
	c.Assert(qa.Evaluate(), Equals, "correct")
	zInput = strings.NewReader("qqq")
	c.Assert(qa.Evaluate(), Equals, qa.Answer)
}

func (s *MainSuite) TestQuizRandom(c *C) {
	qa := QuestionAnswer{"one", "two"}
	quiz := Quiz{[]QuestionAnswer{qa}}
	c.Assert(quiz.Random(), DeepEquals, qa)
}
