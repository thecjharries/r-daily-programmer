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

type Point struct {
	Red   int
	Green int
	Blue  int
}

func (p *Point) String() string {
	return fmt.Sprintf("%3d %3d %3d", p.Red, p.Green, p.Blue)
}

type Grid [][]Point

func NewGrid(columns, rows int) *Grid {
	grid := new(Grid)
	*grid = make(Grid, rows)
	for yIndex := 0; yIndex < rows; yIndex++ {
		(*grid)[yIndex] = make([]Point, columns)
		for xIndex := 0; xIndex < columns; xIndex++ {
			(*grid)[yIndex][xIndex] = Point{}
		}
	}
	return grid
}

func (g *Grid) String() string {
	var output []string
	for _, row := range *g {
		var currentRow []string
		for _, entry := range row {
			currentRow = append(currentRow, entry.String())
		}
		output = append(output, strings.Join(currentRow, " "))
	}
	return strings.Join(output, "\n")
}

func (g *Grid) Line(color Point, startX, startY, endX, endY int) {
	for xIndex := startX; xIndex <= endX; xIndex++ {
		(*g)[xIndex][(endY-startY)*xIndex/(endX-startX)+startY] = color
	}
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
