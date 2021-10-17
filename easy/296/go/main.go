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

var days = []string{
	"first",
	"second",
	"third",
	"fourth",
	"fifth",
	"sixth",
	"seventh",
	"eighth",
	"ninth",
	"tenth",
	"eleventh",
	"twelfth",
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func printTwelveDays(presents []string) (output string) {
	for dayIndex, day := range days {
		output += fmt.Sprintf("On the %s day of Christmas\nmy true love sent to me:\n", day)
		if 0 == dayIndex {
			output += fmt.Sprintf("1 %s\n\n", presents[0])
			continue
		}
		for presentIndex := dayIndex; presentIndex >= 0; presentIndex-- {
			if 0 == presentIndex {
				output += "and "
			}
			output += fmt.Sprintf("%d %s\n", presentIndex+1, presents[presentIndex])
		}
		output += "\n"
	}
	return strings.TrimRight(output, "\n")
}
