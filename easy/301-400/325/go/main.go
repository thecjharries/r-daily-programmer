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

// https://old.reddit.com/r/dailyprogrammer/comments/6qutez/20170801_challenge_325_easy_color_maze/dl0k0k1/
func findPathThroughMazeFollowingSequence(sequence []string, maze [][]string) (path [][]int) {
	row, column := len(maze)-1, len(maze[0])-1
	sequenceIndex := 0
	possible := make([][]int, 0)
	for index := range maze[row] {
		possible = append(possible, []int{row, index, sequenceIndex})
	}
	for 0 < row && sequence[sequenceIndex] != maze[row][column] {
		row, column, sequenceIndex = possible[len(possible)-1][0], possible[len(possible)-1][1], possible[len(possible)-1][2]
		possible = possible[:len(possible)-1]
		if maze[row][column] == sequence[sequenceIndex] {
			if 0 < len(path) {
				for -1 > row+column-path[len(path)-1][0]-path[len(path)-1][1] || 1 < row+column-path[len(path)-1][0]-path[len(path)-1][1] {
					path = path[:len(path)-1]
				}
			}
			path = append(path, []int{row, column})
			sequenceIndex = (sequenceIndex + 1) % len(sequence)
			if len(maze)-1 > row {
				possible = append(possible, []int{row + 1, column, sequenceIndex})
			}
			if 0 < column {
				possible = append(possible, []int{row, column - 1, sequenceIndex})
			}
			if len(maze[row])-1 > column {
				possible = append(possible, []int{row, column + 1, sequenceIndex})
			}
			if 0 < row {
				possible = append(possible, []int{row - 1, column, sequenceIndex})
			}
		}
	}
	return
}
