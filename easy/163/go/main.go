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
	"sort"
)

type DiceRollDistribution struct {
	Counts    []int
	Rolls     [][]int
	DiceSides int
}

func (d *DiceRollDistribution) SingleRoll(rollNumber int) {
	roll := rand.Intn(d.DiceSides)
	for index, count := range d.Counts {
		if rollNumber < count {
			d.Rolls[index][roll]++
		}
	}
}

func (d *DiceRollDistribution) RollAll() {
	for index := 0; index < d.Counts[len(d.Counts)-1]; index++ {
		d.SingleRoll(index)
	}
}

func NewDiceRollDistribution(sides int, counts []int) *DiceRollDistribution {
	var rolls [][]int
	for range counts {
		rolls = append(rolls, make([]int, sides))
	}
	sort.Ints(counts)
	return &DiceRollDistribution{
		Counts:    counts,
		Rolls:     rolls,
		DiceSides: sides,
	}
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
