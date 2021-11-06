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

func buildNumberSpiral(number int) (spiral [][]int) {
	spiral = make([][]int, number)
	for i := range spiral {
		spiral[i] = make([]int, number)
	}
	dx, dy := 1, 0
	x, y := 0, 0
	for index := 1; index <= number*number; index++ {
		spiral[y][x] = index
		if spiral[(y-dy+number)%number][(x+dx+number)%number] != 0 {
			dx, dy = dy, -dx
		}
		x += dx
		y -= dy
	}
	return
}
