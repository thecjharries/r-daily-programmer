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
	"math"
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
	currentLine := words[0]
	for _, word := range words[1:] {
		if columnWidth < len(currentLine)+1+len(word) {
			lines = append(lines, currentLine+strings.Repeat(" ", columnWidth-len(currentLine)))
			currentLine = word
		} else {
			currentLine += " " + word
		}
	}
	lines = append(lines, currentLine+strings.Repeat(" ", columnWidth-len(currentLine)))
	return
}

func convertLinesToFinal(numberOfColumns, columnWidth, spacingWidth int, lines []string) string {
	linesPerColumn := int(math.Ceil(float64(len(lines)) / float64(numberOfColumns)))
	var output []string
	for lineIndex := 0; lineIndex < linesPerColumn; lineIndex++ {
		var currentLine []string
		for columnIndex := 0; columnIndex < numberOfColumns; columnIndex++ {
			if len(lines) <= lineIndex+columnIndex*linesPerColumn {
				currentLine = append(currentLine, strings.Repeat(" ", columnWidth))
			} else {
				currentLine = append(currentLine, lines[lineIndex+columnIndex*linesPerColumn])
			}
		}
		output = append(output, strings.Join(currentLine, strings.Repeat(" ", spacingWidth)))
	}
	return strings.Join(output, "\n")
}

func columnizeText(numberOfColumns, columnWidth, spacingWidth int, text string) string {
	return convertLinesToFinal(
		numberOfColumns,
		columnWidth,
		spacingWidth,
		convertTextToLines(columnWidth, text),
	)
}
