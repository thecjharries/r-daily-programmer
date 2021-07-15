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
	"strconv"
)

var notBinaryRegex = regexp.MustCompile(`[^10]`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func parseBinaryToString(input string) string {
	var parsedRunes []rune
	sanitizedInput := notBinaryRegex.ReplaceAllString(input, "")
	for endIndex := 8; endIndex <= len(sanitizedInput); endIndex += 8 {
		parsedInt, _ := strconv.ParseInt(sanitizedInput[endIndex-8:endIndex], 2, 0)
		parsedRunes = append(parsedRunes, rune(parsedInt))
	}
	return string(parsedRunes)
}
