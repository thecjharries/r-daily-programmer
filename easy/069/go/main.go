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
	"strings"
)

func main() {
	fmt.Println("hello world")
}

func formatDividerLine(lengthOfLongestWord int) string {
	return fmt.Sprintf("+%s+", strings.Repeat("-", lengthOfLongestWord + 2))
}

func formatLine(contents string, lengthOfLongestWord int, isCentered bool) string {
	var lineContents string
	if isCentered {
		lineContents = fmt.Sprintf(
			"%s%s",
			strings.Repeat(" ", int(math.Floor(float64(lengthOfLongestWord - len(contents)) / 2))),
			contents,
		)
	} else {
		lineContents = contents
	}
	return fmt.Sprintf(
		"| %s%s |",
		lineContents,
		strings.Repeat(" ", lengthOfLongestWord - len(lineContents)),
	)
}

func findLengthOfLongestWord(words []string) int {
	maxLength := -1
	for _, word := range words {
		if maxLength < len(word) {
			maxLength = len(word)
		}
	}
	return maxLength
}

//func formatAsciiTableFromTitleAndList(title string, list []string) string {
//	contents := make([]string, len(list) + 4)
//	contents = append(
//		contents,
//		formatDividerLine()
//		)
//}
