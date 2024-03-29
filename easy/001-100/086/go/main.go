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
	"strings"
)

type RunLengthTuple struct {
	Count  int
	Letter string
}

func (r *RunLengthTuple) String() string {
	return strings.Repeat(r.Letter, r.Count)
}

type RunLengthEncoding []RunLengthTuple

func (r *RunLengthEncoding) String() string {
	output := ""
	for _, tuple := range *r {
		output += tuple.String()
	}
	return output
}

func NewRunLengthEncoding(input string) (encoding RunLengthEncoding) {
	currentTuple := RunLengthTuple{1, string(input[0])}
	for index := 1; index < len(input); index++ {
		if currentTuple.Letter == string(input[index]) {
			currentTuple.Count++
		} else {
			encoding = append(encoding, currentTuple)
			currentTuple = RunLengthTuple{1, string(input[index])}
		}
	}
	encoding = append(encoding, currentTuple)
	return
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
