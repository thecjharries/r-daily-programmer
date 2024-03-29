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

var dicePattern = regexp.MustCompile(`(\d+)d(\d+)`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func roll(dice string) int {
	matches := dicePattern.FindStringSubmatch(dice)
	if len(matches) != 3 {
		return 0
	}
	count, _ := strconv.Atoi(matches[1])
	sides, _ := strconv.Atoi(matches[2])
	sum := 0
	for i := 0; i < count; i++ {
		sum += 1 + rand.Intn(sides)
	}
	return sum
}
