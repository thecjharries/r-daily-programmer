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

func findAndSortByPrefix(input []string, prefix string) (remaining, sorted []string) {
	for _, line := range input {
		if strings.HasPrefix(line, prefix) {
			sorted = append(sorted, line)
		} else {
			remaining = append(remaining, line)
		}
	}
	return
}

func sortCode(input []string) (sorted []string) {
	var currentSort []string
	remaining, headerSort := findAndSortByPrefix(input, "#")
	remaining, currentSort = findAndSortByPrefix(remaining, "    ")
	sorted = append(currentSort, sorted...)
	remaining, currentSort = findAndSortByPrefix(remaining, "  ")
	sorted = append(currentSort, sorted...)
	sorted = append(remaining, sorted...)
	sorted = append(headerSort, sorted...)
	return
}
