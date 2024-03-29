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
	"strconv"
)

type Raster struct {
	Image    [][]int
	CurrentX int
	CurrentY int
}

func (r *Raster) Stamp() {
	r.Image[r.CurrentX][r.CurrentY] = 1
}

func (r *Raster) Move(x, y int) {
	r.CurrentX += x
	r.CurrentY += y
}

func (r *Raster) ParseAction(action rune) {
	if 'N' == action {
		r.Move(0, -1)
	} else if 'S' == action {
		r.Move(0, 1)
	} else if 'E' == action {
		r.Move(1, 0)
	} else if 'W' == action {
		r.Move(-1, 0)
	} else if 'P' == action {
		r.Stamp()
	}
}

func (r *Raster) String() string {
	output := ""
	for _, row := range r.Image {
		for _, pixel := range row {
			output += strconv.Itoa(pixel)
		}
		output += "\n"
	}
	return output
}

func NewRaster(width, height int, actions string) (raster *Raster) {
	slices := make([][]int, height)
	for index := 0; index < height; index++ {
		slices[index] = make([]int, width)
	}
	raster = &Raster{
		Image:    slices,
		CurrentX: 0,
		CurrentY: 0,
	}
	for _, action := range actions {
		raster.ParseAction(action)
	}
	return
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
