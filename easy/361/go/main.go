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

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func parseScore(score string) (scores map[rune]int) {
	scores = make(map[rune]int)
	for _, element := range score {
		elementIndex := []rune(strings.ToLower(string(element)))[0]
		scoreChange := 1
		if string(element) == strings.ToUpper(string(element)) {
			scoreChange = -1
		}
		_, exists := scores[elementIndex]
		if exists {
			scores[elementIndex] += scoreChange
		} else {
			scores[elementIndex] = scoreChange
		}
	}
	return
}
