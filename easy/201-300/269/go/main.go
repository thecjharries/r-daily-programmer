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

var notCapitalLetterPattern = regexp.MustCompile(`^[^A-Z]+`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func indentProperly(input, delimiter string) (output string) {
	exploded := strings.Split(input, "\n")
	var final []string
	currentIndentation := 0
	for _, line := range exploded {
		correctedLine := notCapitalLetterPattern.ReplaceAllString(line, "")
		if strings.HasPrefix(correctedLine, "ENDIF") || strings.HasPrefix(correctedLine, "NEXT") {
			currentIndentation--
		}
		final = append(final, fmt.Sprintf("%s%s", strings.Repeat(delimiter, currentIndentation), correctedLine))
		fmt.Println(correctedLine)
		if strings.HasPrefix(correctedLine, "IF ") || strings.HasPrefix(correctedLine, "FOR ") {
			currentIndentation++
		}
	}
	return strings.Join(final, "\n")
}
