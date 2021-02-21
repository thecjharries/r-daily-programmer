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
	"sort"
	"strings"
)

const romanAlphabet string = " abcdefghijklmnopqrstuvwxyz"

type WordValue struct {
	Word string
	Value int
}

func main() {
	fmt.Println("hello world")
}

func getLetterValue(letter string) int {
	return strings.Index(romanAlphabet, letter)
}

func getWordValue(word string) int {
	value := 0
	for _, letter := range strings.ToLower(word) {
		value += getLetterValue(string(letter))
	}
	return value
}

func createWordValueSlice(words []string) (wordValues []WordValue) {
	for _, word := range words {
		wordValues = append(wordValues, WordValue{word, getWordValue(word)})
	}
	return
}

func sortWordValueSlice(wordValues []WordValue) []WordValue {
	sort.Slice(wordValues, func(first, second int) bool {
		return wordValues[first].Value < wordValues[second].Value
	})
	return wordValues
}

func orderListOfWordsByValue(words []string) (sortedWords []string) {
	wordValues := createWordValueSlice(words)
	wordValues = sortWordValueSlice(wordValues)
	for _, wordValue := range wordValues {
		sortedWords = append(sortedWords, wordValue.Word)
	}
	return
}
