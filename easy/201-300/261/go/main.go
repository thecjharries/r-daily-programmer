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

func isMagicSquare(input []int) bool {
	firstDiagonalSum := 0
	secondDiagonalSum := 0
	for index := 0; index < 3; index++ {
		firstDiagonalSum += input[2*(index+1)]
		secondDiagonalSum += input[index*4]
		rowSum := 0
		columnSum := 0
		for subIndex := 0; subIndex < 3; subIndex++ {
			rowSum += input[subIndex+index*3]
			columnSum += input[index+subIndex*3]
		}
		if 15 != rowSum || 15 != columnSum {
			return false
		}
	}
	return 15 == firstDiagonalSum && 15 == secondDiagonalSum
}
