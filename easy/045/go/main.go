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
	"strings"
)

const (
	squareSize int = 3
	whiteCharacter string = " "
	blackCharacter string = "#"
)

func main() {
	board := createBoard(2, 5)
	for _, line := range board {
		fmt.Println(line)
	}
}

func createRow(squareCount int, whiteFirst bool) (row []string) {
	for lineIndex := 0; lineIndex < squareSize; lineIndex++ {
		currentlyWhite := whiteFirst
		currentLine := ""
		for squareIndex := 0; squareIndex < squareCount; squareIndex++ {
			if currentlyWhite {
				currentLine += strings.Repeat(whiteCharacter, squareSize)
			} else {
				currentLine += strings.Repeat(blackCharacter, squareSize)
			}
			currentlyWhite = !currentlyWhite
		}
		row = append(row, currentLine)
	}
	return
}

func createBoard(rows, columns int) (board []string) {
	whiteFirst := true
	for rowIndex := 0; rowIndex < rows; rowIndex++ {
		board = append(board, createRow(columns, whiteFirst)...)
		whiteFirst = !whiteFirst
	}
	return
}
