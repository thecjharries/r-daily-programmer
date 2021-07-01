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

var notAllowedPattern = regexp.MustCompile(`[^a-zA-Z0-9_ ]`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

// Note this doesn't ensure we start with a letter which is usually a requirement
func sanitizeVariableNameString(input string) string {
	return notAllowedPattern.ReplaceAllString(input, "")
}

func writeInCamelCase(words string) (output string) {
	exploded := strings.Split(sanitizeVariableNameString(words), " ")
	output = exploded[0]
	for index := 1; index < len(exploded); index++ {
		output += strings.ToUpper(exploded[index][0:1]) + strings.ToLower(exploded[index][1:])
	}
	return
}

func writeInSnakeCase(words string) string {
	return strings.ReplaceAll(sanitizeVariableNameString(words), " ", "_")
}

func writeInConstantCase(words string) string {
	return strings.ToUpper(strings.ReplaceAll(sanitizeVariableNameString(words), " ", "_"))
}