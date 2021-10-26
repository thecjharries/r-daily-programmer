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

var stateChanges = map[rune]rune{
	'S': 'F',
	' ': 'S',
}

type Map []string

func (m *Map) AddSmoke(x, y int) {
	if len(*m) <= y || len((*m)[y]) <= x {
		return
	}
	change, exists := stateChanges[rune((*m)[y][x])]
	if exists {
		(*m)[y] = (*m)[y][:x] + string(change) + (*m)[y][x+1:]
		if 'S' == (*m)[y][x] {
			for _, yChange := range []int{-1, 1} {
				if len(*m) <= y+yChange {
					continue
				}
				if '|' == (*m)[y+yChange][x] || '/' == (*m)[y+yChange][x] || '_' == (*m)[y+yChange][x] || '=' == (*m)[y+yChange][x] {
					yChange *= 2
				}
				if len(*m) <= y+yChange {
					continue
				}
				if 'F' == (*m)[y+yChange][x] {
					(*m)[y] = (*m)[y][:x] + "F" + (*m)[y][x+1:]
					break
				}
			}
			for _, xChange := range []int{-1, 1} {
				if len((*m)[y]) <= x+xChange {
					continue
				}
				if '|' == (*m)[y][x+xChange] || '/' == (*m)[y][x+xChange] || '_' == (*m)[y][x+xChange] || '=' == (*m)[y][x+xChange] {
					xChange *= 2
				}
				if len((*m)[y]) <= x+xChange {
					continue
				}
				if 'F' == (*m)[y][x+xChange] {
					(*m)[y] = (*m)[y][:x] + "F" + (*m)[y][x+1:]
					break
				}
			}
		}
		for y, _ = range *m {
			for x, _ = range (*m)[y] {
				if 'F' == (*m)[y][x] {
					for _, yChange := range []int{-1, 1} {
						if len(*m) <= y+yChange {
							continue
						}
						if '|' == (*m)[y+yChange][x] || '/' == (*m)[y+yChange][x] || '_' == (*m)[y+yChange][x] || '=' == (*m)[y+yChange][x] {
							yChange *= 2
						}
						if len(*m) <= y+yChange {
							continue
						}
						if 'S' == (*m)[y+yChange][x] {
							(*m)[y+yChange] = (*m)[y+yChange][:x] + "F" + (*m)[y+yChange][x+1:]
						}
					}
					for _, xChange := range []int{-1, 1} {
						if len((*m)[y]) <= x+xChange {
							continue
						}
						if '|' == (*m)[y][x+xChange] || '/' == (*m)[y][x+xChange] || '_' == (*m)[y][x+xChange] || '=' == (*m)[y][x+xChange] {
							xChange *= 2
						}
						if len((*m)[y]) <= x+xChange {
							continue
						}
						if 'S' == (*m)[y][x+xChange] {
							(*m)[y] = (*m)[y][:x+xChange] + "F" + (*m)[y][x+xChange+1:]
						}
					}
				}
			}
		}
	}
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
