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
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

const resultFormatString string = "your name is %s, you are %s years old, and your username is %s"
var prompts = [3]string{
	"Your name?",
	"Your age?",
	"Your username?",
}


func main() {
	fmt.Println(buildResult(io.Reader(os.Stdin)))
}

func getInput(prompt string, source io.Reader) string {
	reader := bufio.NewReader(source)
	fmt.Println(prompt)
	input, _ := reader.ReadString('\n')
	return strings.Replace(input, "\n", "", -1)
}

func buildResult(source io.Reader) string {
	var results [3]string
	for index, prompt := range prompts {
		results[index] = getInput(prompt, source)
	}
	return fmt.Sprintf(resultFormatString, results[0], results[1], results[2])
}
