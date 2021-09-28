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
	"sort"
	"strings"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func printWordRectangle(word string, width, height int) (output string) {
	end := strings.Split(word[1:len(word)-1], "")
	sort.Sort(sort.Reverse(sort.StringSlice(end)))
	searchSpace := word + strings.Join(end, "")
	for y := 0; y < (len(word)-1)*height+1; y++ {
		for x := 0; x < (len(word)-1)*width+1; x++ {
			if 0 == x%(len(word)-1) || 0 == y%(len(word)-1) {
				output += searchSpace[(x+y)%len(searchSpace) : (x+y)%len(searchSpace)+1]
			} else {
				output += " "
			}
		}
		output += "\n"
	}
	return
}
