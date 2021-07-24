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

type Grid [][]rune

func (g *Grid) String() string {
	combined := make([]string, len(*g))
	for index, row := range *g {
		combined[index] = string(row)
	}
	return strings.Join(combined, "\n")
}

func (g *Grid) Fill(startX, startY int, fill rune) {
	characterToReplace := (*g)[startX][startY]
	coordinates := []int{startX, startY}
	for 0 < len(coordinates) {
		currentX, currentY := coordinates[0], coordinates[1]
		coordinates = coordinates[2:]
		if characterToReplace == (*g)[currentX][currentY] {
			(*g)[currentX][currentY] = fill
			coordinates = append(
				coordinates,
				(currentX+len(*g)-1)%len(*g), currentY,
				(currentX+1)%len(*g), currentY,
				currentX, (currentY+len((*g)[currentX])-1)%len((*g)[currentX]),
				currentX, (currentY+1)%len((*g)[currentX]),
			)
		}
	}
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
