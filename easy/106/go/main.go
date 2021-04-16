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
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

type WordFrequency struct {
	Word  string
	Count int
}

var novelPath = filepath.Join("..", "enable1.txt")

var promptPattern = regexp.MustCompile(`[".,:;!?()[\]{}]|[^".,:;!?()[\]{}\s]+`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func loadFileIntoString(filename string) string {
	byteContents, _ := ioutil.ReadFile(filename)
	return strings.Trim(string(byteContents), "\n")
}

func countPatternsInString(input string) map[string]int {
	results := promptPattern.FindAllString(input, -1)
	wordCount := make(map[string]int)
	for _, result := range results {
		_, exists := wordCount[result]
		if exists {
			wordCount[result] += 1
		} else {
			wordCount[result] = 1
		}
	}
	return wordCount
}

func determineTopTenWords(wordCount map[string]int) []*WordFrequency {
	countWord := make(map[int][]string)
	for word, count := range wordCount {
		_, exists := countWord[count]
		if exists {
			countWord[count] = append(countWord[count], word)
		} else {
			countWord[count] = []string{word}
		}
	}
	var counts []int
	for count := range countWord {
		counts = append(counts, count)
	}
	sort.Sort(sort.Reverse(sort.IntSlice(counts)))
	var frequencies []*WordFrequency
	for _, count := range counts {
		wordsAtCount, _ := countWord[count]
		for _, word := range wordsAtCount {
			frequencies = append(
				frequencies,
				&WordFrequency{
					Word:  word,
					Count: count,
				},
			)
		}
		if 10 <= len(frequencies) {
			break
		}
	}
	return frequencies[0:10]
}
