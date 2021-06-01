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

var solutionIndices = [][3]int{
	{0, 1, 2},
	{3, 4, 5},
	{6, 7, 8},
	{0, 3, 6},
	{1, 4, 7},
	{2, 5, 8},
	{0, 4, 8},
	{2, 4, 6},
}

type TicTacToe [9]rune

func (t *TicTacToe) FindWinningPositionInSolution(solution [3]int, piece rune) (solutionPosition int) {
	solutionPosition = -1
	foundCount := 0
	for _, position := range solution {
		if piece == (*t)[position] {
			foundCount++
		}
		if '-' == (*t)[position] {
			solutionPosition = position
		}
	}
	if 2 == foundCount {
		return solutionPosition
	}
	return -1
}

func (t *TicTacToe) FindWinningMove(piece rune) (solutionPosition int) {
	for _, possibleSolution := range solutionIndices {
		solutionPosition = t.FindWinningPositionInSolution(possibleSolution, piece)
		if -1 < solutionPosition {
			return
		}
	}
	return -1
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
