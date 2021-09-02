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
	"math"
	"strings"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func parseNonogram(nonogram string) (rows, columns [][]int) {
	exploded := strings.Split(strings.ReplaceAll(nonogram, "\n", ""), "")
	fmt.Println(exploded)
	length := int(math.Sqrt(float64(len(exploded))))
	fmt.Println(length)
	currentRow := make([]int, 0)
	onCount := 0
	for index, character := range exploded {
		fmt.Printf("'%s'", character)
		if "*" == character {
			onCount++
		}
		if " " == character && 0 < onCount {
			currentRow = append(currentRow, onCount)
			onCount = 0
		}
		if 0 == (index+1)%length {
			if 0 < onCount {
				currentRow = append(currentRow, onCount)
				onCount = 0
			}
			if 0 < len(currentRow) {
				rows = append(rows, currentRow)
			}
			currentRow = make([]int, 0)
			fmt.Printf("\n")
		}
	}
	if 0 < onCount {
		currentRow = append(currentRow, onCount)
	}
	if 0 < len(currentRow) {
		rows = append(rows, currentRow)
	}
	return
}
