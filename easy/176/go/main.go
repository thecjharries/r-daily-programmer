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
	"regexp"
	"strconv"
	"strings"
)

var letterDigit = []rune(" abcdefghijklmnopqrstuvwxyz")

var cellSyntaxPattern = regexp.MustCompile(`(?i)([a-z]+)(\d+)`)

type Cell struct {
	Row    int
	Column int
}

func NewCellFromString(cellDefn string) (cell Cell) {
	matched := cellSyntaxPattern.FindStringSubmatch(cellDefn)
	row, _ := strconv.Atoi(matched[2])
	return Cell{
		Row:    row,
		Column: getCellColumn(matched[1]),
	}
}

type CellSelection []Cell

func NewCellSelectionFromColonRange(cellRange string) (selection CellSelection) {
	exploded := strings.Split(cellRange, ":")
	first := NewCellFromString(exploded[0])
	last := NewCellFromString(exploded[1])
	for rowIndex := first.Row; rowIndex <= last.Row; rowIndex++ {
		for columnIndex := first.Column; columnIndex <= last.Column; columnIndex++ {
			selection = append(selection, Cell{rowIndex, columnIndex})
		}
	}
	return
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
