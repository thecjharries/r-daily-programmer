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

var operations = map[string]func(int, int) int{
	"+": func(a, b int) int { return a + b },
	"-": func(a, b int) int { return a - b },
	"*": func(a, b int) int { return a * b },
	"/": func(a, b int) int { return a / b },
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func getHorizontalRule(maximumNumber int) string {
	return strings.Repeat("-", 3*(maximumNumber+2))
}

func getHeaderRow(operation string, maximumNumber int) (output string) {
	output = fmt.Sprintf(" %s  | ", operation)
	for index := 0; index <= maximumNumber; index++ {
		output += fmt.Sprintf(" %d ", index)
	}
	return
}

func getTableRow(currentNumber, maximumNumber int, operation func(int, int) int) (output string) {
	output = fmt.Sprintf(" %d  | ", currentNumber)
	for index := 0; index <= maximumNumber; index++ {
		output += fmt.Sprintf(" %d ", operation(currentNumber, index))
	}
	return
}

func buildTable(operation string, maximumNumber int) string {
	operationFunc, _ := operations[operation]
	rows := []string{getHeaderRow(operation, maximumNumber)}
	rows = append(rows, getHorizontalRule(maximumNumber))
	for index := 0; index <= maximumNumber; index++ {
		rows = append(rows, getTableRow(index, maximumNumber, operationFunc))
	}
	return strings.Join(rows, "\n")
}
