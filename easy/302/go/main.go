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

var elements = map[string]string{
	"ge": "germanium",
	"ni": "nickel",
	"u":  "uranium",
	"s":  "sulfur",
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func spellWithElements(input string) (output string) {
	workingInput := input
	foundElements := make([]string, 0)
	for 0 < len(workingInput) {
		twos, twosExists := "", false
		if 1 < len(workingInput) {
			twos, twosExists = elements[workingInput[:2]]
		}
		if twosExists {
			output += strings.Title(workingInput[:2])
			foundElements = append(foundElements, twos)
			workingInput = workingInput[2:]
		} else {
			ones, onesExists := elements[workingInput[:1]]
			if onesExists {
				output += strings.ToUpper(workingInput[:1])
				foundElements = append(foundElements, ones)
				workingInput = workingInput[1:]
			} else {
				return ""
			}
		}
	}
	return fmt.Sprintf("%s (%s)", output, strings.Join(foundElements, " "))
}
