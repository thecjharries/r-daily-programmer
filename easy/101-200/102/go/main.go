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

var diceNotationPattern = regexp.MustCompile(`(?P<count>\d*)d(?P<sides>\d+)(?P<modifier>[+\-]\d+)?`)

type DiceSet struct {
	Count    int
	Sides    int
	Modifier int
}

func (d *DiceSet) Roll() (total int) {
	for index := 0; index < d.Count; index++ {
		total += rand.Intn(d.Sides) + 1
	}
	return total + d.Modifier
}

func NewDiceSet(notation string) *DiceSet {
	matches := diceNotationPattern.FindStringSubmatch(notation)
	fmt.Println(matches)
	count, _ := strconv.ParseInt(matches[1], 10, 0)
	if 1 > count {
		count = 1
	}
	sides, _ := strconv.ParseInt(matches[2], 10, 0)
	modifier, _ := strconv.ParseInt(matches[3], 10, 0)
	return &DiceSet{
		Count:    int(count),
		Sides:    int(sides),
		Modifier: int(modifier),
	}
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
