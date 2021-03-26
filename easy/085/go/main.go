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

import "fmt"

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func sum(numbers ...int) int {
	totalSum := 0
	for _, number := range numbers {
		totalSum += number
	}
	return totalSum
}

func computeRowSums(matrix []int, numRows, numCols int) (rowSums map[int][][]int) {
	rowSums = make(map[int][][]int)
	for rowIndex := 0; rowIndex < numRows*numCols; rowIndex += numCols {
		row := matrix[rowIndex : rowIndex+numCols]
		rowSum := sum(row...)
		_, rowSumExists := rowSums[rowSum]
		if rowSumExists {
			rowSums[rowSum] = append(rowSums[rowSum], row)
		} else {
			rowSums[rowSum] = [][]int{row}
		}
	}
	return
}

func computeColumnSums(matrix []int, numRows, numCols int) (colSums map[int][]int) {
	colSums = make(map[int][]int)
	for columnIndex := 0; columnIndex < numCols; columnIndex++ {
		colSum := 0
		for rowIndex := 0; rowIndex < numRows; rowIndex++ {
			colSum += matrix[columnIndex+rowIndex*numCols]
		}
		_, colSumExists := colSums[colSum]
		if colSumExists {
			colSums[colSum] = append(colSums[colSum], columnIndex)
		} else {
			colSums[colSum] = []int{columnIndex}
		}
	}
	return
}
