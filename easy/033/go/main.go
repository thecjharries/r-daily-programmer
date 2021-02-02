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
	. "github.com/thecjharries/dprgstd/clinput"
	"io"
	"math/rand"
	"os"
	"time"
)

type QuestionAnswer struct {
	Question string
	Answer string
}

type Quiz struct {
	Questions []QuestionAnswer
}

func (q *Quiz) Random() QuestionAnswer {
	return q.Questions[rand.Intn(len(q.Questions))]
}

func (q *Quiz) Run() {
	var output string
	for "exiting" != output {
		question := q.Random()
		output = question.Evaluate()
		fmt.Println(output)
	}
}

var zInput = io.Reader(os.Stdin)
var zSeed = int64(time.Now().UnixNano())

func (q *QuestionAnswer) Evaluate() string {
	result := GetStringInput(q.Question, zInput)
	if "exit" == result {
		return "exiting"
	} else if q.Answer == result {
		return "correct"
	}
	return q.Answer
}

var sampleQuiz = Quiz{
	[]QuestionAnswer{
		{"12 * 12?", "144"},
		{"What is reddit?", "website with cats"},
		{"Translate: hola","hello"},
	},
}


func main() {
	rand.Seed(zSeed)
	sampleQuiz.Run()
}
