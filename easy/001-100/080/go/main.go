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
	"sort"
	"strings"
)

var dictionaryPath string = path.Join("..", "enable1.txt")

type AnagramDictionary map[string][]string

func (d *AnagramDictionary) ToAnagramKey(word string) string {
	exploded := strings.Split(strings.ToLower(word), "")
	sort.Strings(exploded)
	return strings.Join(exploded, "")
}

func (d *AnagramDictionary) Anagrams(word string) []string {
	anagrams, hasAnagrams := (*d)[d.ToAnagramKey(word)]
	if hasAnagrams {
		return anagrams
	}
	return []string{word}
}

func NewAnagramDictionary(filename string) *AnagramDictionary {
	byteContents, _ := ioutil.ReadFile(filename)
	stringContents := strings.Split(
		strings.Trim(string(byteContents), "\n"),
		"\n",
	)
	dictionary := make(AnagramDictionary, len(stringContents))
	for _, word := range stringContents {
		key := dictionary.ToAnagramKey(word)
		_, keyExists := dictionary[key]
		if keyExists {
			dictionary[key] = append(dictionary[key], word)
		} else {
			dictionary[key] = []string{word}
		}
	}
	return &dictionary
}

var zPrint = fmt.Printf

func main() {
	dictionary := NewAnagramDictionary(dictionaryPath)
	_, _ = zPrint("%v", dictionary.Anagrams("triangle"))
}
