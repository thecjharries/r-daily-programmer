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

var infoPattern = regexp.MustCompile(`^(?P<name>[^:].{19})(?P<age>\d{2})(?P<birthday>\d{6})$`)
var extensionPattern = regexp.MustCompile(`::EXT::(?P<name>.{4})(?P<value>.{17})`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func findHighestSalary(input []string) (highest string) {
	highestSalary := -1
	highestSalaryOwner := ""
	currentName := ""
	for _, line := range input {
		matches := extensionPattern.FindStringSubmatch(line)
		if 0 == len(matches) {
			matches = infoPattern.FindStringSubmatch(line)
			currentName = strings.Trim(matches[1], " ")
		} else {
			extension := strings.Trim(matches[1], " ")
			if "SAL" == extension {
				salary, _ := strconv.Atoi(strings.Trim(matches[2], " "))
				if highestSalary < salary {
					highestSalary = salary
					highestSalaryOwner = currentName
				}
			}
		}
	}
	return fmt.Sprintf("%s, $%d", highestSalaryOwner, highestSalary)
}
