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
	length := int(math.Sqrt(float64(len(exploded))))
	currentRow := make([]int, 0)
	onCount := 0
	for index, character := range exploded {
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
		}
	}
	if 0 < onCount {
		currentRow = append(currentRow, onCount)
		onCount = 0
	}
	if 0 < len(currentRow) {
		rows = append(rows, currentRow)
	}
	currentColumn := make([]int, 0)
	onCount = 0
	for columnIndex := 0; columnIndex < length; columnIndex++ {
		for rowIndex := 0; rowIndex < length; rowIndex++ {
			character := exploded[rowIndex+length*columnIndex]
			if "*" == character {
				onCount++
			}
			if " " == character && 0 < onCount {
				currentColumn = append(currentColumn, onCount)
				onCount = 0
			}
		}
		if 0 < onCount {
			currentColumn = append(currentColumn, onCount)
			onCount = 0
		}
		if 0 < len(currentColumn) {
			columns = append(columns, currentColumn)
		}
		currentColumn = make([]int, 0)
	}
	return
}
