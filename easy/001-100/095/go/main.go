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

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func reverseSliceOfStrings(collection []string) (reversed []string) {
	reversed = make([]string, len(collection))
	copy(reversed, collection)
	for index := 0; index < len(reversed)/2; index++ {
		reversed[index], reversed[len(reversed)-index-1] = reversed[len(reversed)-index-1], reversed[index]
	}
	return
}

func reverseText(input string) (output string) {
	lines := strings.Split(input, "\n")
	for index, line := range lines {
		exploded := strings.Split(line, " ")
		lines[index] = strings.Join(reverseSliceOfStrings(exploded), " ")
	}
	return strings.Join(reverseSliceOfStrings(lines), "\n")
}
