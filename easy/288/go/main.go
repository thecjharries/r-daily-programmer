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

var stopWords = map[string]struct{}{
	"i":     {},
	"a":     {},
	"about": {},
	"an":    {},
	"and":   {},
	"are":   {},
	"as":    {},
	"at":    {},
	"be":    {},
	"by":    {},
	"com":   {},
	"for":   {},
	"from":  {},
	"how":   {},
	"in":    {},
	"is":    {},
	"it":    {},
	"of":    {},
	"on":    {},
	"or":    {},
	"that":  {},
	"the":   {},
	"this":  {},
	"to":    {},
	"was":   {},
	"what":  {},
	"when":  {},
	"where": {},
	"who":   {},
	"will":  {},
	"with":  {},
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func findAlliterationGroups(input string) (alliteration [][]string) {
	exploded := strings.Split(input, " ")
	index := 0
	for index < len(exploded)-1 {
		sanitizedCurrentWord := strings.ToLower(exploded[index])
		_, isCurrentStopWord := stopWords[sanitizedCurrentWord]
		if isCurrentStopWord {
			index++
			continue
		}
		currentAlliterationGroup := []string{exploded[index]}
		fmt.Println(exploded[index])
		nextIndex := index + 1
		for nextIndex < len(exploded) {
			sanitizedNextWord := strings.ToLower(exploded[nextIndex])
			_, isNextStopWord := stopWords[sanitizedNextWord]
			if isNextStopWord {
				nextIndex++
				continue
			}
			if sanitizedCurrentWord[0] == sanitizedNextWord[0] {
				currentAlliterationGroup = append(currentAlliterationGroup, exploded[nextIndex])
				nextIndex++
			} else {
				break
			}
		}
		if 1 < len(currentAlliterationGroup) {
			alliteration = append(alliteration, currentAlliterationGroup)
		}
		index = nextIndex
	}
	return
}
