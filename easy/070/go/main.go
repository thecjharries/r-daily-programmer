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
	"regexp"
	"sort"
)

type WordFrequency struct {
	Word string
	Frequency int
}

type WordFrequencies []WordFrequency

func (w WordFrequencies) Len() int {
	return len(w)
}

func (w WordFrequencies) Swap(i, j int) {
	w[i], w[j] = w[j], w[i]
}

func (w WordFrequencies) Less(i, j int) bool {
	return w[i].Frequency > w[j].Frequency
}

var wordPattern = regexp.MustCompile(`\w+`)

func main() {
	filename := "lorem_ipsum.txt"
	contents := readFileIntoString(filename)
	frequencyMap := generateFrequencyMapFromString(contents)
	frequencies := convertFrequencyMapToWordFrequencies(frequencyMap)
	fmt.Println(selectTopWordsFromWordFrequencies(frequencies, 5))
}

func readFileIntoString(filename string) string {
	buffer, _ := ioutil.ReadFile(filename)
	return string(buffer)
}

func generateFrequencyMapFromString(input string) (frequencyMap map[string]int) {
	frequencyMap = make(map[string]int)
	words := wordPattern.FindAllString(input, -1)
	for _, word := range words {
		_, ok := frequencyMap[word]
		if ok {
			frequencyMap[word] += 1
		} else {
			frequencyMap[word] = 1
		}
	}
	return
}

func convertFrequencyMapToWordFrequencies(frequencyMap map[string]int) (frequencies WordFrequencies) {
	for key, value := range frequencyMap {
		frequencies = append(frequencies, WordFrequency{Word: key, Frequency: value})
	}
	return
}

func selectTopWordsFromWordFrequencies(frequencies WordFrequencies, maxCount int) (results WordFrequencies) {
	sort.Sort(frequencies)
	if len(frequencies) < maxCount {
		return frequencies
	}
	return frequencies[:maxCount]
}
