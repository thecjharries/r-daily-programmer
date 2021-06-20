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

var letterDigit = []rune(" abcdefghijklmnopqrstuvwxyz")

type Cell struct {
	Row    int
	Column int
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func getLetterDigit(letter rune) int {
	for index, element := range letterDigit {
		if letter == element {
			return index
		}
	}
	return 0
}

func getCellColumn(column string) (cellColumn int) {
	sanitizedColumn := strings.ToLower(column)
	power := 0.0
	for index := len(sanitizedColumn) - 1; index >= 0; index-- {
		cellColumn += getLetterDigit(rune(sanitizedColumn[index])) * int(math.Pow(26, power))
		power += 1
	}
	return cellColumn
}
