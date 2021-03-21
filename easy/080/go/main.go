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
	"io/ioutil"
	"path"
	"strings"
)

var dictionaryPath string = path.Join("..", "enable1.txt")

type AnagramDictionary map[string][]string

func NewDictionary(filename string) *AnagramDictionary {
	byteContents, _ := ioutil.ReadFile(filename)
	stringContents := strings.Split(
		strings.Trim(string(byteContents), "\n"),
		"\n",
	)
	dictionary := make(AnagramDictionary, len(stringContents))
	for _, word := range stringContents {
		dictionary[word] = []string(nil)
	}
	return &dictionary
}

var zPrint = fmt.Printf

func main() {
	_, _ = zPrint("hello world")
}
