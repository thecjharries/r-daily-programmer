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

func main() {
	fmt.Println("hello world")
}

func createExceptionsMap(exceptions []string) (exceptionsMap map[string]bool) {
	exceptionsMap = make(map[string]bool, len(exceptions))
	for _, exception := range exceptions {
		exceptionsMap[exception] = true
	}
	return
}

func titleCaseWithExceptions(input string, exceptions []string) string {
	exceptionMap := createExceptionsMap(exceptions)
	exploded := strings.Split(input, " ")
	exploded[0] = strings.Title(strings.ToLower(exploded[0]))
	for index := 1; index < len(exploded); index++ {
		exploded[index] = strings.ToLower(exploded[index])
		_, isException := exceptionMap[exploded[index]]
		if !isException {
			exploded[index] = strings.Title(exploded[index])
		}
	}
	return strings.Join(exploded, " ")
}
