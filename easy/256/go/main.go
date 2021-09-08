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
	"strconv"
	"strings"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func oblique(input []int) (output string) {
	size := int(math.Sqrt(float64(len(input))))
	for diagonalRank := 0; diagonalRank < size; diagonalRank++ {
		currentRow := make([]string, 0)
		for columnIndex := 0; columnIndex <= diagonalRank; columnIndex++ {
			currentRow = append(currentRow, strconv.Itoa(input[columnIndex+size*(diagonalRank-columnIndex)]))
		}
		output += fmt.Sprintf("%s\n", strings.Join(currentRow, " "))
	}
	offset := 0
	for diagonalRank := size - 1; diagonalRank > 0; diagonalRank-- {
		currentRow := make([]string, 0)
		for rowIndex := size - diagonalRank; rowIndex < size; rowIndex++ {
			currentRow = append(currentRow, strconv.Itoa(input[size-rowIndex+offset+size*rowIndex]))
		}
		offset++
		output += fmt.Sprintf("%s\n", strings.Join(currentRow, " "))
	}
	return
}
