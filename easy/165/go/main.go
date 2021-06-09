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
