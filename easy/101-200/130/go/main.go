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
	"math/rand"
	"regexp"
	"strconv"
)

var dieNotationPattern = regexp.MustCompile(`(?P<count>\d+)d(?P<sides>\d+)`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func parseNotation(notation string) (count, sides int) {
	matches := dieNotationPattern.FindStringSubmatch(notation)
	count, _ = strconv.Atoi(matches[1])
	sides, _ = strconv.Atoi(matches[2])
	return
}

func rollTheDice(count, sides int) (rolls []int) {
	for index := 0; index < count; index++ {
		rolls = append(rolls, rand.Intn(sides)+1)
	}
	return
}

func parseAndRoll(notation string) []int {
	count, sides := parseNotation(notation)
	return rollTheDice(count, sides)
}
