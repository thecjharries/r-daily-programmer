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

type GameOfLife struct {
	Map    string
	Width  int
	Height int
}

func (g *GameOfLife) String() (output string) {
	for index := 0; index < len(g.Map); index += g.Width {
		output += g.Map[index:index+g.Width] + "\n"
	}
	return
}

func (g *GameOfLife) GetCellNeighborOnCount(x, y int) (count int) {
	for _, verticalIndex := range []int{(y - 1 + g.Height) % g.Height, y, (y + 1 + g.Height) % g.Height} {
		for _, horizontalIndex := range []int{(x - 1 + g.Width) % g.Width, x, (x + 1 + g.Width) % g.Width} {
			if (x != horizontalIndex || y != verticalIndex) && '#' == g.Map[horizontalIndex+verticalIndex*g.Width] {
				count++
			}
		}
	}
	return
}

func (g *GameOfLife) Iterate() {
	newMap := ""
	for y := 0; y < g.Height; y++ {
		for x := 0; x < g.Width; x++ {
			neighborsOn := g.GetCellNeighborOnCount(x, y)
			if 3 == neighborsOn && "." == string(g.Map[x+y*g.Width]) {
				newMap += "#"
			} else if (2 > neighborsOn || 3 < neighborsOn) && '#' == g.Map[x+y*g.Width] {
				newMap += "."
			} else {
				newMap += string(g.Map[x+y*g.Width])
			}
		}
	}
	g.Map = newMap
}

func NewGameOfLife(width, height int, gameMap string) *GameOfLife {
	return &GameOfLife{
		Map:    strings.ReplaceAll(gameMap, "\n", ""),
		Width:  width,
		Height: height,
	}
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
