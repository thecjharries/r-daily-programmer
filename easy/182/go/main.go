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
	"regexp"
	"strings"
)

var spacePattern = regexp.MustCompile(`\s+`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func convertTextToLines(columnWidth int, text string) (lines []string) {
	words := spacePattern.Split(text, -1)
	var currentLine string
	for _, word := range words {
		if columnWidth < len(currentLine)+1+len(word) {
			lines = append(lines, currentLine+strings.Repeat(" ", columnWidth-len(currentLine)))
			currentLine = word
		} else {
			currentLine += " " + word
		}
	}
	return
}
