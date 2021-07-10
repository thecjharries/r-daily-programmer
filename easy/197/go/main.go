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
	"strings"
)

var notDashPattern = regexp.MustCompile(`[^-]`)
var notNumberPattern = regexp.MustCompile(`[^\d]`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func isValidIsbn(input string) bool {
	dashStripped := notDashPattern.ReplaceAllString(input, "")
	if 10 != len(dashStripped) {
		return false
	}
	converted := notNumberPattern.ReplaceAllString(input, "")
	checksum := 0
	if 9 == len(converted) {
		checksum = 10
	} else if 10 != len(converted) {
		return false
	}
	for index, character := range strings.Split(converted, "") {
		number, _ := strconv.Atoi(character)
		checksum += (10 - index) * number
	}
	return 0 == checksum%11
}
